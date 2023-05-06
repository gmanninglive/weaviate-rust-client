use crate::Connection;

use self::getter::GraphQLGetter;

mod getter;

pub struct GraphQL {
    getter: GraphQLGetter,
}

impl GraphQL {
    pub fn new(conn: Connection) -> Self {
        Self {
            getter: GraphQLGetter::new(conn),
        }
    }
}
