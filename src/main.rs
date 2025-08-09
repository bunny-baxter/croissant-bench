mod game_model;

use std::io;
use std::io::Write;
use std::rc::Rc;
use std::sync::LazyLock;

use regex::Regex;

use game_model::{CroissantGame, CroissantGameConfig, format_money};


static ACTION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)(\s*\d*)").unwrap());


fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_bytes) => {},
        Err(_error) => input.push_str("error"),
    }
    input
}

fn prompt_user() -> String {
    print!("> ");
    io::stdout().flush().expect("failed flush");
    read_line().trim().to_string()
}

fn process_action(game: &mut CroissantGame, action: &str) -> game_model::ActionResult<()> {
    let Some(captures) = ACTION_REGEX.captures(action) else {
        return Err(game_model::InvalidActionError { cause: game_model::InvalidActionErrorCause::InvalidAction });
    };
    let maybe_quantity = captures[2].trim().parse::<u32>();
    // validate quantity and action first
    match &captures[1] {
        "1" | "3" | "4" | "5" => if maybe_quantity.is_ok() {
            return Err(game_model::InvalidActionError { cause: game_model::InvalidActionErrorCause::ExtraneousQuantity });
        },
        "2" | "6" => if maybe_quantity.is_err() {
            return Err(game_model::InvalidActionError { cause: game_model::InvalidActionErrorCause::InvalidQuantity });
        },
        _ => {
            return Err(game_model::InvalidActionError { cause: game_model::InvalidActionErrorCause::InvalidAction });
        },
    };
    // at this point everything should be validated, so execute actions
    match &captures[1] {
        "1" => game.execute_cook(),
        "2" => game.execute_buy_cheese(maybe_quantity.unwrap()),
        "3" => todo!(),
        "4" => todo!(),
        "5" => todo!(),
        "6" => game.execute_buy_croissants(maybe_quantity.unwrap()),
        _ => unreachable!(),
    }
}

fn main() {
    let game_config_owned: CroissantGameConfig = toml::from_str(include_str!("game_config.toml")).unwrap();
    let game_config = Rc::new(game_config_owned);

    let mut game = CroissantGame::new(game_config.clone());

    while !game.is_game_over() {
        let (mature_cheese, non_mature_cheeses) = game.count_cheeses();
        println!("It is turn {}/{} and you have:", game.turn, game_config.turns);
        println!("- {} dollars", format_money(game.money));
        println!("- {} recipes", game.recipes);
        println!("- {} mature and {} non-mature cheese", mature_cheese, non_mature_cheeses);
        println!("- {} cookbooks", game.cookbooks);
        println!("- {} croissants", game.croissants);
        println!("");

        println!("The market price of croissants is {}.", format_money(game.croissant_price));

        println!("1. Cook");
        println!("2. Buy fresh cheese [quantity]");
        println!("3. Sell all mature cheese");
        println!("4. Buy 1 recipe");
        println!("5. Buy 1 cookbook");
        println!("6. Buy croissants [quantity]");
        println!("");

        let action = prompt_user().to_lowercase();
        let result = process_action(&mut game, &action);
        match result {
            Ok(()) => println!("\n================================\n"),
            Err(e) => println!("\nERROR: {}\n", e.describe()),
        };
    }

    println!("Game is over now!");
}
