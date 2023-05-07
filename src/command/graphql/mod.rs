use crate::Connection;

pub use self::getter::GraphQLGetter;

mod getter;

pub struct GraphQL<'a> {
    conn: &'a Connection,
}

impl<'a> GraphQL<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn getter(self) -> GraphQLGetter<'a> {
        GraphQLGetter::new(self.conn)
    }
}
