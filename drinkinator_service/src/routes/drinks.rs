use crate::models::models::{ModelOperationError, PostDrink};
use crate::responses::responses::{response_created, response_error_invalid_json};
use futures::io::ReuniteError;
use hyper::body::Bytes;
use hyper::{Body, Response};
use sqlx::{Pool, Sqlite, Row};
use std::convert::Infallible;
use std::error::Error;

pub fn get_all_drinks(pool: Pool<Sqlite>) -> Result<Response<Body>, Infallible> {
  //let q = sqlx::query("SELECT * FROM drinks");
    Ok(Response::new("Getting drinks".into()))
}

pub async fn insert_drink(
    drink: PostDrink,
    pool: Pool<Sqlite>,
) -> Result<PostDrink, ModelOperationError> {

        let return_drink = drink.clone();

        let transaction = match pool.begin().await {
            Ok(transaction) => transaction,
            Err(e) => return Err(ModelOperationError::CreateError(e.to_string())),
        };

        let drink_id = match sqlx::query("INSERT INTO drinks (name) VALUES (?) RETURNING id")
            .bind(&drink.name)
            .fetch_one(&pool)
            .await {
                Ok(id) => id,
                Err(e) => return Err(ModelOperationError::CreateError(e.to_string())),
            };

        let drink_id: u32 = drink_id.get(0);

        for instruction in drink.instructions {
            match sqlx::query("INSERT OR IGNORE INTO ingredients (name) VALUES (?)")
                .bind(&instruction.ingredient.name)
                .execute(&pool)
                .await {
                    Ok(_) => (),
                    Err(e) => return Err(ModelOperationError::CreateError(e.to_string())),
                };
            
            match sqlx::query("INSERT into ingredient_instruction (ingredient_name, drink_id, unit, quantity) VALUES (?, ?, ?, ?)")
            .bind(instruction.ingredient.name)
            .bind(drink_id)
            .bind(instruction.measurement.unit.to_string())
            .bind(instruction.measurement.quantity)
            .execute(&pool)
            .await {
                Ok(_) => (),
                Err(e) => return Err(ModelOperationError::CreateError(e.to_string())),
            };
        }
        transaction.commit();

        Ok(return_drink)
}
