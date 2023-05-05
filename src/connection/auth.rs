use std::default;

pub type ApiKey = Option<String>;

#[derive(Clone, Default)]
pub enum Auth {
    #[default]
    None,
    ApiKey(String),
    Oidc(Oidc),
}

#[derive(Clone)]
pub struct OidcToken {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
}

#[derive(Clone)]
pub struct OidcCredentials {}

#[derive(Clone)]
pub struct Oidc {
    pub token: Option<OidcToken>,
    pub credentials: OidcCredentials,
}

pub struct OidcAuthenticator {}
impl OidcAuthenticator {
    pub fn new(credentials: &OidcCredentials) -> Self {
        unimplemented!("oidc not implemented!")
    }

    pub fn refresh(&self) -> Oidc {
        unimplemented!("oidc not implemented!")
    }
}
