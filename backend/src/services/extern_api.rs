use crate::router::Valid;

pub async fn valid_word(guess: &str) -> Valid {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", guess);
    let response = reqwest::get(&url).await.expect("Failed to send request");

    if response.status().is_success() {
        Valid::Pass
    } else {
        Valid::Fail
    }
}

#[cfg(test)]
mod extern_api_tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_word() {
        let result = valid_word("test").await;
        assert_eq!(result, Valid::Pass);
    }

    #[tokio::test]
    async fn test_invalid_word() {
        let result = valid_word("testtesttest").await;
        assert_eq!(result, Valid::Fail);
    }
}
