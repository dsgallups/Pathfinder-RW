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
    pub fn str(&self) -> String {
        format!(
            "{:12}: logic_type: {:60?}, children:{:?}, parents: {:?}",
            self.component.name, self.component.logic_type, self.children, self.parents
        )
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
            println!("{:>2}: {}", pos, req.str().as_str());
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        /*println!("\n\nLong print.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for (pos, req) in self.reqs.iter().enumerate() {
            println!("{:>2}: {:?}", pos, req);
        }
        println!("------------------------------------------End Reqs------------------------------------------");
        */

        self.build_queue()?;
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

        self.satisfy_requirements(0)?;
        //since this root component has a logic type of GroupAND, all of its requirements MUST
        //be fulfilled

        Ok(())
    }

    fn satisfy_requirements(&mut self, requirement_indice: usize) -> Result<i32, ScheduleError> {
        //println!("called satisfy_requirements");
        //borrow checker doesn't like that I'm mutating its memory and also calling it.
        //The easiest way to fix this is to make a clone of it and then set the requirement to that
        //clone after manipulation. This will decrease performance but is guaranteed to be safe

        //let mut requirement = &mut self.reqs[requirement_indice];

        if let Some(logic_type) = &mut self.reqs[requirement_indice].component.logic_type {
            let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);

            match logic_type.as_str() {
                "GroupAND" => {
                    //Since all of the degrees in our catalog is GroupAND

                    //So we want to make sure that the current requirement is satisfied
                    //To do this, we need to store a value that tells us if the
                    //requirement is selected.

                    //Make sure that we get a return value that will tell us if it is checked
                    for child in &mut self.reqs[requirement_indice].children {
                        self.satisfy_requirements(child.0)?;

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
                    let mut internal_indice: usize = 0;

                    for mut child in &mut self.reqs[requirement_indice].children {
                        let result = self.satisfy_requirements(child.0)?;
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
                        internal_indice += 1;
                    }

                    //now set the selected indice to checkedandselected
                    //I wonder if we should instead provide it a path cost....hmm.
                    /*
                       Where the requirement would have something like
                       (MA 16010, 5, Required),
                       (MA 162, 3, Best)
                    */
                    self.reqs[requirement_indice].children[minimal_cost.0].1 = CheckedAndSelected;

                    //Also, on the child, make sure to add checkedandselected to its parent
                    let child_indice_in_reqs =
                        self.reqs[requirement_indice].children[minimal_cost.0].0;

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
                    //Return Ok ONLY IF every prereq is satisfied here.
                    let mut can_associate = true;
                    for child in &mut self.reqs[requirement_indice].children {
                        match self.evaluate_prereq(child.0) {
                            Ok(res) => {}
                            Err(e) => {
                                //If this has returned an error, this means that a better situation has been identified
                                //Or this means that the algorithm is naive and has no clue that this option could be
                                //potentially better. This will require more insight as tests are developed for the
                                //algorithm.
                                can_associate = false;
                            }
                        }
                    }

                    /*
                       TEST1 specific debugging
                    */

                    if requirement_indice == 6 {
                        println!(
                            "\n\nEVALUATING {:?}\nCAN ASSOCIATE: {:?}\n",
                            &self.reqs[requirement_indice], can_associate
                        );
                    }

                    if can_associate {
                        for child in &mut self.reqs[requirement_indice].children {
                            //give each child for this component a value of CheckedAndSelected
                            child.1 = CheckedAndSelected;
                            for parent in &mut self.reqs[child.0].parents {
                                if parent.0 == requirement_indice {
                                    parent.1 = CheckedAndSelected;
                                }
                            }
                        }
                        if self.reqs[requirement_indice].component.pftype.eq("Class") {
                            return Ok(self.reqs[requirement_indice]
                                .class
                                .as_ref()
                                .unwrap()
                                .credits
                                .unwrap());
                        }
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
            if self.reqs[requirement_indice].component.pftype.eq("Class") {
                return Ok(self.reqs[requirement_indice]
                    .class
                    .as_ref()
                    .unwrap()
                    .credits
                    .unwrap());
            }
            panic!(
                "Requirement is NOT a class and has a logic_type of NONE: {:?}",
                &self.reqs[requirement_indice]
            );
        }

        Ok(0)
    }

    fn evaluate_prereq(&mut self, requirement_indice: usize) -> Result<String, ScheduleError> {
        let requirement = self.reqs[requirement_indice].clone();

        //So we need to check its parents
        //This is because we cannot evaluate the prereq without its parents evaluating it alongside other components
        //in their children
        for parent in &requirement.parents {
            match self.reqs[parent.0].component.pftype.as_str() {
                //Since we need to satisfy this prereq, we should go ahead and test out
                //this group
                //TODO: This feels like this could potentially infinitely recurse.
                "Group" => match parent.1 {
                    Unchecked => {
                        //Depending on the type of parent (group or class)
                        //We need to perform different actions
                        /*
                           Lets says MA 16010 has an unchecked parent of MA 16020
                           The problem is that MA 16020 probably called this. So if
                           we try to run the parent through the original function,
                           this program will infinitely loop

                           So, only do logic on unchecked groups
                        */
                        //self.satisfy_requirements(parent.0);
                    }
                    Checked => {}
                    CheckedAndSelected => {}
                },
                "Class" => match parent.1 {
                    //this could potentially result in a infinite loop
                    //self.evaluate_prereq(parent.0);
                    Unchecked => {}
                    Checked => {
                        //This implies that it was not initially selected,
                        //or that it, at one point, was selected and is no longer selected
                        //This means it should be re-evaluated based on what the dependencies
                        //of this prereq are.
                        //TODO
                    }
                    CheckedAndSelected => {
                        //This means we're all good to use this prereq and should return ok
                        return Ok(String::from("Prereq Checked and Selected"));
                    }
                },
                &_ => {
                    panic!("This component has no pftype!");
                }
            }
        }

        self.reqs[requirement_indice] = requirement;

        //TODO: remove
        Ok(String::from("Finished"))
    }

    pub fn build_queue(&mut self) -> Result<Vec<(usize, i32)>, ScheduleError> {
        //contains (requirement in the graph, priority #)

        let mut queue: Vec<(usize, i32)> = Vec::new();

        //Now that the requirements have been completely analyzed, we should see
        //if it's possible to build a queue for a schedule
        /*
           Something like
           { Component (which is a class), priority #}
        */

        //add things to queue based on their prio
        self.check_to_add(0, &mut queue);

        Ok(queue)
    }
    pub fn check_to_add(&mut self, index: usize, queue: &mut Vec<(usize, i32)>) {
        //Since we know the classes to be added, this should be relatively easy.
        //this is a simple graph pruning problem

        for child in &self.reqs[index].children {
            match child.1 {
                CheckedAndSelected => match self.reqs[child.0].component.pftype.as_str() {
                    "Group" => {}
                    "Class" => {}
                    _ => {}
                },
                Checked => {}
                Unchecked => {
                    panic!("Something wasn't checked before building the queue!!")
                }
            }
        }
    }
}
