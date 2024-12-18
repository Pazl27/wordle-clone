use crate::router::Valid;

pub async fn valid_word(guess: &str) -> Valid {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", guess);
    let response = reqwest::get(&url).await.expect("Failed to send request");

    let valid = if response.status().is_success() {
        Valid::Pass
    } else {
        Valid::Fail
    };

    valid
}
