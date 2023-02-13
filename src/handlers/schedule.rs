use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use std::rc::Rc;
use thiserror::Error;

use crate::{
    handlers::types::ComponentLogic,
    models::{
        associations::{ComponentToComponent, DegreeToComponent},
        component::Component,
        degree::Degree,
    },
};

#[derive(Debug)]
struct Req {
    component: Rc<Component>,
    logic_type: Option<ComponentLogic>,
    pftype: String,
    children: Option<Vec<Rc<Req>>>,
    parent: Option<Rc<Req>>,
}

pub struct Schedule {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    pub degree: Degree,
    flat_reqs: Vec<Req>,
}

#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Diesel Error")]
    DieselError(#[from] diesel::result::Error),
}
/**
 * There's two methods of attack here.
 * 1)   First I load all of the components into an array
 *      then I reference the components via their array indice to make a recursive solution
 *
 * 2)   As I call from the database for particular components, I create the struct as they load in
 *      This will utilize the Req Struct
 *      But down the line, when I'm building the schedule, how do I know
 *
 *
 *
 */
impl Schedule {
    pub fn new(
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
        degree_code: &str,
    ) -> Result<Self, ScheduleError> {
        let degree = Degree::find_by_code(degree_code, &mut conn)?;

        let flat_reqs: Vec<Req> = Vec::new();

        Ok(Self {
            conn,
            degree,
            flat_reqs,
        })
    }

    pub fn build_schedule(&mut self) -> Result<String, diesel::result::Error> {
        //get the degree root components
        //all of these components must be satisfied for the schedule

        //println!("Component List: {:?}", &self.root_components);

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

        Another thing to note is that we can store the component information within our
        Req struct...
    */
    pub fn build_requirements_tree(&mut self) {
        todo!();
        /*let root_components = DegreeToComponent::get_components(&self.degree, &mut self.conn)?;

        for comp in root_components {
            //So we're going to basically take the values from this,
            //and its relationships to other components
            //and put it in our own personal struct

            let mut req = Req {
                component: comp,
                logic_type:
            };
        }*/
    }
}
