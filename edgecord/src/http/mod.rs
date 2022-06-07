mod bucket;
pub mod guild;
pub mod interaction;
mod route;

use cfg_if::cfg_if;
pub use route::Routes;

use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use twilight_model::id::marker::ApplicationMarker;
use twilight_model::id::Id;
use worker::Method;

cfg_if! {
    if #[cfg(all(not(target_arch = "wasm32"), feature = "local"))] {
        use std::str::FromStr;
    } else {
        use worker::wasm_bindgen::JsValue;
        use edgelord::http::RequestBuilder;
    }
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    token: String,
    application_id: Id<ApplicationMarker>,
    ua: String,
    #[cfg(not(target_arch = "wasm32"))]
    _http: reqwest::Client,
}

#[allow(dead_code)]
const BASE_URL: &str = "https://discord.com/api/v10";

impl HttpClient {
    pub fn new(token: &str, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            token: format!("Bot {token}"),
            application_id,
            ua: "Discord Bot (https://github.com/sizumita/edgelord 0.0.1)".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            _http: reqwest::Client::new(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn request<B, T>(
        &self,
        method: Method,
        route: Routes,
        body: Option<B>,
    ) -> crate::Result<Option<T>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let mut response = RequestBuilder::new(&*format!("{}{}", BASE_URL, route))
            .method(method)
            .header("Content-Type", "application/json")
            .header("Authorization", &*self.token)
            .header("User-Agent", &*self.ua)
            .body(body.and_then(|b| Some(JsValue::from(&*serde_json::to_string(&b).unwrap()))))
            .send()
            .await
            .unwrap();

        match response.status_code() {
            204 => Ok(None),
            i if i < 399 => Ok(Some(response.json::<T>().await.unwrap())),
            403 => Err(Error::Forbidden),
            404 => Err(Error::NotFound),
            // TODO: add errors
            _ => Err(Error::HttpError(response.text().await.unwrap())),
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "local"))]
    pub async fn request<B, T>(
        &self,
        method: Method,
        route: Routes,
        body: Option<B>,
    ) -> crate::Result<Option<T>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            ._http
            .request(
                reqwest::Method::from_str(&*method.to_string()).unwrap(),
                &*format!("{}{}", BASE_URL, route),
            )
            .header("Content-Type", "application/json")
            .header("Authorization", &*self.token)
            .header("User-Agent", &*self.ua)
            .body(
                body.map(|x| serde_json::to_string(&x).unwrap())
                    .unwrap_or_default(),
            )
            .send()
            .await
            .unwrap();

        if response.status().clone().is_success() {
            if response.status().as_u16() == 204 {
                return Ok(None);
            }
            return Ok(Some(response.json::<T>().await.unwrap()));
        }
        match response.status() {
            reqwest::StatusCode::FORBIDDEN => Err(Error::Forbidden),
            reqwest::StatusCode::NOT_FOUND => Err(Error::NotFound),
            _ => Err(Error::HttpError(response.text().await.unwrap_or_default())),
        }
    }
}
