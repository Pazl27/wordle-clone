use crate::services::database;
use std::collections::HashMap;
use crate::router::Valid;

pub fn adjust_score_by_attempt(user: &mut database::User) {
    user.attempts += 1;
    if user.attempts <= 1 {
        return;
    }
    user.score -= 200;
}

pub fn adjust_score_by_guess(user: &mut database::User, in_word: &HashMap<i8, char>, right_place: &HashMap<i8, char>) {
    user.score += in_word.len() as i32 * 50;
    user.score += right_place.len() as i32 * 100;
}

pub fn correct_guess(user: &mut database::User, correct_word: &Valid) {
    match correct_word {
        Valid::Pass => user.score += 500,
        Valid::Fail => {},
    }
}

pub async fn get_users() -> Vec<database::User> {
    let pool = database::establish_connection().await.unwrap();
    database::get_users(&pool).await.unwrap()
}

pub async fn timestamp_score(user: &mut database::User) {
    let pool = database::establish_connection().await.unwrap();
    match database::get_duration(&pool, &user.id).await {
        Ok(Some(diff)) => {
            let base_score = 100;
            let base_time = 120;

            let new_score = base_score * (base_time / diff);

            user.score += new_score as i32;

            let _ = database::update_user(&pool, user).await;
        }
        Ok(None) => {
            println!("No duration found");
        }
        Err(e) => {
            eprintln!("Error getting duration: {}", e);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;
    use crate::router::Valid;
    use crate::services::database::User;

    #[test]
    fn test_adjust_score_by_attempt() {
        let mut user = User {
            id: uuid::Uuid::new_v4(),
            name: "Test".to_string(),
            score: 1000,
            attempts: 0,
            word: "test".to_string(),
        };
        adjust_score_by_attempt(&mut user);
        assert_eq!(user.attempts, 1);
        assert_eq!(user.score, 800);
    }

    #[test]
    fn test_adjust_score_by_guess() {
        let mut user = User {
            id: uuid::Uuid::new_v4(),
            name: "Test".to_string(),
            score: 1000,
            attempts: 0,
            word: "test".to_string(),
        };
        let mut in_word = HashMap::new();
        in_word.insert(0, 't');
        in_word.insert(1, 'e');
        let mut right_place = HashMap::new();
        right_place.insert(0, 't');
        right_place.insert(1, 'e');
        adjust_score_by_guess(&mut user, &in_word, &right_place);
        assert_eq!(user.score, 1300);
    }

    #[test]
    fn test_correct_guess() {
        let mut user = User {
            id: uuid::Uuid::new_v4(),
            name: "Test".to_string(),
            score: 1000,
            attempts: 0,
            word: "test".to_string(),
        };
        correct_guess(&mut user, &Valid::Pass);
        assert_eq!(user.score, 1500);
    }

    #[test]
    fn test_correct_guess_fail() {
        let mut user = User {
            id: uuid::Uuid::new_v4(),
            name: "Test".to_string(),
            score: 1000,
            attempts: 0,
            word: "test".to_string(),
        };
        correct_guess(&mut user, &Valid::Fail);
        assert_eq!(user.score, 1000);
    }
}
