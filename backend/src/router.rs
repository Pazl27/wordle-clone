use actix_web::web;
use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::services::database::{create_user, establish_connection, get_user, update_user, User};
use crate::services::extern_api::valid_word;
use crate::word_provider::{
    find_not_containe, find_right_place, find_same_letters, get_word, is_right_word, remove_duplicates
};

#[derive(PartialEq, Debug, Serialize)]
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
    not_in_word: Vec<char>,
}

#[derive(Serialize)]
struct UserDTO {
    id: Uuid,
    attempts: i32,
    word: String,
    name: String,
}

impl GuessResponseDTO {
    pub fn new(
        valid_word: Valid,
        correct_word: Valid,
        in_word: HashMap<i8, char>,
        right_place: HashMap<i8, char>,
        attempts: i32,
        not_in_word: Vec<char>,
    ) -> GuessResponseDTO {
        GuessResponseDTO {
            valid_word,
            correct_word,
            in_word,
            right_place,
            attempts,
            not_in_word,
        }
    }
}

impl UserDTO {
    pub fn to_dto(user: &User) -> UserDTO {
        UserDTO {
            id: user.id,
            attempts: user.attempts,
            word: "".to_string(),
            name: user.name.clone(),
        }
    }

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

    let valid = valid_word(&guess).await;

    let correct = is_right_word(&user.word, &guess);

    if Valid::Fail == valid {
        let response = GuessResponseDTO::new(
            valid,
            correct,
            HashMap::new(),
            HashMap::new(),
            user.attempts,
            Vec::new(),
        );
        return HttpResponse::Ok().json(response);
    }

    let word = user.word.clone();

    let mut contains_letters = find_same_letters(&word, &guess);
    let right_letters = find_right_place(&word, &guess);
    remove_duplicates(&mut contains_letters, &word);
    let not_in_word = find_not_containe(&word, &guess);

    user.attempts += 1;
    update_user(&pool, &user).await.unwrap();

    let response = GuessResponseDTO::new(
        valid,
        correct,
        contains_letters,
        right_letters,
        user.attempts,
        not_in_word,
    );

    HttpResponse::Ok().json(response)
}

#[get("/api/users")]
async fn get_users() -> impl Responder {
    let pool = establish_connection().await.unwrap();
    let users = crate::services::database::get_users(&pool).await.unwrap();

    let response: Vec<UserDTO> = users.iter().map(UserDTO::to_dto_with_word).collect();

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
