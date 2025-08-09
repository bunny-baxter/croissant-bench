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
    NoCheeseToSell,
    CroissantMaxQuantityExceeded(u32),
}

#[derive(Debug, Clone)]
pub struct InvalidActionError {
    pub cause: InvalidActionErrorCause,
}

impl InvalidActionError {
    pub fn describe(&self) -> String {
        match self.cause {
            InvalidActionErrorCause::InvalidAction => "Invalid action.".to_string(),
            InvalidActionErrorCause::InvalidQuantity => "Action requires a quantity greater than 0.".to_string(),
            InvalidActionErrorCause::ExtraneousQuantity => "Action should not have a quantity.".to_string(),
            InvalidActionErrorCause::GameOver => "Game is over.".to_string(),
            InvalidActionErrorCause::NotEnoughMoney(total_cost) => format!("Not enough money, need at least {}.", format_money(total_cost)),
            InvalidActionErrorCause::CheeseMaxQuantityExceeded(max) => format!("Cannot buy that much cheese (max {}).", max),
            InvalidActionErrorCause::NoCheeseToSell => "You have no mature cheese to sell.".to_string(),
            InvalidActionErrorCause::CroissantMaxQuantityExceeded(max) => format!("Cannot buy that many croissants (max {}).", max),
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
    format!("${}.{:02}", raw_money / 100, raw_money % 100)
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
        self.money += self.config.recipe_dividend * self.recipes;
        self.money += self.config.cookbook_dividend * self.cookbooks;
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
        if quantity == 0 {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::InvalidQuantity });
        }
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

    pub fn execute_sell_cheese(&mut self) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        let (mature_cheeses, _non_mature_cheeses) = self.count_cheeses();
        if mature_cheeses == 0 {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::NoCheeseToSell });
        }
        let total_gain = mature_cheeses * self.config.cheese_payoff;
        self.money += total_gain;
        self.cheeses = self.cheeses.iter().filter(|&&age| age < self.config.cheese_mature_turns).cloned().collect();
        self.end_turn();
        Ok(())
    }

    pub fn execute_publish_recipe(&mut self) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        if self.config.recipe_cost > self.money {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::NotEnoughMoney(self.config.recipe_cost) });
        }
        self.money -= self.config.recipe_cost;
        self.recipes += 1;
        self.end_turn();
        Ok(())
    }

    pub fn execute_publish_cookbook(&mut self) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        if self.config.cookbook_cost > self.money {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::NotEnoughMoney(self.config.cookbook_cost) });
        }
        self.money -= self.config.cookbook_cost;
        self.cookbooks += 1;
        self.end_turn();
        Ok(())
    }

    pub fn execute_buy_croissants(&mut self, quantity: u32) -> ActionResult<()> {
        if self.is_game_over() {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::GameOver });
        }
        if quantity == 0 {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::InvalidQuantity });
        }
        if quantity > self.config.croissant_quantity_maximum {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::CroissantMaxQuantityExceeded(self.config.croissant_quantity_maximum) });
        }
        let total_cost = self.croissant_price * quantity as i32;
        if total_cost > self.money {
            return Err(InvalidActionError { cause: InvalidActionErrorCause::NotEnoughMoney(total_cost) });
        }
        self.money -= total_cost;
        self.croissants += quantity as i32;
        self.end_turn();
        Ok(())
    }
}
