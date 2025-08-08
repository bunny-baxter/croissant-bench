use std::fmt;
use std::rc::Rc;

use serde::Deserialize;


#[derive(Debug, Clone)]
pub enum InvalidActionErrorCause {
    InvalidInput,
    GameOver,
    NotEnoughMoney,
}

#[derive(Debug, Clone)]
pub struct InvalidActionError {
    pub cause: InvalidActionErrorCause,
}

impl InvalidActionError {
    pub fn describe(&self) -> &'static str {
        match self.cause {
            InvalidActionErrorCause::InvalidInput => "Invalid input.",
            InvalidActionErrorCause::GameOver => "Game is over.",
            InvalidActionErrorCause::NotEnoughMoney => "Not enough money.",
        }
    }
}

impl fmt::Display for InvalidActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

type ActionResult<T> = std::result::Result<T, InvalidActionError>;


pub fn format_money(raw_money: i32) -> String {
    format!("{}.{:02}", raw_money / 100, raw_money % 100)
}


#[derive(Deserialize)]
pub struct CroissantGameConfig {
    pub turns: i32,
    pub starting_money: i32,
    pub cook_payoff: i32,
    pub cheese_cost: i32,
    pub cheese_mature_turns: i32,
    pub cheese_payoff: i32,
    pub recipe_cost: i32,
    pub recipe_dividend: i32,
    pub cookbook_cost: i32,
    pub cookbook_dividend: i32,
    pub croissant_starting_price: i32,
    pub croissant_quantity_maximum: i32,
    pub croissant_price_fall: i32,
    pub croissant_price_rise: i32,
    pub croissant_minimum_price: i32,
}


pub struct CroissantGame {
    config: Rc<CroissantGameConfig>,

    pub turn: i32,
    pub money: i32,
    pub cheeses: Vec<i32>,
    pub recipes: i32,
    pub cookbooks: i32,
    pub croissant_price: i32,
    pub croissants: i32,
}

impl CroissantGame {
    pub fn new(config: Rc<CroissantGameConfig>) -> Self {
        let starting_money = config.starting_money;
        let croissant_starting_price = config.croissant_starting_price;
        CroissantGame {
            config: config,
            turn: 1,
            money: starting_money,
            cheeses: vec![],
            recipes: 0,
            cookbooks: 0,
            croissant_price: croissant_starting_price,
            croissants: 0,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.turn > self.config.turns
    }

    // Returns (mature cheeses, non-mature cheeses)
    pub fn count_cheeses(&self) -> (i32, i32) {
        let mut mature = 0;
        let mut non_mature = 0;
        for &cheese_age in self.cheeses.iter() {
            if cheese_age >= self.config.cheese_mature_turns {
                mature += 1;
            } else {
                non_mature += 1;
            }
        }
        (mature, non_mature)
    }

    fn end_turn(&mut self) {
        self.turn += 1;
    }

    pub fn execute_cook(&mut self) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        self.money += self.config.cook_payoff;
        self.end_turn();
        Ok(())
    }
}
