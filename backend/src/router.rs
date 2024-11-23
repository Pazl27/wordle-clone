use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::word_provider::{get_word, is_word_in_list};
use crate::database::{establish_connection, create_user};

#[derive(Serialize)]
pub enum Valid {
    Pass,
    Fail,
}

#[derive(Deserialize)]
struct GuessDTO {
    guess: String,
}

#[derive(Serialize)]
struct GuessResponseDTO {
    correct: Valid,
    in_word: Vec<String>,
    right_place: Vec<String>,
}

#[derive(Serialize)]
struct UserDTO {
    id: Uuid,
    attempts: i32,
}

#[post("/start")]
async fn start_game() -> impl Responder {
    let pool = establish_connection().await.unwrap();

    match get_word() {
        Ok(word) => {
            match create_user(&pool, &word).await {
                Ok(user) => {
                    let response = UserDTO {
                        id: user.id,
                        attempts: user.attempts,
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/guess")]
async fn guess(dto: web::Json<GuessDTO>) -> impl Responder {
    let guess = dto.guess.clone();

    let valid = is_word_in_list(&guess);

    let response = GuessResponseDTO {
        correct: valid,
        in_word: vec![],
        right_place: vec![],
    };

    HttpResponse::Ok().json(response)
}
