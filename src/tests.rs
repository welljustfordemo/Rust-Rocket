#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use crate::rocket;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_hello() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/hello/tester").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, tester!");
    }

    #[test]
    fn test_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "world!");
    }
}