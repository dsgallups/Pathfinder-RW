use diesel::{
    PgConnection,
    r2d2::{
        PooledConnection,
        ConnectionManager
    }
};

use crate::{
    models::{
        associations::{
            ComponentToComponent,
            DegreeToComponent
        },
        component::Component,
        degree::Degree
    },
    handlers::types::ComponentLogic
};
use diesel::result::Error;


struct Req {
    name: String,
    logic_type: Option<ComponentLogic>,
    pftype: String,
    requirements: Vec<Req>
}
pub struct Schedule {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    pub degree: Degree,
    root_components: Vec<Component>
}

impl Schedule {
    pub fn new(mut conn: PooledConnection<ConnectionManager<PgConnection>>, degree_code: &str) -> Result<Self, Error> {

        let degree = Degree::find_by_code(degree_code, &mut conn)?;

        let root_components = DegreeToComponent::get_components(&degree, &mut conn)?;

        Ok(Self { conn , degree, root_components })
    }

    pub fn build_schedule(&mut self) -> Result<String, diesel::result::Error> {
        
        //get the degree root components
        //all of these components must be satisfied for the schedule
        
        println!("Component List: {:?}", &self.root_components);
        


        Ok(String::from("Success!"))
    }

    /**
     * This function will display a full tree of every root component, and every component
     * which satisifes its conditions.
     * 
     */
    /*
        Example:
        [
            {
                name: CNIT CORE,
                logic_type: AND,
                pftype: logical
                requirements: [
                    {
                        name: CNIT 27000,
                        logic_type: None,
                        pftype: class,
                        requirements: None
                    },
                    {
                        name: CNIT NETWORKING FUNDAMENTALS,
                        logic_type: OR,
                        pftype: logical,
                        requirements: [
                            {
                                name: CNIT 34400
                                ...
                            }
                            {
                                name: CNIT 24000
                            }
                        ]
                    }
                ]
            },
            {
                name: CNIT 315
                logic_type: None,
                pftype: class
            }

        ]
    
    */
    pub fn display_requirements_tree(&mut self) {

        for comp in &self.root_components {
            //So we're going to basically take the values from this,
            //and its relationships to other components
            //and put it in our own personal struct
        }
    }
}