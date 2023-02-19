use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use std::collections::HashMap;
use thiserror::Error;

use crate::models::{
    associations::{ComponentToComponent, DegreeToComponent},
    class::Class,
    component::Component,
    degree::Degree,
};

#[derive(Debug, Clone)]
struct Req {
    id: i32,
    name: String,
    pftype: String,
    class: Option<Class>,
    logic_type: Option<String>,
    children: Vec<(i32, Status)>,
    parents: Vec<(i32, Status)>,
    in_analysis: bool,
}

impl Req {
    pub fn str(&self) -> String {
        format!(
            "{:12}: logic_type: {:60?}, children:{:?}, parents: {:?}",
            self.name, self.logic_type, self.children, self.parents
        )
    }
}

#[derive(Debug, Clone)]
struct ReqHolder {
    reqs: HashMap<i32, Req>,
}

impl ReqHolder {
    fn new() -> ReqHolder {
        ReqHolder {
            reqs: HashMap::new(),
        }
    }
    fn add_degree_req(&mut self, degree: Req) {
        self.reqs.insert(degree.id, degree);
    }
    fn add_component(
        &mut self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        component: Component,
        spacing: &String,
    ) {
        let class = if component.pftype.eq("Class") {
            Some(
                Class::find_by_component_id(&component.id, conn)
                    .ok()
                    .unwrap(),
            )
        } else {
            None
        };

        self.reqs.insert(
            component.id,
            Req {
                id: component.id,
                name: component.name,
                pftype: component.pftype,
                class,
                logic_type: component.logic_type,
                children: Vec::new(),
                parents: Vec::new(),
                in_analysis: false,
            },
        );
        println!(
            "{}Created new requirement (req_id: {})",
            spacing, component.id
        );
    }

    fn try_add_association(
        &mut self,
        spacing: &str,
        parent_id: i32,
        child_id: i32,
    ) -> Result<(), ScheduleError> {
        //This function SHOULD ONLY BE USED when checking if the child exists.
        //Ideally, the parent will already have existed.

        if self.reqs.get_mut(&parent_id).is_none() {
            println!("{spacing}Error: parent (req_id: {parent_id}) doesn't exist!");
            return Err(ScheduleError::AssociationError);
        }
        if self.reqs.get_mut(&child_id).is_none() {
            println!("{spacing}Error: child (req_id: {child_id}) doesn't exist!");
            return Err(ScheduleError::AssociationError);
        }

        if let Some(child_req) = self.get_req(child_id) {
            println!("{spacing}Gave child (req_id: {child_id}) a parent (req_id: {parent_id})");
            child_req.parents.push((parent_id, Unchecked));
        }

        if let Some(parent_req) = self.get_req(parent_id) {
            println!("{spacing}Gave parent (req_id: {parent_id}) a child(req_id: {child_id})");
            parent_req.children.push((child_id, Unchecked));
        }

        Ok(())
    }

    fn get_req(&mut self, id: i32) -> Option<&mut Req> {
        self.reqs.get_mut(&id)
    }

    fn display_graph(&self, id: i32, displayed_reqs: &mut Vec<i32>) {
        if !displayed_reqs
            .iter_mut()
            .any(|displayed_id| *displayed_id == id)
        {
            displayed_reqs.push(id);
            let req = self.reqs.get(&id).unwrap();
            println!("{req:?}");
            for child in &req.children {
                self.display_graph(child.0, displayed_reqs);
            }
        }
    }
}

struct Cost<'a> {
    index: usize,
    //An array that follows a path of components to satisfy and cost
    path_cost: Vec<(Vec<&'a Req>, i32)>,
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Unchecked,
    Checked,
    //Unsuitable,
    Desirable,
    Selected,
}

use Status::{Checked, Desirable, Selected, Unchecked};

#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Diesel Error")]
    DieselError(#[from] diesel::result::Error),

    #[error("Component not found")]
    AssociationError,

    #[error("Prereq is invalid for this Degree")]
    PrereqError,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Period {
    year: u32,
    time: String,
    classes: Vec<Req>,
}
/**
 * Uses an adjacent array to build a graph via the Req struct.
 */
pub struct ScheduleMaker {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    pub degree: Degree,
    schedule: Option<Schedule>,
}

impl ScheduleMaker {
    pub fn new(
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
        degree_code: &str,
    ) -> Result<Self, ScheduleError> {
        let degree = Degree::find_by_code(degree_code, &mut conn)?;

        Ok(Self {
            conn,
            degree,
            schedule: None,
        })
    }

    pub fn build_schedule(&mut self) -> Result<String, ScheduleError> {
        //get the degree root components
        //all of these components must be satisfied for the schedule

        //This builds our graph in an adjacency matrix stores in self.reqs
        //Note that the degree itself is modeled into a fake req
        //This degree root is at self.reqs[0]
        println!("------------------------------------------------------------------------------------Begin build_schedule()------------------------------------------------------------------------------------\n");
        let mut req_holder = ReqHolder::new();
        let root_id = self.build_requirements_graph(&mut req_holder)?;

        println!("\n\nbuild_requirements_graph() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        //TODO, show requirements graph based on degree
        req_holder.display_graph(root_id, &mut Vec::new());
        println!("------------------------------------------End Reqs------------------------------------------");

        //turn the requirements graph into a schedule
        self.analyze_requirements_graph(&mut req_holder)?;

        println!("\n\nanalyze_requirements_graph() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        req_holder.display_graph(root_id, &mut Vec::new());
        println!("------------------------------------------End Reqs------------------------------------------");

        /*println!("\n\nLong print.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for (pos, req) in self.reqs.iter().enumerate() {
            println!("{:>2}: {:?}", pos, req);
        }
        println!("------------------------------------------End Reqs------------------------------------------");
        */

        let queue = self.build_queue(&mut req_holder, root_id, &mut Vec::new(), 0)?;
        println!("\n\nbuild_queue() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for item in &queue {
            println!("{item:?}");
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        //From the queue, build a schedule
        let schedule = self.create_schedule_from_queue(&mut req_holder, queue);

        println!("\n\ncreate_schedule_from_queue() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for item in &schedule.periods {
            println!("{item:?}");
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        println!("\n------------------------------------------------------------------------------------End build_schedule()------------------------------------------------------------------------------------");
        Ok(String::from("Success!"))
    }

    /**
     * This function will display a full tree of every root component, and every component
     * which satisifes its conditions.
     *
     */
    fn build_requirements_graph(
        &mut self,
        req_holder: &mut ReqHolder,
    ) -> Result<i32, ScheduleError> {
        //build a root node for the degree
        //TODO: this is extremely poor practice...notably giving it an id of -1.
        let root_id = -1;
        let degree_req = Req {
            id: root_id,
            name: self.degree.name.to_string(),
            pftype: "Degree".to_string(),
            class: None,
            logic_type: Some("GroupAND".to_string()),
            children: Vec::new(),
            parents: Vec::new(),
            in_analysis: false,
        };

        let root_components = DegreeToComponent::get_components(&self.degree, &mut self.conn)?;

        req_holder.add_degree_req(degree_req);

        //TODO, FIX: 0: Req { component: Component { id: -1, name: "TEST MAJOR", pftype: "Degree", logic_type: Some("GroupAND") }, class: None, children: [(0, Unchecked), (0, Unchecked)], parents: [] }
        println!("Root component: {:?}", &req_holder.get_req(-1));

        self.associate_components(req_holder, -1, root_components, 0)?;

        Ok(root_id)
    }

    fn associate_components(
        &mut self,
        req_holder: &mut ReqHolder,
        parent_id: i32,
        components: Vec<Component>,
        nests: usize,
    ) -> Result<(), ScheduleError> {
        for component in components {
            let spaces = 4 * nests;
            let spacing = (0..=spaces).map(|_| " ").collect::<String>();
            let extra_space = (0..=4).map(|_| " ").collect::<String>();
            println!("{}Component: {:?}", &spacing, &component);

            //Determine if this component is already in reqs
            println!(
                "{}-----------------START COMPONENT (req_id: {})-----------------",
                &spacing, component.id
            );

            match req_holder.try_add_association(&spacing, parent_id, component.id) {
                Ok(_) => {
                    //done, maybe remove soon
                    println!("{} ALREADY EXISTS!", &spacing);
                }
                Err(_) => {
                    //not done, so create a new component and associate
                    //get the ID of this component
                    let id = component.id;
                    //push this component to the reqs
                    req_holder.add_component(&mut self.conn, component, &spacing);
                    req_holder.try_add_association(&spacing, parent_id, id)?;

                    //get the children of this component
                    let children = ComponentToComponent::get_children(&id, &mut self.conn)?;
                    println!(
                        "{}Grabbed new children of component (req_id: {}):\n{}{}{:?}\n\n",
                        &spacing, id, spacing, extra_space, &children
                    );

                    //recursively call this function
                    self.associate_components(req_holder, id, children, nests + 1)?;
                }
            };
        }
        Ok(())
    }

    /**
     * This function should only be called once the graph is made in build_requirements_graph()
     */
    fn analyze_requirements_graph(
        &mut self,
        req_holder: &mut ReqHolder,
    ) -> Result<(), ScheduleError> {
        //let schedule = Schedule::new();

        println!("Now generating schedule....");

        self.satisfy_requirements(req_holder, -1, Some(String::from("GroupAND")), true, 0)?;
        //since this root component has a logic type of GroupAND, all of its requirements MUST
        //be fulfilled

        Ok(())
    }
    #[allow(unreachable_code)]
    fn satisfy_requirements(
        &mut self,
        req_holder: &mut ReqHolder,
        req_id: i32,
        logic_type: Option<String>,
        required: bool,
        nests: usize,
    ) -> Result<i32, ScheduleError> {
        //println!("called satisfy_requirements");
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();

        println!(
            "\n{}Satisfying Requirements of: \n{}{:?}",
            &spacing,
            &spacing,
            &req_holder.get_req(req_id).unwrap()
        );
        //borrow checker doesn't like that I'm mutating its memory and also calling it.
        //The easiest way to fix this is to make a clone of it and then set the requirement to that
        //clone after manipulation. This will decrease performance but is guaranteed to be safe
        let mut updated_children = req_holder.get_req(req_id).unwrap().children.clone();
        req_holder.get_req(req_id).unwrap().in_analysis = true;
        //There is no real other way to do this, because it gets made that I even try

        let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);
        //let mut internal_indice: usize = 0;
        let mut can_associate = true;
        for (internal_indice, child) in updated_children.iter_mut().enumerate() {
            let child_logic_type = req_holder.get_req(child.0).unwrap().logic_type.clone();

            if let Some(logic_type) = &logic_type {
                match logic_type.as_str() {
                    "GroupAND" => {
                        self.satisfy_requirements(
                            req_holder,
                            child.0,
                            child_logic_type,
                            required,
                            nests + 1,
                            
                        )?;
                        child.1 = Selected;
                        for parent in &mut req_holder.get_req(child.0).unwrap().parents {
                            if parent.0 == req_id {
                                parent.1 = Selected;
                                break;
                            }
                        }
                    }
                    "GroupOR" => {
                        //println!("Internal Indice = {}", internal_indice);
                        let result = self.satisfy_requirements(
                            req_holder,
                            child.0,
                            child_logic_type,
                            false,
                            nests + 1,
                        )?;
                        minimal_cost = if result < minimal_cost.1 {
                            (internal_indice, result)
                        } else {
                            minimal_cost
                        };
                        /*println!(
                            "Minimal cost: {:?}, result for req_id {}: {}",
                            minimal_cost, child.0, result
                        );*/

                        child.1 = Checked;
                        //also check the parent
                        for parent in &mut req_holder.get_req(child.0).unwrap().parents {
                            if parent.0 == req_id {
                                parent.1 = Checked;
                                break;
                            }
                        }
                    }
                    "PrereqAND" => {
                        match self.evaluate_prereq(req_holder, child.0, required, nests + 1) {
                            Ok(_) => {
                                //If all went well, then these should all be checked.
                                child.1 = Selected
                            }
                            Err(e) => {
                                //If this has returned an error, this means that a better situation has been identified
                                //Or this means that the algorithm is naive and has no clue that this option could be
                                //potentially better. This will require more insight as tests are developed for the
                                //algorithm.
                                println!("{}Error evaluating prereq!: {e:?}", &spacing);
                                can_associate = false;
                            }
                        }
                    }
                    "PrereqOR" => {}
                    _ => {}
                }
            }
        }

        //After evaluating all the children
        println!("{}Children of requirement (req_id: {}) partially evaluated: \n{}With updated children of:\n{}{:?}\n\n", 
            &spacing,
            &req_id,
            &spacing,
            &spacing,
            &updated_children
        );
        if let Some(logic_type) = logic_type {
            //println!("Logic type: {}", &logic_type);
            match logic_type.as_str() {
                "GroupAND" => {}
                "GroupOR" => {
                    println!("{}Minimal Cost: {:?}", &spacing, &minimal_cost);
                    updated_children[minimal_cost.0].1 = Selected;
                    //req_holder.get_req(req_id).unwrap().children[minimal_cost.0].1 =
                    //    CheckedAndSelected;

                    //Also, on the child, make sure to add checkedandselected to its parent
                    //Note that this child should not be an active part of this recursion.
                    //Otherwise, this will get overwritten once updated_children is called at the bottom..
                    let child_id_in_req_holder =
                        req_holder.get_req(req_id).unwrap().children[minimal_cost.0].0;

                    //Note that we aren't copying this and setting the value to it. We are
                    //going straight to the value in self.reqs and changing it.

                    for parent in &mut req_holder.get_req(child_id_in_req_holder).unwrap().parents {
                        if parent.0 == req_id {
                            parent.1 = Selected;
                            break;
                        }
                    }
                }
                "PrereqAND" => {
                    if can_associate {
                        for child in &mut updated_children {
                            //give each child for this component a value of CheckedAndSelected
                            child.1 = Selected;
                            for parent in &mut req_holder.get_req(child.0).unwrap().parents {
                                if parent.0 == req_id {
                                    parent.1 = Selected;
                                }
                            }
                        }
                        if req_holder.get_req(req_id).unwrap().pftype.eq("Class") {
                            req_holder.get_req(req_id).unwrap().children = updated_children;
                            req_holder.get_req(req_id).unwrap().in_analysis = false;
                            return Ok(req_holder
                                .get_req(req_id)
                                .unwrap()
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
                        req_holder.get_req(req_id).unwrap()
                    )
                }
            }
        } else {
            //No logic type
            //Check this class's value
            if req_holder.get_req(req_id).unwrap().pftype.eq("Class") {
                req_holder.get_req(req_id).unwrap().children = updated_children;
                req_holder.get_req(req_id).unwrap().in_analysis = false;
                return Ok(req_holder
                    .get_req(req_id)
                    .unwrap()
                    .class
                    .as_ref()
                    .unwrap()
                    .credits
                    .unwrap());
            }
            panic!(
                "{}Requirement is NOT a class and has a logic_type of NONE: {:?}",
                &spacing,
                &req_holder.get_req(req_id).unwrap()
            );
        }
        println!(
            "{}Children of requirement fully evaluated: \n{}{:?}\n{}With updated children of:\n{}{:?}\n\n",
            &spacing,
            &spacing,
            &req_holder.get_req(req_id).unwrap(),
            &spacing,
            &spacing,
            &updated_children
        );
        //finally update the req's children
        req_holder.get_req(req_id).unwrap().children = updated_children;
        req_holder.get_req(req_id).unwrap().in_analysis = false;
        println!(
            "{}Requirement after satisfaction: \n{}{:?}\n",
            &spacing,
            &spacing,
            &req_holder.get_req(req_id).unwrap()
        );

        Ok(0)
    }

    fn evaluate_prereq(
        &mut self,
        req_holder: &mut ReqHolder,
        req_id: i32,
        parent_required: bool,
        nests: usize,
    ) -> Result<String, ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        println!("{}Evaluating prereq (req_id: {})", &spacing, req_id);
        //So we need to check its parents
        //This is because we cannot evaluate the prereq without its parents evaluating it alongside other components
        //in their children
        let mut updated_parents = req_holder.get_req(req_id).unwrap().parents.clone();
        
        if updated_parents
            .iter()
            .filter(|parent| req_holder.get_req(req_id).unwrap().pftype == "Group")
            .collect::<Vec<&(i32, Status)>>()
            .is_empty() {
            return Err(ScheduleError::PrereqError);
        }
        for parent in &mut updated_parents {
            let parent_ref = req_holder.get_req(parent.0).unwrap();
            let parent_type = &parent_ref.logic_type;
            println!(
                "{}Evaluating parent (req_id: {}) of prereq",
                &spacing, parent.0
            );

            if let Some(parent_logic_type) = parent_type {
                //Since we need to satisfy this prereq, we should go ahead and test out
                //this group
                //TODO: This feels like this could potentially infinitely recurse.
                match parent_logic_type.as_str() {
                    "GroupAND" => {
                        match parent.1 {
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
                            Checked => {
                                //If this component has a parent value of checked
                                //This mean it has run through logic already.
                                //In this case, we want our parent to reconsider
                                for child in parent_ref.children.iter_mut() {
                                    //Now we re-evalute each child's status of the parent.
                                    //This is where we get our reasoning.
                                    if child.0 != req_id {
                                        match child.1 {
                                            Unchecked => panic!("Parent's child not checked, even though this child has been!"),
                                            Checked => {
                                                //something
                                            }
                                            Desirable => {
                                                //something

                                            }
                                            Selected => {
                                                //something
                                                
                                            }
                                        }
                                    }
                                    
                                }
                            }
                            Selected => {}
                            Desirable => {}
                        }
                    }
                    "GroupOR" => {
                        match parent.1 {
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
                            Checked => {
                                //If this component has a parent value of checked
                                //This mean it has run through logic already.
                                //In this case, we want our parent to reconsider
                                for child in parent_ref.children.iter_mut() {
                                    //Now we re-evalute each child's status of the parent.
                                    //This is where we get our reasoning.
                                    if child.0 != req_id {
                                        match child.1 {
                                            Unchecked => panic!("Parent's child not checked, even though this child has been!"),
                                            Checked => {
                                                //Here we do nothing because this child is irrelevant to us
                                            }
                                            Desirable => {
                                                //something
                                                //Here we also don't do anything yet...but I imagine this will change
                                                //and the entire parent will have to be re-evaluated
                                            }
                                            Selected => {
                                                //this is where it gets interesting. A class that doesn't fulfill our requirement
                                                //is selected by a parent with a groupOR value. 

                                                //So if the parent MUST be included in the degree requirement, this selected value
                                                //must become checked.
                                                if parent_required {
                                                    child.1 = Checked;
                                                }
                                                
                                            }
                                        }
                                    }
                                    if child.0 == req_id && parent_required {
                                        child.1 = Selected;
                                    }
                                    
                                }
                            }
                            Selected => {}
                            Desirable => {}
                        }
                    }
                    "PrereqAND" | "PrereqOR" => {
                        //Do nothing. This could be the original caller of this evaluate (its children)
                        println!("{}Req (req_id: {}) is a class and is not being evaluated yet!", &spacing, &req_id);
                        //return Ok(String::from("Not tested"))
                    },
                    
                    /*match parent.1 {
                        panic!("PrereqOR has not yet been implemented for evaluating prereqs!");
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
                        Selected => {
                            //This means we're all good to use this prereq and should return ok
                            req_holder.get_req(req_id).unwrap().parents = updated_parents;
                            return Ok(String::from("Prereq Checked and Selected"));
                        }
                        Desirable => {}
                    },*/
                    &_ => {
                        panic!("This component has no pftype!");
                    }
                }
            } else {
                //No parent logic type...so it's a class without a prereq
            }
        }

        req_holder.get_req(req_id).unwrap().parents = updated_parents;
        println!("{}Updated parents: {:?}", &spacing, &req_holder.get_req(req_id).unwrap().parents);
        println!("{}Children of the updated parents:", &spacing);
        for parent in req_holder.get_req(req_id).unwrap().parents.clone() {
            println!("{}{}{:?}", &spacing, &extra_space, req_holder.get_req(parent.0));
        }
        //TODO: remove
        Ok(String::from("Finished"))
    }

    fn build_queue(
        &mut self,
        req_holder: &mut ReqHolder,
        id: i32,
        parent_queue: &mut Vec<(i32, i32)>,
        nests: usize,
    ) -> Result<Vec<(i32, i32)>, ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        //contains (requirement in the graph, priority #)
        println!("{}Now evaluating {:?}", &spacing, &req_holder.get_req(id));
        println!("{}Parent queue: {:?}\n\n", &spacing, &parent_queue);

        //Contains the indice in ReqHolder and it's queue number
        //let mut queue: Vec<(i32, i32)> = Vec::new();

        //check to see if this req is in the queue
        if let Some(i) = parent_queue.iter_mut().position(|item| item.0 == id) {
            return Ok(Vec::new());
        }

        //Otherwise, we just want classes.
        let current_req = req_holder.get_req(id).unwrap().clone();

        if let Some(_class) = current_req.class {
            //See if the class has selected children

            for child in current_req.children {
                if child.1 == Selected {
                    let mut _result =
                        self.build_queue(req_holder, child.0, parent_queue, nests + 1)?;
                    //parent_queue.append(&mut result);
                }
            }

            let mut minimum_queue_no = i32::MAX;
            for item in parent_queue.clone() {
                minimum_queue_no = if item.1 < minimum_queue_no {
                    item.1
                } else {
                    minimum_queue_no
                }
            }

            if minimum_queue_no == i32::MAX {
                //there is no children of this
                parent_queue.push((id, 0));
            } else {
                parent_queue.push((id, minimum_queue_no + 1));
            }

            println!("{}This queue (class): {:?}\n\n", &spacing, &parent_queue);
            return Ok(parent_queue.clone());
        };

        //IF THIS IS NOT A CLASS
        for child in current_req.children {
            if child.1 == Selected {
                let mut _new_queue =
                    self.build_queue(req_holder, child.0, parent_queue, nests + 1)?;
            }
        }
        println!("{}This queue (group): {:?}\n\n", &spacing, &parent_queue);
        Ok(parent_queue.clone())
    }
    fn check_to_add(
        &mut self,
        req_holder: &mut ReqHolder,
        queue: &mut [(usize, i32)],
        req_id: i32,
    ) {
        //Since we know the classes to be added, this should be relatively easy.
        //this is a simple graph pruning problem
        let requirement = req_holder.get_req(req_id).unwrap();

        for child in &mut requirement.children {
            match child.1 {
                Selected => match requirement.pftype.as_str() {
                    "Group" => {}
                    "Class" => {}
                    _ => {}
                },
                Checked => {}
                Unchecked => {
                    panic!("Something wasn't checked before building the queue!!")
                }
                Desirable => {}
            }
        }
    }

    fn create_schedule_from_queue(
        &mut self,
        req_holder: &mut ReqHolder,
        queue: Vec<(i32, i32)>,
    ) -> Schedule {
        //So, in order to build this schedule, we need to match queue numbers
        //and create periods of time based on those queue numbers.
        //So, we know that the minimum queue number will be 0.
        //Let's iteratively work through this
        let mut checkable_queue = queue
            .into_iter()
            .map(|item| (item.0, item.1, true))
            .collect::<Vec<(i32, i32, bool)>>();

        let mut max_queue_no = 0;

        let mut schedule = Schedule::new();

        let mut time_counter = 0;

        while !checkable_queue.is_empty() {
            let year = 2023 + (time_counter / 2);

            let time = if time_counter % 2 == 0 {
                "Spring".to_owned()
            } else {
                "Fall".to_owned()
            };

            let mut new_period = Period {
                year,
                time,
                classes: Vec::new(),
            };

            for item in &mut checkable_queue {
                if item.1 <= max_queue_no {
                    new_period
                        .classes
                        .push(req_holder.get_req(item.0).unwrap().clone());
                    item.2 = false;
                }
            }

            //remove all the items whose value is false.
            checkable_queue.retain(|item| item.2);

            //Increase the queue number by 1
            max_queue_no += 1;

            //increase the time_counter by 1
            time_counter += 1;

            //Finally, push this new period into our schedule
            schedule.periods.push(new_period);
        }

        //self.generate_periods(req_holder, &mut queue, &mut schedule, 0);

        schedule
    }
}
