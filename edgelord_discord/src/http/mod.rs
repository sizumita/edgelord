#[derive(Default)]
pub struct HttpClient {
    token: String,
    application_id: String,
}

impl HttpClient {
    pub fn new(token: &str, application_id: &str) -> Self {
        Self {
            token: token.to_string(),
            application_id: application_id.to_string(),
        }
    }
}
