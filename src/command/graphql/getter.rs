use crate::{command::Command, Connection};

#[derive(Default)]
struct Properties {
    after: Option<String>,
    ask_string: Option<String>,
    bm25_string: Option<String>,
    class_name: Option<String>,
    fields: Option<String>,
    group_string: Option<String>,
    hybrid_string: Option<String>,
    includes_near_media_filter: bool,
    limit: Option<i32>,
    near_image_string: Option<String>,
    near_object_string: Option<String>,
    near_text_string: Option<String>,
    near_vector_string: Option<String>,
    offset: Option<i32>,
    sort_string: Option<String>,
    where_string: Option<String>,
    generate_string: Option<String>,
    // consistency_level: Option<ConsistencyLevel>,
    group_by_string: Option<String>,
}

pub struct GraphQLGetter<'a> {
    props: Properties,
    conn: &'a Connection,
    errors: Vec<String>,
}

impl<'a> GraphQLGetter<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            props: Properties::default(),
            conn,
            errors: Vec::new(),
        }
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

#[async_trait::async_trait]
impl Command<String> for GraphQLGetter<'_> {
    async fn r#do(&self) -> Result<String, anyhow::Error> {
        todo!();
    }

    fn validate() {
        unimplemented!("validate not implemented for Graphql Getter")
    }
}
