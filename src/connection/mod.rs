use std::iter::Map;

pub struct OidcAuthenticator {}

pub struct GraphQLClient {}

pub struct HttpClient {}

struct HeadersInit {}

pub type ApiKey = String;

pub struct Connection {
    api_key: ApiKey,
    auth_enabled: bool,
    gql: GraphQLClient,
    pub http: HttpClient,
    pub oidc_auth: Option<OidcAuthenticator>,
}

pub struct ConnectionParams {
    auth_client_secret: Option<String>,
    api_key: ApiKey,
    host: String,
    scheme: String,
    headers: Option<HeadersInit>,
}

pub struct MetaGetter {}

pub struct ModuleInfo {
    version: String,
    word_count: i32,
}

pub struct MetaResponse {
    hostname: String,
    modules: Map<String, ModuleInfo>,
    pub version: String,
}

impl MetaGetter {
    pub fn new(conn: Connection) -> Self {
        unimplemented!("meta getter command not implemented")
    }

    pub async fn r#do(self) -> Result<MetaResponse, anyhow::Error> {
        unimplemented!("meta do method not implemented")
    }
}

impl Connection {
    fn new(params: ConnectionParams) -> Self {
        Self {
            api_key: params.api_key,
            auth_enabled: false,
            gql: GraphQLClient {},
            http: HttpClient {},
            oidc_auth: None,
        }
    }
}
