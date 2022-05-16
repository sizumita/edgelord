#[derive(Default, Clone)]
pub struct HttpClient {
    token: String,
    application_id: String,
    public_key: String,
}

impl HttpClient {
    pub fn new(token: &str, application_id: &str, public_key: &str) -> Self {
        Self {
            token: token.to_string(),
            application_id: application_id.to_string(),
            public_key: public_key.to_string(),
        }
    }
}
