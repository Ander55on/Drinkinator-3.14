use core::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

pub enum ModelOperationError {
    CreateError(String),
    DeleteError,
    UpdateError,
    ParseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDrink {
    id: String,
    name: String,
    instructions: Vec<Instruction>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostDrink {
    pub name: String,
    pub instructions: Vec<Instruction>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GetIngredient {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ingredient {
    pub name: String,
}

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct Instruction {
    pub ingredient: Ingredient,
    pub measurement: Measurement,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Unit {
    #[serde(rename = "cl")]
    Cl,
    #[serde(rename = "ml")]
    Ml
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Cl => write!(f, "cl"),
            Unit::Ml => write!(f, "ml"),
        }
    }
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cl" => Ok(Unit::Cl),
            "ml" => Ok(Unit::Ml),
            _ => Err(())
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Measurement {
    pub quantity: u32,
    pub unit: Unit
} 

