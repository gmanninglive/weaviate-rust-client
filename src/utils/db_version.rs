use crate::{
    command::{misc::MetaGetter, Command},
    connection::Connection,
};

#[async_trait::async_trait]
pub trait VersionProvider {
    async fn get_version() -> String;
}

pub struct DbVersionSupport {
    db_version_provider: DbVersionProvider,
}

struct SupportReponse {
    version: String,
    supports: bool,
    warns: VersionWarnings,
}

impl DbVersionSupport {
    fn new(db_version_provider: DbVersionProvider) -> Self {
        Self {
            db_version_provider,
        }
    }

    async fn supports_classname_namespaced_endpoints_future(self) -> SupportReponse {
        let meta = self.db_version_provider.version_getter.r#do().await;
        let version = match meta {
            Ok(meta) => meta.version,
            Err(_) => "".to_string(),
        };

        SupportReponse {
            version: version.clone(),
            supports: true,
            warns: VersionWarnings { version },
        }
    }

    // >= 1.14
    fn supports_classname_namespaced_endpoints(version: Option<String>) -> bool {
        return match version {
            Some(version) => {
                let version_numbers: Vec<&str> = version.split('.').collect();
                if version_numbers.len() >= 2 {
                    let major: i8 = version_numbers[0].parse().unwrap();
                    let minor: i8 = version_numbers[1].parse().unwrap();
                    return (major == 1 && minor >= 14) || major >= 2;
                }
                return false;
            }
            None => false,
        };
    }
}

#[derive(Clone)]
pub struct DbVersionProvider {
    version: Option<String>,
    empty_version: String,
    version_getter: MetaGetter,
}

#[async_trait::async_trait]
impl VersionProvider for DbVersionProvider {
    async fn get_version() -> String {
        unimplemented!("get version")
    }
}

impl DbVersionProvider {
    pub fn new(conn: &Connection) -> Self {
        Self {
            version: None,
            empty_version: "".to_owned(),
            version_getter: MetaGetter::new(conn),
        }
    }

    pub async fn get(mut self) -> Result<String, anyhow::Error> {
        match self.version {
            Some(version) => Ok(version),
            None => {
                let version = self.version_getter.r#do().await?.version;
                Ok(self.version.insert(version).to_owned())
            }
        }
    }
}

struct VersionWarnings {
    version: String,
}

impl VersionWarnings {
    fn deprecated_non_classname_namespaced_endpoints_for_objects(self) {
        log::warn!(
        "Usage of objects paths without className is deprecated in Weaviate {}. Please provide className parameter"
      , self.version);
    }

    fn deprecated_non_classname_namespaced_endpoints_for_references(self) {
        log::warn!(
        "Usage of references paths without className is deprecated in Weaviate {}. Please provide className parameter"
     ,self.version);
    }

    fn deprecated_non_classname_namespaced_endpoints_for_beacons(self) {
        log::warn!(
        "Usage of beacons paths without className is deprecated in Weaviate {}. Please provide className parameter"
     , self.version );
    }
    fn not_supported_classname_namespaced_endpoints_for_objects(self) {
        log::warn!(
        "Usage of objects paths with className is not supported in Weaviate {}. className parameter is ignored"
     , self.version );
    }
    fn not_supported_classname_namespaced_endpoints_for_references(self) {
        log::warn!(
        "Usage of references paths with className is not supported in Weaviate {}. className parameter is ignored"
     , self.version );
    }
    fn not_supported_classname_namespaced_endpoints_for_beacons(self) {
        log::warn!(
        "Usage of beacons paths with className is not supported in Weaviate {}. className parameter is ignored"
     , self.version );
    }
    fn not_supported_class_parameter_in_endpoints_for_objects(self) {
        log::warn!(
        "Usage of objects paths with class query parameter is not supported in Weaviate {}. class query parameter is ignored"
     , self.version );
    }
}
