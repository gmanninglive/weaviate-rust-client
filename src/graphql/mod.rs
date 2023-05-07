use crate::Connection;

pub use self::getter::GraphQLGetter;

mod getter;

pub struct GraphQL {
    pub getter: GraphQLGetter,
}

impl GraphQL {
    pub fn new(conn: Connection) -> Self {
        Self {
            getter: GraphQLGetter::new(conn),
        }
    }
}
