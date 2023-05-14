use rocket::http::Status;

use crate::rocket;
use crate::tests::CLIENT;

#[test]
fn test_fetch_security_questions_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_all_security_questions)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}