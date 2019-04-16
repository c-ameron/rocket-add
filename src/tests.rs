use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

fn client() -> Client {
    Client::new(rocket::ignite().mount("/", routes![super::hello, super::add])).unwrap()
}

fn test(uri: &str, expected: String) {
    let client = client();
    assert_eq!(client.get(uri).dispatch().body_string(), Some(expected));
}

fn test_404(uri: &str) {
    let client = client();
    assert_eq!(client.get(uri).dispatch().status(), Status::NotFound);
}

#[test]
fn test_add() {
    for &(num1, num2) in &[(20, 22), (1, 2), (100, 200), (1000, 1001)] {
        test(
            &format!("/add/{}/{}", num1, num2),
            format!("The sum of {} and {} is {}", num1, num2, num1 + num2),
        );
    }
}

#[test]
fn test_failing_add() {
    test_404("/add/abc/1000");
    test_404("/add/33/123.12");
    test_404("/add/10/bar");
}

#[test]
fn test_hello() {
    test("/", format!("Hello, world!"));
}
