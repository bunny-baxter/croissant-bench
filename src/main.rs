mod game_model;

use std::io;
use std::io::Write;
use std::rc::Rc;

use game_model::{CroissantGame, CroissantGameConfig, format_money};

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

        println!("The market price of croissants is {}.", game.croissant_price);

        println!("1. Cook");
        println!("2. Buy fresh cheese");
        println!("3. Sell mature cheese");
        println!("4. Buy recipe");
        println!("5. Buy cookbook");
        println!("6. Buy croissants");
        println!("");

        let action = prompt_user().to_lowercase();
        let result = match action.as_str() {
            "1" | "cook" => game.execute_cook(),
            _ => Err(game_model::InvalidActionError { cause: game_model::InvalidActionErrorCause::InvalidInput }),
        };
        match result {
            Ok(()) => println!("\n================================\n"),
            Err(e) => println!("{}\n", e.describe()),
        };
    }

    println!("Game is over now!");
}
