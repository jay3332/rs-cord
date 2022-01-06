use crate::constants::DISCORD_API_URL;

use reqwest::Method;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT, HeaderMap, HeaderValue};

#[derive(Copy, Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

impl Into<Method> for RequestMethod {
    fn into(self) -> Method {
        match self {
            RequestMethod::GET => Method::GET,
            RequestMethod::POST => Method::POST,
            RequestMethod::PUT => Method::PUT,
            RequestMethod::PATCH => Method::PATCH,
            RequestMethod::DELETE => Method::DELETE,
            RequestMethod::OPTIONS => Method::OPTIONS,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Route<'a> {
    pub method: RequestMethod,
    pub route: &'a str,
}

impl<'a> Route<'a> {
    pub fn url(&self) -> String {
        format!("{}/{}", DISCORD_API_URL, self.route)
    }
}

impl<'a> From<(RequestMethod, &'a str)> for Route<'a> {
    fn from((method, route): (RequestMethod, &'a str)) -> Self {
        Route {
            method,
            route,
        }
    }
}

/// A helper for elegantly building an authenticated HTTP request.
#[derive(Debug)]
pub struct HttpClientRequestBuilder<'c, 'r> {
    client: &'c HttpClient,
    reqwest_client: &'c reqwest::Client,

    /// The route (method + endpoint) to request to.
    pub route: Route<'r>,

    /// The headers to send with the request.
    pub headers: Option<HeaderMap>,

    /// The Content-Type header of this request.
    /// This is separate from `headers` as this is commonly used, and creating a new HeaderMap every request can cause overhead.
    pub content_type: Option<String>,

    /// The raw body to send with the request.
    pub body: Option<Vec<u8>>,

    /// The query parameters to send with the request.
    pub query: Vec<(String, String)>,
}

impl<'c, 'r> HttpClientRequestBuilder<'c, 'r> {
    pub const USER_AGENT: &'static str = concat!(
        "DiscordBot (https://github.com/jay3332/rs-cord), rs-cord ",
        env!("CARGO_PKG_VERSION"),
    );

    fn _build_request(&self) -> Result<reqwest::RequestBuilder, crate::ThreadSafeError> {
        let url = self.route.url();

        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&Self::USER_AGENT)?,
        );

        if let Some(t) = &self.client.token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bot {}", t))?,
            );
        }

        if let Some(c) = &self.content_type {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(&c)?,
            );
        }

        if let Some(h) = &self.headers {
            headers.extend(h.clone());
        }
    
        let mut request = self.reqwest_client.request(self.route.method.into(), url).headers(headers);

        if let Some(b) = &self.body {
            request = request.body(b.clone());
        }

        if self.query.len() > 0 {
            request = request.query(&self.query);
        }

        Ok(request)
    }

    /// Adds a JSON payload to the request. Must be serializable.
    pub fn json(&mut self, json: impl serde::Serialize) -> Result<&mut Self, crate::ThreadSafeError> {
        self.content_type = Some("application/json".to_string());
        self.body = Some(serde_json::to_vec(&json)?);

        Ok(self)
    }

    /// Adds a query parameter to the request.
    pub fn query(&mut self, key: &str, value: impl std::fmt::Display) -> &mut Self {
        self.query.push((key.to_string(), value.to_string()));

        self
    }
 
    /// Sends this request, returning the response sanitized into the given struct.
    /// 
    /// # Example
    /// let raw_msg = client.http.request(route!(POST, "/channels/{channel_id}/messages", channel_id = 1234))
    ///     .json(MessageCreatePayload {
    ///         content: "Hello, world!".to_string(),
    ///     })
    ///     .send_expecting_json::<MessagePayload>()
    ///     .await?;
    pub async fn send_expecting_json<T>(&self) -> Result<T, crate::ThreadSafeError>
        where
            T: serde::de::DeserializeOwned,
    {
        let request = self._build_request()?;
        Ok(request.send().await?.json::<T>().await?)
    }
}

/// Handles authenticated requests to Discord's REST API.
#[derive(Debug)]
pub struct HttpClient {
    /// The internal reqwest client being used.
    client: reqwest::Client,
    
    /// The authentication token to be used.
    token: Option<String>,
}

impl HttpClient {
    pub(crate) fn new() -> Self {
        HttpClient {
            client: reqwest::Client::new(),
            token: None,
        }
    }

    pub(crate) fn new_with_token(token: String) -> Self {
        HttpClient {
            client: reqwest::Client::new(),
            token: Some(token),
        }
    }

    /// Set the authentication token to be used.
    pub(crate) fn set_token(&mut self, token: impl std::fmt::Display) -> &mut Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn request<'c, 'r>(&'c self, route: Route<'r>) -> HttpClientRequestBuilder<'c, 'r> {
        HttpClientRequestBuilder {
            client: self,
            reqwest_client: &self.client,
            route,
            headers: None,
            content_type: None,
            body: None,
            query: Vec::<(String, String)>::new(),
        }
    }
}