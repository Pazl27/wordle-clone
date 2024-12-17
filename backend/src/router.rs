use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::word_provider::{get_word, find_same_letters, find_right_place, is_right_word};
use crate::database::{User, establish_connection, create_user, get_user, update_user};

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
    correct_word: Valid,
    in_word: HashMap<i8, char>,
    right_place: HashMap<i8, char>,
    attempts: i32,
}

#[derive(Serialize)]
struct UserDTO {
    id: Uuid,
    attempts: i32,
    word: String,
    name: String,
}

impl GuessResponseDTO {
    pub fn new(valid_word: Valid, correct_word: Valid, in_word: HashMap<i8, char>, right_place: HashMap<i8, char>, attempts: i32) -> GuessResponseDTO {
        GuessResponseDTO {
            valid_word,
            correct_word,
            in_word,
            right_place,
            attempts,
        }
    }
}

impl UserDTO {
    // TODO: maybe don't clone
    pub fn to_dto(user: &User) -> UserDTO {
        UserDTO {
            id: user.id,
            attempts: user.attempts,
            word: "".to_string(),
            name: user.name.clone(),
        }
    }

    // TODO: maybe don't clone
    pub fn to_dto_with_word(user: &User) -> UserDTO {
        UserDTO {
            id: user.id,
            attempts: user.attempts,
            word: user.word.clone(),
            name: user.name.clone(),
        }
    }
}

#[post("/api/start")]
async fn start_game() -> impl Responder {
    let pool = establish_connection().await.unwrap();

    match get_word() {
        Ok(word) => {
            match create_user(&pool, &word.to_lowercase()).await {
                Ok(user) => {
                    // TODO: change function to .to_dto()
                    let response = UserDTO::to_dto_with_word(&user);
                    HttpResponse::Ok().json(response)
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/api/guess")]
async fn guess(dto: web::Json<GuessDTO>) -> impl Responder {
    let pool = establish_connection().await.unwrap();
    let mut user = get_user(&pool, dto.id).await.unwrap();
    let guess = dto.guess.clone();

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", guess);
    let response = reqwest::get(&url).await.expect("Failed to send request");

    let valid = if response.status().is_success() {
        Valid::Pass
    } else {
        Valid::Fail
    };

    let correct = is_right_word(&user.word, &guess);

    match valid {
        Valid::Fail => {
            let response = GuessResponseDTO::new(valid, correct, HashMap::new(), HashMap::new(), user.attempts);
            return HttpResponse::Ok().json(response);
        }
        _ => {}
    }

    let word = user.word.clone();

    let contains_letters = find_same_letters(&word, &guess);
    let right_letters = find_right_place(&word, &guess);

    user.attempts += 1;
    update_user(&pool, &user).await.unwrap();

    let response = GuessResponseDTO::new(valid, correct, contains_letters, right_letters, user.attempts);

    HttpResponse::Ok().json(response)
}

#[get("/api/users")]
async fn get_users() -> impl Responder {
    let pool = establish_connection().await.unwrap();
    let users = crate::database::get_users(&pool).await.unwrap();

    let response: Vec<UserDTO> = users.iter().map(|u| UserDTO::to_dto_with_word(u)).collect();

    HttpResponse::Ok().json(response)
}

#[get("/api/user/word/{id}")]
async fn get_user_word(id: web::Path<Uuid>) -> impl Responder {
    let pool = establish_connection().await.unwrap();
    let user = get_user(&pool, id.into_inner()).await.unwrap();

    HttpResponse::Ok().json(user.word)
}

#[get("/api/user/score/{id}")]
async fn get_user_score(id: web::Path<Uuid>) -> impl Responder {
    let pool = establish_connection().await.unwrap();
    let user = get_user(&pool, id.into_inner()).await.unwrap();

    HttpResponse::Ok().json(user.score)
}

#[post("/api/user/name/{id}")]
async fn set_user_name(id: web::Path<Uuid>, name: web::Json<String>) -> impl Responder {
    let pool = establish_connection().await.unwrap();
    let mut user = get_user(&pool, id.into_inner()).await.unwrap();

    user.name = name.clone();
    update_user(&pool, &user).await.unwrap();

    HttpResponse::Ok().json(name)
}
