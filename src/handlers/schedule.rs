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
        
        //get the degree root components
        //all of these components must be satisfied for the schedule
        let root_components = DegreeToComponent::get_components(&self.degree, &mut self.conn)?;
        println!("Component List: {:?}", root_components);
        


        Ok(String::from("Success!"))
    }

    /**
     * This function will display a full tree of every root component, and every component
     * which satisifes its conditions.
     * 
     */
    /*
        Example:
        []
            CNIT CORE,
            AND,

        }
    
    */
    pub fn display_requirements_tree(&mut self) {


    }
}