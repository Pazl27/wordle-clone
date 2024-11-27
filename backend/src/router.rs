use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::word_provider::{get_word, is_word_in_list, find_same_letters, find_right_place };
use crate::database::{establish_connection, create_user, get_user, update_user};

#[derive(Serialize)]
pub enum Valid {
    Pass,
    Fail,
}

#[derive(Deserialize)]
struct GuessDTO {
    id: Uuid,
    guess: String,
}

#[derive(Serialize)]
struct GuessResponseDTO {
    valid_word: Valid,
    in_word: HashMap<i8, char>,
    right_place: HashMap<i8, char>,
    attempts: i32,
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
    let pool = establish_connection().await.unwrap();
    let mut user = get_user(&pool, dto.id).await.unwrap();
    let guess = dto.guess.clone();

    let valid = is_word_in_list(&guess);

    match valid {
        Valid::Fail => {
            let response = GuessResponseDTO {
                valid_word: valid,
                in_word: HashMap::new(),
                right_place: HashMap::new(),
                attempts: user.attempts,
            };
            return HttpResponse::Ok().json(response);
        }
        _ => {}
    }

    let word = user.word.clone();

    let contains_letters = find_same_letters(&word, &guess);
    let right_letters = find_right_place(&word, &guess);

    user.attempts += 1;
    update_user(&pool, &user).await.unwrap();

    let response = GuessResponseDTO {
        valid_word: valid,
        in_word: contains_letters,
        right_place: right_letters,
        attempts: user.attempts,
    };

    HttpResponse::Ok().json(response)
}
