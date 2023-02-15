use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use thiserror::Error;

use crate::models::{
    associations::{ComponentToComponent, DegreeToComponent},
    class::Class,
    component::Component,
    degree::Degree,
};

#[derive(Debug, Clone)]
struct Req {
    component: Component,
    class: Option<Class>,
    children: Vec<(usize, Status)>,
    parents: Vec<(usize, Status)>,
}

impl Req {
    pub fn satisfy_requirement(&mut self) -> Result<(), ScheduleError> {
        Ok(())
    }
}

struct Cost<'a> {
    index: usize,
    //An array that follows a path of components to satisfy and cost
    path_cost: Vec<(Vec<&'a Req>, i32)>,
}

#[derive(Debug, Clone)]
enum Status {
    Unchecked,
    Checked,
    CheckedAndSelected,
}

use Status::{Checked, CheckedAndSelected, Unchecked};

use super::types::LogicalType;

#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Diesel Error")]
    DieselError(#[from] diesel::result::Error),
}

pub struct Schedule {
    pub periods: Vec<Period>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            periods: Vec::new(),
        }
    }
}

pub struct Period {
    year: u32,
    time: String,
    classes: Vec<Component>,
}
/**
 * Uses an adjacent array to build a graph via the Req struct.
 */
pub struct ScheduleMaker {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    pub degree: Degree,
    reqs: Vec<Req>,
    schedule: Option<Schedule>,
}

impl ScheduleMaker {
    pub fn new(
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
        degree_code: &str,
    ) -> Result<Self, ScheduleError> {
        let degree = Degree::find_by_code(degree_code, &mut conn)?;

        let reqs: Vec<Req> = Vec::new();

        Ok(Self {
            conn,
            degree,
            reqs,
            schedule: None,
        })
    }

    pub fn build_schedule(&mut self) -> Result<String, ScheduleError> {
        //get the degree root components
        //all of these components must be satisfied for the schedule

        //This builds our graph in an adjacency matrix stores in self.reqs
        //Note that the degree itself is modeled into a fake req
        //This degree root is at self.reqs[0]
        self.build_requirements_graph()?;

        println!("\n\nbuild_requirements_graph() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for (pos, req) in self.reqs.iter().enumerate() {
            println!("{:>2}: {:?}", pos, req);
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        //turn the requirements graph into a schedule
        self.analyze_requirements_graph()?;

        println!("\n\nanalyze_requirements_graph() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for (pos, req) in self.reqs.iter().enumerate() {
            println!("{:>2}: {:?}", pos, req);
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        Ok(String::from("Success!"))
    }

    /**
     * This function will display a full tree of every root component, and every component
     * which satisifes its conditions.
     *
     */
    fn build_requirements_graph(&mut self) -> Result<(), ScheduleError> {
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
            class: None,
            children: Vec::new(),
            parents: Vec::new(),
        };

        let root_components = DegreeToComponent::get_components(&self.degree, &mut self.conn)?;

        self.reqs.push(req);
        let id = self.reqs.len() - 1;

        //TODO, FIX: 0: Req { component: Component { id: -1, name: "TEST MAJOR", pftype: "Degree", logic_type: Some("GroupAND") }, class: None, children: [(0, Unchecked), (0, Unchecked)], parents: [] }
        println!("Root component: {:?}", &self.reqs[id]);

        self.associate_components(id, root_components, 0)?;

        Ok(())
    }

    fn associate_components(
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
                    self.reqs[id].parents.push((parent_id, Unchecked));
                    println!(
                        "{}Gave self (req_id: {}) a parent (req_id: {})",
                        &spacing, id, parent_id
                    );

                    //push this id to the parent's children
                    self.reqs[parent_id].children.push((id, Unchecked));
                    println!(
                        "{}Gave parent (req_id: {}) a child(req_id: {})",
                        &spacing, parent_id, id
                    );

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
                    //If this component is a class...
                    let class = if component.pftype.eq("Class") {
                        Some(Class::find_by_component_id(&component.id, &mut self.conn)?)
                    } else {
                        None
                    };

                    //Create the req for this component
                    let req = Req {
                        component,
                        class,
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
                    self.reqs[id].parents.push((parent_id, Unchecked));
                    println!(
                        "{}Gave self (req_id: {}) a parent (req_id: {})",
                        &spacing, id, parent_id
                    );

                    //push this id to the parent component's children
                    self.reqs[parent_id].children.push((id, Unchecked));
                    println!(
                        "{}Gave parent (req_id: {}) a child(req_id: {})",
                        &spacing, parent_id, id
                    );

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

    /**
     * This function should only be called once the graph is made in build_requirements_graph()
     */
    fn analyze_requirements_graph(&mut self) -> Result<(), ScheduleError> {
        let schedule = Schedule::new();

        println!("Now generating schedule....");

        self.satisfy_requirements(&mut (0, Unchecked))?;
        //since this root component has a logic type of GroupAND, all of its requirements MUST
        //be fulfilled

        Ok(())
    }

    fn satisfy_requirements(
        &mut self,
        requirement_info: &mut (usize, Status),
    ) -> Result<i32, ScheduleError> {
        //println!("called satisfy_requirements");
        //borrow checker doesn't like that I'm mutating its memory and also calling it.
        //The easiest way to fix this is to make a clone of it and then set the requirement to that
        //clone after manipulation. This will decrease performance but is guaranteed to be safe

        let requirement_indice = requirement_info.0;
        let requirement_status = &mut requirement_info.1;

        let mut requirement = self.reqs[requirement_indice].clone();

        if let Some(logic_type) = &requirement.component.logic_type {
            let children = &mut requirement.children;

            let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);

            match logic_type.as_str() {
                "GroupAND" => {
                    //Since all of the degrees in our catalog is GroupAND, we
                    let children = &mut requirement.children;

                    //So we want to make sure that the current requirement is satisfied
                    //To do this, we need to store a value that tells us if the
                    //requirement is selected.

                    //Make sure that we get a return value that will tell us if it is checked
                    for child in children {
                        self.satisfy_requirements(child)?;

                        child.1 = CheckedAndSelected;
                        for parent in &mut self.reqs[child.0].parents {
                            if parent.0 == requirement_indice {
                                parent.1 = CheckedAndSelected;
                                break;
                            }
                        }
                    }
                }
                "GroupOR" => {
                    for (internal_indice, child) in children.into_iter().enumerate() {
                        let result = self.satisfy_requirements(child)?;
                        println!(
                            "Minimal cost: {:?}, result for req_id {}: {}",
                            minimal_cost, child.0, result
                        );
                        minimal_cost = if result < minimal_cost.1 {
                            (internal_indice, result)
                        } else {
                            minimal_cost
                        };

                        child.1 = Checked;
                        //also check the parent
                        for parent in &mut self.reqs[child.0].parents {
                            if parent.0 == requirement_indice {
                                parent.1 = Checked;
                                break;
                            }
                        }
                    }

                    //now set the selected indice to checkedandselected
                    //I wonder if we should instead provide it a path cost....hmm.
                    /*
                       Where the requirement would have something like
                       (MA 16010, 5, Required),
                       (MA 162, 3, Best)
                    */
                    children[minimal_cost.0].1 = CheckedAndSelected;

                    //Also, on the child, make sure to add checkedandselected to its parent
                    let child_indice_in_reqs = children[minimal_cost.0].0;

                    //Note that we aren't copying this and setting the value to it. We are
                    //going straight to the value in self.reqs and changing it.

                    for parent in &mut self.reqs[child_indice_in_reqs].parents {
                        if parent.0 == requirement_indice {
                            parent.1 = CheckedAndSelected;
                            break;
                        }
                    }
                }
                "PrereqAND" => {
                    for (internal_indice, child) in children.into_iter().enumerate() {
                        //let result = self.satisfy_requirements
                    }
                }
                "PrereqOR" => {}
                _ => {
                    panic!(
                        "This component has an invalid logic_type! {:?}",
                        self.reqs[requirement_indice]
                    )
                }
            }
        } else {
            //No logic type
            //Check this class's value
            if requirement.component.pftype.eq("Class") {
                return Ok(requirement.class.unwrap().credits.unwrap());
            }
            panic!(
                "Requirement is NOT a class and has a logic_type of NONE: {:?}",
                &requirement
            );
        }

        self.reqs[requirement_indice] = requirement;
        Ok(0)
    }

    //fn evaluate_prereqs(&mut self, )
}
