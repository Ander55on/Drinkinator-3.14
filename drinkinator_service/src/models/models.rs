use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDrink {
    id: String,
    name: String,
    instructions: Vec<Instruction>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostDrink {
    name: String,
    instructions: Vec<Instruction>
}

#[derive(Debug, Deserialize, Serialize)]
struct GetIngredient {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ingredient {
    name: String,
}

#[derive(Debug, Deserialize,Serialize)]
struct Instruction {
    ingredient: Ingredient,
    measurement: Measurement,
}

#[derive(Debug, Deserialize, Serialize)]
enum Unit {
    #[serde(rename = "cl")]
    Cl,
    #[serde(rename = "ml")]
    Ml
}

#[derive(Debug, Deserialize, Serialize)]
struct Measurement {
    quantity: u32,
    unit: Unit
} 

