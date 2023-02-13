use diesel::{
    PgConnection,
    r2d2::{
        PooledConnection,
        ConnectionManager
    }
};

pub struct Schedule {
    conn: PooledConnection<ConnectionManager<PgConnection>>
}

impl Schedule {
    pub fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self { conn }
    }
}