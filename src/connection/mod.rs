mod auth;
pub mod http;
pub use auth::Auth;

use crate::utils::string::trim_trailing_slash;
use auth::{Oidc, OidcCredentials};
use http::HttpClient;
use http::HttpParams;
use reqwest::header::HeaderMap;

pub struct GraphQLClient {}

pub type Headers = HeaderMap;

pub struct Connection {
    gql: GraphQLClient,
    auth: Auth,
    pub http: HttpClient,
}

pub enum AuthParams {
    None,
    AccessToken(String),
    Oidc(OidcCredentials),
}

pub struct ConnectionParams {
    pub host: String,
    pub scheme: String,
    pub headers: Option<Headers>,
    pub auth: AuthParams,
}

impl Connection {
    pub fn new(params: ConnectionParams) -> Self {
        let auth = match params.auth {
            AuthParams::AccessToken(key) => Auth::ApiKey(key),
            AuthParams::Oidc(credentials) => Auth::Oidc(Oidc {
                credentials,
                token: None,
            }),
            AuthParams::None => Auth::None,
        };

        Self {
            auth: auth.clone(),
            http: HttpClient::new(HttpParams {
                auth,
                scheme: params.scheme,
                host: trim_trailing_slash(params.host),
                headers: params.headers,
            }),
            gql: GraphQLClient {},
        }
    }
}

pub fn auth_enabled(auth: &Auth) -> bool {
    match auth {
        Auth::ApiKey(_) => true,
        Auth::Oidc(_) => true,
        Auth::None => false,
    }
}
