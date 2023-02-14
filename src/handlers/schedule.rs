use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use thiserror::Error;

use crate::models::{
    associations::{ComponentToComponent, DegreeToComponent},
    component::Component,
    degree::Degree,
};

#[derive(Debug)]
struct Req {
    component: Component,
    children: Vec<usize>,
    parents: Vec<usize>,
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
pub struct Schedule {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    pub degree: Degree,
    reqs: Vec<Req>,
}

impl Schedule {
    pub fn new(
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
        degree_code: &str,
    ) -> Result<Self, ScheduleError> {
        let degree = Degree::find_by_code(degree_code, &mut conn)?;

        let reqs: Vec<Req> = Vec::new();

        Ok(Self { conn, degree, reqs })
    }

    pub fn build_schedule(&mut self) -> Result<String, ScheduleError> {
        //get the degree root components
        //all of these components must be satisfied for the schedule

        //println!("Component List: {:?}", &self.root_components);
        self.build_requirements_graph()?;

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
    pub fn build_requirements_graph(&mut self) -> Result<(), ScheduleError> {
        //build a root node for the degree
        //TODO: this is extremely poor practice...notably giving it an id of -1.
        let degree_component = Component {
            id: -1,
            name: self.degree.name.to_string(),
            pftype: "Degree".to_string(),
            logic_type: Some("GroupAND".to_string()),
        };
        let req = Req {
            component: degree_component,
            children: Vec::new(),
            parents: Vec::new(),
        };

        let root_components = DegreeToComponent::get_components(&self.degree, &mut self.conn)?;

        self.reqs.push(req);
        let id = self.reqs.len() - 1;

        self.associate_components(id, root_components, 0)?;

        println!("\n\n------------------------------------------Begin Reqs------------------------------------------");
        for (pos, req) in self.reqs.iter().enumerate() {
            println!("{:>2}: {:?}", pos, req);
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        Ok(())
    }

    pub fn associate_components(
        &mut self,
        parent_id: usize,
        components: Vec<Component>,
        nests: usize,
    ) -> Result<(), ScheduleError> {
        for component in components {
            let spaces = 4 * nests;
            let spacing = (0..=spaces).map(|_| " ").collect::<String>();
            let extra_space = (0..=4).map(|_| " ").collect::<String>();
            println!("{}Component: {:?}", &spacing, &component);

            //Determine if this component is already in reqs
            match self
                .reqs
                .iter()
                .position(|req| req.component.id == component.id)
            {
                Some(id) => {
                    println!(
                        "{}-----------------START COMPONENT (already exists, req_id: {})-----------------",
                        &spacing, id
                    );
                    //push the parent id to this component
                    self.reqs[id].parents.push(parent_id);

                    //push this id to the parent's children
                    self.reqs[parent_id].children.push(id);

                    println!(
                        "{}Associated parent (req_id: {}) to this child (req_id: {})",
                        &spacing, parent_id, id
                    );
                    println!(
                        "{}-----------------END COMPONENT (already exists, req_id: {})-----------------\n",
                        &spacing, id
                    );
                    //since this req exists, it has already associated its children.
                    //No need to run it again.
                }
                None => {
                    //Create the req for this component
                    let req = Req {
                        component: component,
                        children: Vec::new(),
                        parents: Vec::new(),
                    };

                    //push this req to the reqs
                    self.reqs.push(req);

                    //get the ID of this component
                    let id = self.reqs.len() - 1;

                    println!(
                        "{}-----------------START COMPONENT (new, req_id: {})-----------------",
                        &spacing, id
                    );

                    //push the parent_id to this component
                    self.reqs[id].parents.push(parent_id);

                    //push this id to the parent component's children
                    self.reqs[parent_id].children.push(id);

                    println!(
                        "{}Associated parent (req_id: {}) to this child (req_id: {})",
                        &spacing, parent_id, id
                    );

                    //get the children of this component
                    let children = ComponentToComponent::get_children(
                        &self.reqs[id].component,
                        &mut self.conn,
                    )?;

                    println!(
                        "{}Grabbed new children of component (req_id: {}):\n{}{}{:?}\n\n",
                        &spacing, id, spacing, extra_space, &children
                    );

                    //recursively call this function
                    self.associate_components(id, children, nests + 1)?;

                    println!(
                        "{}------------------END COMPONENT (new, req_id: {})-----------------\n\n",
                        &spacing, id
                    );
                }
            };
        }
        Ok(())
    }
}
