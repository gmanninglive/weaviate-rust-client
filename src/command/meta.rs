use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Command, CommandTrait};

pub type MetaGetter = Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModuleInfo {
    version: String,
    word_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct MetaResponse {
    hostname: String,
    modules: HashMap<String, ModuleInfo>,
    pub version: String,
}

#[async_trait::async_trait]
impl<'a> CommandTrait<MetaResponse> for MetaGetter {
    async fn r#do(&self) -> Result<MetaResponse, anyhow::Error> {
        let res: MetaResponse = self
            .client
            .get("/meta".to_string(), None)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    fn validate() {
        unimplemented!()
    }
}
