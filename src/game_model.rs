use std::fmt;
use std::rc::Rc;

use serde::Deserialize;


#[derive(Debug, Clone)]
pub enum InvalidActionErrorCause {
    InvalidAction,
    InvalidQuantity,
    ExtraneousQuantity,
    GameOver,
    NotEnoughMoney(i32),
    CheeseMaxQuantityExceeded(u32),
}

#[derive(Debug, Clone)]
pub struct InvalidActionError {
    pub cause: InvalidActionErrorCause,
}

impl InvalidActionError {
    pub fn describe(&self) -> String {
        match self.cause {
            InvalidActionErrorCause::InvalidAction => "Invalid action.".to_string(),
            InvalidActionErrorCause::InvalidQuantity => "Action requires a quantity.".to_string(),
            InvalidActionErrorCause::ExtraneousQuantity => "Action should not have a quantity.".to_string(),
            InvalidActionErrorCause::GameOver => "Game is over.".to_string(),
            InvalidActionErrorCause::NotEnoughMoney(total_cost) => format!("Not enough money, need at least {} dollars.", format_money(total_cost)),
            InvalidActionErrorCause::CheeseMaxQuantityExceeded(max) => format!("Cannot buy that much cheese (max {}).", max),
        }
    }
}

impl fmt::Display for InvalidActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

pub type ActionResult<T> = std::result::Result<T, InvalidActionError>;


pub fn format_money(raw_money: i32) -> String {
    format!("{}.{:02}", raw_money / 100, raw_money % 100)
}


#[derive(Deserialize)]
pub struct CroissantGameConfig {
    pub turns: i32,
    pub starting_money: i32,
    pub cook_payoff: i32,
    pub cheese_cost: i32,
    pub cheese_quantity_maximum: u32,
    pub cheese_mature_turns: i32,
    pub cheese_payoff: i32,
    pub recipe_cost: i32,
    pub recipe_dividend: i32,
    pub cookbook_cost: i32,
    pub cookbook_dividend: i32,
    pub croissant_starting_price: i32,
    pub croissant_quantity_maximum: u32,
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
        for i in 0..self.cheeses.len() {
            self.cheeses[i] += 1;
        }
    }

    pub fn execute_cook(&mut self) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        self.money += self.config.cook_payoff;
        self.end_turn();
        Ok(())
    }

    pub fn execute_buy_cheese(&mut self, quantity: u32) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        // TODO: reject quantity==0
        if quantity > self.config.cheese_quantity_maximum {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::CheeseMaxQuantityExceeded(self.config.cheese_quantity_maximum) });
        }
        let total_cost = self.config.cheese_cost * quantity as i32;
        if total_cost > self.money {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::NotEnoughMoney(total_cost) });
        }
        self.money -= total_cost;
        let mut new_cheeses = vec![ 0 ; quantity as usize ];
        self.cheeses.append(&mut new_cheeses);
        self.end_turn();
        Ok(())
    }

    pub fn execute_buy_croissants(&mut self, quantity: u32) -> ActionResult<()> {
        todo!();
        Ok(())
    }
}
