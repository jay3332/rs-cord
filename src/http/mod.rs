use crate::constants::DISCORD_API_URL;

pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

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

pub struct HttpClientRequestBuilder<'c, 'r> {
    client: &'c HttpClient,
    reqwest_client: &'c reqwest::Client,
    pub route: Route<'r>,
}

impl<'c, 'r> HttpClientRequestBuilder<'c, 'r> {
    pub async fn send(&self) -> Result<(), ()> {
        let url = self.route.url();
        self.reqwest_client.get(url).send().await.map_err(|_| ())?;

        Ok(())
    }
}

/// Handles authenticated requests to Discord's REST API.
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
        }
    }
}
