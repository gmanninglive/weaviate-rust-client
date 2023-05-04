pub mod http;

use http::HttpClient;
use reqwest::header::HeaderMap;

pub struct OidcAuthenticator {}

pub struct GraphQLClient {}

pub type Headers = HeaderMap;

pub type ApiKey = String;

pub struct Connection {
    api_key: ApiKey,
    auth_enabled: bool,
    gql: GraphQLClient,
    pub http: HttpClient,
    pub oidc_auth: Option<OidcAuthenticator>,
}

pub struct ConnectionParams {
    pub auth_client_secret: Option<String>,
    pub api_key: ApiKey,
    pub host: String,
    pub scheme: String,
    pub headers: Option<Headers>,
}

impl Connection {
    pub fn new(params: ConnectionParams) -> Self {
        Self {
            api_key: params.api_key.clone(),
            auth_enabled: false,
            gql: GraphQLClient {},
            http: HttpClient::new(params),
            oidc_auth: None,
        }
    }
}
