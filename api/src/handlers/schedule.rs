use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use std::collections::HashMap;

use crate::handlers::types::{
    Period, Req, Schedule, ScheduleError,
    Status::{self, Checked, Desirable, Selected, Unchecked, Unsuitable},
};
use crate::models::{
    associations::{ComponentToComponent, DegreeToComponent},
    class::Class,
    component::Component,
    degree::Degree,
};

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
            println!("{spacing}Error: parent (req_id: {parent_id}) doesn't exist, association failed.");
            return Err(ScheduleError::AssociationError);
        }
        if self.reqs.get_mut(&child_id).is_none() {
            println!("{spacing}Error: child (req_id: {child_id}) doesn't exist!, association failed.");
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

    pub fn build_schedule(&mut self) -> Result<Schedule, ScheduleError> {
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
        //return Ok(Schedule::new());
        //turn the requirements graph into a schedule
        self.analyze_requirements_graph(&mut req_holder)?;

        println!("\n\nanalyze_requirements_graph() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        req_holder.display_graph(root_id, &mut Vec::new());
        println!("------------------------------------------End Reqs------------------------------------------");
        
        //return Ok(Schedule::new());
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
        println!(
            "\n\nFor Degree {:?}\ncreate_schedule_from_queue() finished.",
            &self.degree.code
        );
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for item in &schedule.periods {
            println!("{item:?}");
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        println!("\n------------------------------------------------------------------------------------End build_schedule()------------------------------------------------------------------------------------");
        
        Ok(schedule)
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
            pftype: "Group".to_string(),
            class: None,
            logic_type: Some("AND".to_string()),
            children: Vec::new(),
            parents: Vec::new(),
            in_analysis: false,
        };

        let root_components = DegreeToComponent::get_components(&self.degree, &mut self.conn)?;

        req_holder.add_degree_req(degree_req);

        //TODO, FIX: 0: Req { component: Component { id: -1, name: "TEST MAJOR", pftype: "Degree", logic_type: Some("AND") }, class: None, children: [(0, Unchecked), (0, Unchecked)], parents: [] }
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

        println!("Now analyzing graph....");

        self.satisfy_requirements(req_holder, -1, true, 0)?;
        //since this root component has a logic type of AND, all of its requirements MUST
        //be fulfilled

        Ok(())
    }
    #[allow(unreachable_code)]
    fn satisfy_requirements(
        &mut self,
        req_holder: &mut ReqHolder,
        req_id: i32,
        required: bool,
        nests: usize,
    ) -> Result<i32, ScheduleError> {
        //println!("called satisfy_requirements");
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        let mut carried_result: Result<i32, ScheduleError> = Ok(0);
        println!(
            "\n{}Evaluating requirements of: \n{}{:?}",
            &spacing,
            &spacing,
            &req_holder.get_req(req_id).unwrap()
        );

        //This is necessary because we need to clone the req, and we can't clone a reference.
        if req_holder.get_req(req_id).unwrap().in_analysis {
            panic!("{}Already in analysis! {}", &spacing, &req_id);
        }

        //this ordering of these two lines is PARTICULAR. Do not mix these around.
        let mut req = req_holder.get_req(req_id).unwrap().clone();
        req_holder.get_req(req_id).unwrap().in_analysis = true;
        
        //Perform logic based on the req's type and logic type.
        let logic_type = req.logic_type.clone();
        if req.pftype.eq("Group") {
            //if it's a group, we can do a few things.
            if let Some(logic_type) = logic_type {
                match logic_type.as_str() {
                    "AND" => {
                        for child in &mut req.children {
                            
                            match self.satisfy_requirements(req_holder, req_id, required, nests) {
                                Ok(_) => {
                                    child.1 = Selected;

                                    //This parent also needs to be selected in this child's parents.
                                    for parent in &mut req_holder.get_req(child.0).unwrap().parents {
                                        if parent.0 == req_id {
                                            parent.1 = Selected;
                                        }
                                    }
                                }
                                Err(e) => {
                                    match e {
                                        ScheduleError::PrereqError => {
                                            //If a single child in this GroupAND has a prereqError, the whole
                                            //group must be thrown away.
                                            println!("{}This Group (req_id: {}) has a child (req_id: {}) with a PrereqError!",
                                            &spacing, req_id, child.0);
    
                                            child.1 = Unsuitable;
                                            carried_result = Err(e);
                                            break;
                                        }
                                        _ => panic!("GroupAND recieved a child with an invalid error!"),
                                    }
                                }
                            }
                        }
                    } //End GroupAND logic
                    "OR" => {
                        let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);

                        for (internal_indice, child) in req.children.iter_mut().enumerate() {
                            
                            match self.satisfy_requirements(req_holder, req_id, required, nests) {
                                Ok(result) => {
                                    minimal_cost = if result < minimal_cost.1 {
                                        (internal_indice, result)
                                    } else {
                                        minimal_cost
                                    };
    
                                    child.1 = Checked;
                                    //also check the parent
                                    for parent in &mut req_holder.get_req(child.0).unwrap().parents {
                                        if parent.0 == req_id {
                                            parent.1 = Checked;
                                            break;
                                        }
                                    }
                                }
                                Err(e) => {
                                    //This means that something was wrong with this particular requirement
                                    //and therefore should be labeled as unsuitable
    
                                    match e {
                                        ScheduleError::PrereqError => {
                                            //this means that this child is not suitable for use
                                            //however since this is OR logic, we can keep evaluating children.
                                            //unlike groupAND.
                                            child.1 = Unsuitable;
                                        }
                                        _ => panic!("GroupOR recieved a child with an invalid error!"),
                                    }
                                }
                            }
                        }
                    } //End GroupOR logic
                    _ => panic!("Invalid logic type for Group {:?}", req)
                }
            } else {
                panic!("This group has no logic type! {:?}", req);
            }

        } else if req.pftype.eq("Class") {
            
            if let Some(logic_type) = logic_type {
                match logic_type.as_str() {
                    "AND" => {
                        for child in &mut req.children {
                            
                            match self.satisfy_requirements(req_holder, req_id, required, nests) {
                                Ok(_) => {
                                    child.1 = Selected;

                                    //This parent also needs to be selected in this child's parents.
                                    for parent in &mut req_holder.get_req(child.0).unwrap().parents {
                                        if parent.0 == req_id {
                                            parent.1 = Selected;
                                        }
                                    }
                                }
                                Err(e) => {
                                    match e {
                                        ScheduleError::PrereqError => {
                                            //If a single child in this GroupAND has a prereqError, the whole
                                            //group must be thrown away.
                                            println!("{}This Group (req_id: {}) has a child (req_id: {}) with a PrereqError!",
                                            &spacing, req_id, child.0);
    
                                            child.1 = Unsuitable;
                                            carried_result = Err(e);
                                            break;
                                        }
                                        _ => panic!("GroupAND recieved a child with an invalid error!"),
                                    }
                                }
                            }
                        }
                    }
                    "OR" => {
                        let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);

                        'child: for (internal_indice, child) in req.children.iter_mut().enumerate() {
                            //We need to check the parent's of this child to see if
                            //it's part of other requirements and was overlooked.
                            let childrens_parents = req_holder.get_req(child.0).unwrap().parents.clone();

                            'child_parents: for parent in childrens_parents {
                                //we must make sure that this child's parent is not the current requirement
                                //Nor is another class. when that happens, this code will run and will satisfy
                                //any issues.
                                if parent.0 == req_id || req_holder.get_req(parent.0).unwrap().pftype.eq("Class") {
                                    continue 'child_parents;
                                }
                                //Now, let's take a simple example. Let's say this child is a class without
                                //prereqs. If it's selected by another parent/group, its cost will be zero.
                                if parent.1 == Selected {
                                    //If even one parent has this child as selected, we can say, alright.
                                    //break here. It's good enough. This may not necessarily be true, though.
                                    child.1 = Checked;
                                    minimal_cost = (internal_indice, 0);
                                    break 'child;
                                }
                            }
                        }

                        println!("{}Minimal Cost: {:?}", &spacing, &minimal_cost);
                        req.children[minimal_cost.0].1 = Selected;
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
                    _ => panic!("Invalid logic type for Class {:?}", req)
                }
            } else {
                panic!("This Class has no logic type! {:?}", req);
            }
        } else {
            panic!("This reqs pftype is currently unsupported! {:?}", req);
        }

        //No need to set in_analysis to false, because our clone never had in_analysis set to true.
        //req_holder.get_req(req_id).unwrap().in_analysis = false;
        *req_holder.get_req(req_id).unwrap() = req;

        println!(
            "{}Children of requirement fully evaluated: \n{}{:?}\n",
            &spacing,
            &spacing,
            &req_holder.get_req(req_id).unwrap()
        );
        carried_result
    }

    fn evaluate_prereq(
        &mut self,
        req_holder: &mut ReqHolder,
        req_id: i32,
        parent_required: bool,
        nests: usize,
    ) -> Result<i32, ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        println!("{}Evaluating prereq (req_id: {})", &spacing, req_id);
        //So we need to check its parents
        //This is because we cannot evaluate the prereq without its parents evaluating it alongside other components
        //in their children
        let mut updated_parents = req_holder.get_req(req_id).unwrap().parents.clone();

        //If this prereq isn't part of a larger component
        //And only if this prereq in question is a class
        //This doesn't work because this class could be part of a group not directly
        //tied to the major. Then what are ya gonna do?
        //Alternatively, what about classes tied to a GroupOR that's not part of the degree requirement? Well, that degree requirement should absolutely be part of the degree requirements, but for now it will not be evaluated. TODO.
        let this_pftype = req_holder.get_req(req_id).unwrap().pftype.clone();
        let this_logic_type = req_holder.get_req(req_id).unwrap().logic_type.clone();

        if this_pftype.eq("Class") && updated_parents
            .iter()
            .filter(|parent| req_holder.get_req(parent.0).unwrap().pftype.eq("Group"))
            .collect::<Vec<&(i32, Status)>>()
            .is_empty()
        {
            println!("{}Prereq (req_id: {}) has no group parent, as displayed below:\n{}{:?}\n\n{}Parents:",
                &spacing, 
                req_id, 
                &spacing, 
                &req_holder.get_req(req_id).unwrap(),
                &spacing
            );
            for parent in updated_parents {
                println!("{}{:?}", &spacing, req_holder.get_req(parent.0).unwrap());
            }
            return Err(ScheduleError::PrereqError);
        }

        //This kinda makes sense for a class, but not for a group passed
        //into this function. This will require a rewrite.

        //Let's take NETWORK ENGR GROUP 455 PREREQ
        //GroupOR, children are CNIT 34400 (in degree), and CNIT 345 (not in degree)
        if let Some(this_logic) = this_logic_type {
            //Don't execute the logic past this if statement, because that's for a class.
            //So, we need to evaluate two classes, one in the degree, and one that isn't. It's possible that 
            match this_logic.as_str() {
                _ => {}
            };
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
                    
                    &_ => {
                        panic!("This component has no pftype!");
                    }
                }
            } else {
                //No parent logic type...so it's a class without a prereq
            }
        }

        req_holder.get_req(req_id).unwrap().parents = updated_parents;
        println!(
            "{}Updated parents: {:?}",
            &spacing,
            &req_holder.get_req(req_id).unwrap().parents
        );
        println!("{}Children of the updated parents:", &spacing);
        for parent in req_holder.get_req(req_id).unwrap().parents.clone() {
            println!(
                "{}{}{:?}",
                &spacing,
                &extra_space,
                req_holder.get_req(parent.0)
            );
        }
        //TODO: this is -1 because it should not reach this point for now.
        Ok(-1)
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
        println!("{}Parent queue: {:?}", &spacing, &parent_queue);

        //Contains the indice in ReqHolder and it's queue number
        //let mut queue: Vec<(i32, i32)> = Vec::new();

        //check to see if this req is in the queue
        if let Some(i) = parent_queue.iter_mut().position(|item| item.0 == id) {
            //If it's in the queue, it should return a copy of itself as a vector
            let self_in_queue = parent_queue[i].clone();
            let mut self_vector: Vec<(i32, i32)> = Vec::new();
            self_vector.push(self_in_queue);
            println!(
                "{}this component is already in queue and is returning self as vector\n\n",
                &spacing
            );
            return Ok(self_vector);
        }

        //Otherwise, we just want classes.
        let current_req = req_holder.get_req(id).unwrap().clone();

        if let Some(_class) = current_req.class {
            //See if the class has selected children
            //So what are we doing here
            //If this is a class, we want to get its children.

            //So first, let's take ezclass 1.
            //No prereqs, so so we give it a 0
            //Then we look at ezclass 2.
            //No prereqs, so we give it a 0
            //Then we look at normalclass
            //Since this has children, we append the minimum queue no of its prereqs + 1 to it in the queue.
            println!("{}This component is a class\n\n", &spacing);

            let mut children_in_parent_queue = Vec::new();
            for child in current_req.children {
                if child.1 == Selected {
                    let mut result =
                        self.build_queue(req_holder, child.0, parent_queue, nests + 1)?;
                    children_in_parent_queue.append(&mut result);
                }
            }
            println!("{}This component (req_id: {}) has returned from evaluating children. The children in parent_queue are:\n{}{:?}\n{}With parent queue as\n{}{:?}", 
                &spacing, &id, &spacing, &children_in_parent_queue, &spacing, &spacing, &parent_queue);

            let mut max_queue_no = -1;
            //get the minimum queue number of the child
            for item in children_in_parent_queue.clone() {
                max_queue_no = if item.1 > max_queue_no {
                    item.1
                } else {
                    max_queue_no
                }
            }

            //Classes with no children will have -1 as the number.
            /*if max_queue_no == -1 {
                //If minimum_queue is STILL -1, then we push 0 to parent_queue.
                //new_queue will be discarded because if this is the case, do nothing with it
                parent_queue.push((id, 0));

                //Additionally, we will want to push a copy of itself to our return value
                let mut self_vector: Vec<(i32, i32)> = Vec::new();
                self_vector.push((id, 0));

                return Ok(self_vector);
            } else {*/
            parent_queue.push((id, max_queue_no + 1));
            //}

            println!("{}This queue (class): {:?}\n\n", &spacing, &parent_queue);
            return Ok(parent_queue.clone());
        };
        println!("{}This component (req_id: {}) is not a class.", &spacing, &id);
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
                Unsuitable => {}
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
