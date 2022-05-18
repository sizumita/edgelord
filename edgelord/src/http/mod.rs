use worker::wasm_bindgen::JsValue;
pub use worker::wasm_bindgen::JsValue as Body;
pub use worker::{Method, Url};

pub type HttpResult = Result<worker::Response, Box<dyn std::error::Error>>;

/**
A convenient function for fetch request for GET.
**/
pub async fn get(url: &str) -> HttpResult {
    RequestBuilder::new(url).send().await
}

/**
A convenient function for fetch request for POST.
 **/
pub async fn post(url: &str, body: Option<Body>) -> HttpResult {
    RequestBuilder::new(url).body(body).send().await
}

/**
Http request builder for fetch function.

# Example

```
use edgelord::http::{RequestBuilder, Method};
use worker::console_log;

let mut res = RequestBuilder::new("https://example.com")
    .method(Method::Delete)
    .send().await?;

console_log!("{}", res.text().await?)
```
**/
pub struct RequestBuilder {
    _url: Url,
    _headers: worker::Headers,
    _method: Method,
    _body: Option<Body>,
}

impl RequestBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            _url: Url::parse(url).unwrap(),
            _headers: worker::Headers::new(),
            _method: Method::Get,
            _body: None,
        }
    }

    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self._headers.append(name, value).unwrap();
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self._method = method;
        self
    }

    pub fn body(&mut self, body: Option<JsValue>) -> &mut Self {
        self._body = body;
        self
    }

    pub async fn send(&self) -> Result<worker::Response, Box<dyn std::error::Error>> {
        let request = worker::Request::new_with_init(
            self._url.as_str(),
            worker::RequestInit::new()
                .with_headers(self._headers.clone())
                .with_method(self._method.clone())
                .with_body(self._body.clone()),
        )?;
        worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|err| err.into())
    }
}

impl From<Url> for RequestBuilder {
    fn from(url: Url) -> Self {
        Self {
            _url: url,
            _headers: worker::Headers::new(),
            _method: Method::Get,
            _body: None,
        }
    }
}
