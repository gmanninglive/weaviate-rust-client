use super::Command;
use crate::Connection;

/// ## Misc Commands
#[derive(derive_new::new)]
pub struct Schema<'a> {
    conn: &'a Connection,
}

impl<'a> Schema<'a> {
    pub fn get_schema(self) -> SchemaGetter<'a> {
        SchemaGetter::new(self.conn)
    }
}

#[derive(derive_new::new)]
pub struct SchemaGetter<'a> {
    conn: &'a Connection,
}

#[async_trait::async_trait]
impl<'a> Command<String> for SchemaGetter<'a> {
    async fn r#do(&self) -> Result<String, anyhow::Error> {
        unimplemented!()
    }
    fn validate() {
        unimplemented!()
    }
}
