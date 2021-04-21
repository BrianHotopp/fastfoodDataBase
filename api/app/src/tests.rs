#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn api_up() {
        /* .. */
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn get_menuentries() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
    #[test]
    fn get_menuentries() {
        /* .. */
    }
    #[test]
    fn get_menuentries() {
        /* .. */
    }
    #[test]
    fn get_menuentries() {
        /* .. */
    }




}