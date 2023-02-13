use diesel::{
    PgConnection,
    r2d2::{
        PooledConnection,
        ConnectionManager
    }
};

use crate::models::{
    associations::{
        ComponentToComponent,
        DegreeToComponent
    },
    degree::Degree
};
use diesel::result::Error;

pub struct Schedule {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    degree: Degree
}

impl Schedule {
    pub fn new(mut conn: PooledConnection<ConnectionManager<PgConnection>>, degree_code: &str) -> Result<Self, Error> {

        let degree = Degree::find_by_code(degree_code, &mut conn)?;

        Ok(Self { conn , degree })
    }

    pub fn build_schedule(&mut self) -> Result<String, diesel::result::Error> {
        let component_list = DegreeToComponent::get_components(&self.degree, &mut self.conn);
        println!("Component List: {:?}", component_list);

        Ok(String::from("Success!"))
    }
}