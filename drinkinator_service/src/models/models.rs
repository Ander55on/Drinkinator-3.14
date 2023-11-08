use serde::Deserialize;

#[derive(Deserialize)]
struct GetDrink {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct GetIngredient {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct Ingredient {
    name: String,
}

#[derive(Deserialize)]
struct Instruction {
    ingredient: Ingredient,
    measurement: Measurement,
}

#[derive(Deserialize)]
enum Unit {
    #[serde(rename = "cl")]
    Cl,
    #[serde(rename = "ml")]
    Ml
}

#[derive(Deserialize)]
struct Measurement {
    quantity: u32,
    unit: Unit
} 

