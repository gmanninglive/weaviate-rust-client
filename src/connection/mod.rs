mod auth;
pub mod http;

use crate::utils::string::trim_trailing_slash;
pub use auth::Auth;
use auth::OidcCredentials;
use http::HttpClient;
use http::HttpParams;
use reqwest::header::HeaderMap;

pub type Headers = HeaderMap;

pub struct Connection {
    auth: Auth,
    pub client: HttpClient,
}

pub enum AuthParams {
    None,
    AccessToken(String),
    Oidc(OidcCredentials),
}

///
/// # Weaviate ConnectionBuilder
///
/// ## Limitations
/// Oidc Auth is not fully implemented
///
/// ## Examples
/// ```
/// use weaviate_client::{ConnectionBuilder, Connection};
///
/// let conn = ConnectionBuilder::new("http", "localhost:8080").build();
/// ```
#[derive(Default)]
pub struct ConnectionBuilder {
    pub host: String,
    pub scheme: String,
    pub headers: Option<Headers>,
    pub auth: Auth,
}

impl ConnectionBuilder {
    pub fn new(scheme: impl Into<String>, host: impl Into<String>) -> Self {
        Self {
            scheme: scheme.into(),
            host: host.into(),
            ..Default::default()
        }
    }

    pub fn headers(&mut self, headers: Headers) -> &mut Self {
        let _ = self.headers.insert(headers);
        self
    }

    pub fn auth(&mut self, auth: Auth) -> &mut Self {
        self.auth = auth;
        self
    }

    pub fn build(&self) -> Connection {
        Connection {
            auth: self.auth.clone(),
            client: HttpClient::new(HttpParams {
                auth: self.auth.clone(),
                scheme: self.scheme.clone(),
                host: trim_trailing_slash(self.host.clone()),
                headers: self.headers.clone(),
            }),
        }
    }
}

impl Connection {
    pub fn auth_enabled(&self) -> bool {
        match self.auth {
            Auth::ApiKey(_) => true,
            Auth::Oidc(_) => true,
            Auth::None => false,
        }
    }
}
