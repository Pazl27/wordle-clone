use actix_web::{test, App};
use word_provider::get_word;

#[actix_rt::test]
async fn test_get_word() {
    let mut app = test::init_service(App::new().service(get_word)).await;

    // Simulate a request to the /word endpoint
    let req = test::TestRequest::get().uri("/word").to_request();
    let resp = test::call_service(&mut app, req).await;

    // Check if the response is successful
    assert!(resp.status().is_success(), "Expected successful response");

    let body = test::read_body(resp).await;
    assert!(body.len() > 0, "Expected non-empty response body");
}
