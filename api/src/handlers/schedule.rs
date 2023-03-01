use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

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
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

        if self.get_req(parent_id).is_none() {
            println!(
                "{spacing}Error: parent (req_id: {parent_id}) doesn't exist, association failed."
            );
            return Err(ScheduleError::AssociationError);
        }
        if self.get_req(child_id).is_none() {
            println!(
                "{spacing}Error: child (req_id: {child_id}) doesn't exist!, association failed."
            );
            return Err(ScheduleError::AssociationError);
        }

        if let Some(child_req) = self.get_req(child_id) {
            child_req.parents.push((parent_id, Unchecked));
            println!("{spacing}Gave child (req_id: {child_id}) a parent (req_id: {parent_id})");
            println!("child = {:?}", child_req);
            //child_req.parents.push((parent_id, Unchecked));
        }

        if let Some(parent_req) = self.get_req(parent_id) {
            parent_req.children.push((child_id, Unchecked, None));
            println!("{spacing}Gave parent (req_id: {parent_id}) a child(req_id: {child_id})");
            println!("parent = {:?}", parent_req);
        }

        Ok(())
    }

    fn get_req(&mut self, id: i32) -> Option<&mut Req> {
        match self.reqs.get_mut(&id) {
            Some(req) => return Some(req),
            None => {
                return None;
            }
        };
    }

    fn display_graph(&mut self, id: i32, displayed_reqs: &mut Vec<i32>) {
        if !displayed_reqs
            .iter_mut()
            .any(|displayed_id| *displayed_id == id)
        {
            displayed_reqs.push(id);

            let req_children = {
                let req = self.get_req(id).unwrap();
                println!("{req:?}");
                req.children.clone()
            };

            for child in &req_children {
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

        /*let queue = self.build_queue(&mut req_holder, root_id, &mut Vec::new(), 0)?;
        println!("\n\nbuild_queue() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        for item in &queue {
            println!("{item:?}");
        }
        println!("------------------------------------------End Reqs------------------------------------------");

        //From the queue, build a schedule
        //let schedule = self.create_schedule_from_queue(&mut req_holder, queue);
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
        */
        Ok(Schedule::new())
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

    //This function should only be called once the graph is made in build_requirements_graph()

    fn analyze_requirements_graph(
        &mut self,
        req_holder: &mut ReqHolder,
    ) -> Result<(), ScheduleError> {
        //let schedule = Schedule::new();

        println!("Now analyzing graph....");
        let root_req_id = -1;
        self.satisfy_requirements(req_holder, None, root_req_id, 0)?;
        //since this root component has a logic type of AND, all of its requirements MUST
        //be fulfilled

        Ok(())
    }

    /*
        This function satisfies the requirements of the given req.
        Note that THIS FUNCTION SHOULD ONLY MUTATE THE REQ, NOT ITS CHILDREN OR PARENTS.
    */
    #[allow(unreachable_code)]
    fn satisfy_requirements(
        &mut self,
        req_holder: &mut ReqHolder,
        parent_req_id: Option<i32>,
        req_id: i32,
        nests: usize,
    ) -> Result<(Option<i32>, Status), ScheduleError> {
        //println!("called satisfy_requirements");
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();

        let (logic_type, pftype) = {
            let req = req_holder.get_req(req_id).unwrap();
            req.in_analysis = true;
            println!(
                "\n{}Evaluating requirements of: \n{}{:?}",
                &spacing, &spacing, &req
            );

            (req.logic_type.clone(), req.pftype.clone())
        };

        //The root component will say it is in analysis, but the cloned will not.
        //This probably won't matter. But might matter in the future.
        /*if req.borrow().in_analysis {
            panic!("{}Already in analysis! {:?}", &spacing, &req);
        }*/

        let parent_req_id = match parent_req_id {
            Some(id) => id,
            None => {
                //This should only occur for the root component.
                if req_id != -1 {
                    panic!("{}No parent req id! {:?}", &spacing, &req_id);
                }
                //Implement recursive solution to the root id
                return Err(ScheduleError::UnimiplementedLogicError);
            }
        };

        //Perform logic based on the req's type and logic type.

        /*
           There are commonalities among all the situations, which are
               GROUP AND, GROUP ARE, CLASS AND, CLASS OR, CLASS NONE
           So we can do some common things first, and then do the specific things
           - we look at the children of the req. Additionally, we mutate the values of req's children.
           - we return our cost and status to our parent, which called this function.
               - Errors are dealt with per recursive call, so the only error passed up the tree
                 should be one where the req in the call had an error

           - The following operatons
           for (internal_indice, child) in req.children.iter_mut().enumerate() {
           }

        */
        if let None = logic_type {
            match pftype.as_str() {
                "Class" => {
                    println!(
                        "{}Class (req_id: {}) has no logic type. Returning credits",
                        &spacing, req_id
                    );
                    let status = Status::Checked;
                    self.modify_parent_status(
                        req_holder,
                        status.clone(),
                        parent_req_id,
                        req_id,
                        nests,
                    );

                    let req = req_holder.get_req(req_id).unwrap();
                    let credits = req.class.as_ref().unwrap().credits.unwrap();
                    return Ok((Some(credits), status));
                }
                _ => {
                    panic!("This reqs pftype is currently unsupported! {:?}", req_id);
                }
            }
        }
        let logic_type = logic_type.unwrap();

        if pftype.eq("Group") {
            //if it's a group, we can do a few things.
            let req_children = self.evaluate_children(&pftype, req_holder, req_id, nests);

            match logic_type.as_str() {
                "AND" => {
                    //After cycling the children, and no error performed, return an Ok to the parent.
                    //For now, we will set the status of our parent as checked, because
                    let mut req = req_holder.get_req(req_id).unwrap();
                    req.children = req_children;

                    let cost = req
                        .children
                        .iter()
                        .fold(0, |acc, child| acc + child.2.unwrap_or(0));

                    for parent in &mut req.parents {
                        if parent.0 == parent_req_id {
                            parent.1 = Status::Checked;
                            return Ok((Some(cost), Status::Checked));
                        }
                    }
                }
                "OR" => {
                    //After cycling the children, and no error performed, return an Ok to the parent.
                    //For now, we will set the status of our parent as checked, because
                    let mut req = req_holder.get_req(req_id).unwrap();
                    req.children = req_children;

                    let cost = req
                        .children
                        .iter()
                        .fold(0, |acc, child| acc + child.2.unwrap_or(0));

                    for parent in &mut req.parents {
                        if parent.0 == parent_req_id {
                            parent.1 = Status::Checked;
                            return Ok((Some(cost), Status::Checked));
                        }
                    }
                }
                _ => panic!("Invalid logic type for Group {:?}", req_id),
            }
        } else if pftype.eq("Class") {
            let req_children = self.evaluate_children(&pftype, req_holder, req_id, nests);
            match logic_type.as_str() {
                "AND" => {
                    //After cycling the children, and no error performed, return an Ok to the parent.
                    //For now, we will set the status of our parent as checked, because
                    let mut req = req_holder.get_req(req_id).unwrap();
                    req.children = req_children;

                    let cost = req
                        .children
                        .iter()
                        .fold(0, |acc, child| acc + child.2.unwrap_or(0));

                    for parent in &mut req.parents {
                        if parent.0 == parent_req_id {
                            parent.1 = Status::Checked;
                            return Ok((Some(cost), Status::Checked));
                        }
                    }
                }

                "OR" => {}

                _ => panic!("Invalid logic type for Class {:?}", req_id),
            }
        } else {
            panic!("This reqs pftype is currently unsupported! {:?}", req_id);
        }

        //No need to set in_analysis to false, because our clone never had in_analysis set to true.
        //req_holder.get_req(req_id).unwrap().in_analysis = false;
        //*req_holder.get_req(req.id).unwrap() = req;

        return Err(ScheduleError::UnimiplementedLogicError);
    }
    fn modify_parent_status(
        &mut self,
        req_holder: &mut ReqHolder,
        status: Status,
        parent_id: i32,
        id: i32,
        nests: usize,
    ) {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        println!(
            "{}Selecting parent (req_id: {}) of child (req_id: {})",
            &spacing, parent_id, id
        );
        let req = req_holder.get_req(id).unwrap();

        for parent in &mut req.parents {
            if parent.0 == parent_id {
                parent.1 = status;
                break;
            }
        }
    }

    fn satisfy_prereq(
        &mut self,
        req_holder: &mut ReqHolder,
        postreq_id: i32,
        id: i32,
        nests: usize,
    ) -> Result<(Option<i32>, Status), ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        println!(
            "{}Satisfying prereq (req_id: {}) of parent (req_id: {})",
            &spacing, id, postreq_id
        );

        let (logic_type, pftype) = {
            let req = req_holder.get_req(id).unwrap();
            req.in_analysis = true;
            println!(
                "\n{}Evaluating requirements of: \n{}{:?}",
                &spacing, &spacing, &req
            );

            (req.logic_type.clone(), req.pftype.clone())
        };

        //if the logic type is none, it's probably a class.
        if let None = logic_type {
            match pftype.as_str() {
                "Class" => {
                    let status = Status::Checked;
                    self.modify_parent_status(req_holder, status.clone(), postreq_id, id, nests);

                    let req = req_holder.get_req(id).unwrap();
                    let credits = req.class.as_ref().unwrap().credits.unwrap();
                    return Ok((Some(credits), status));
                }
                _ => {
                    panic!("This reqs pftype is currently unsupported! {:?}", id);
                }
            }
        }

        let logic_type = logic_type.unwrap();
        if pftype.eq("Group") {
        } else if pftype.eq("Class") {
        } else {
            panic!("This reqs pftype is currently unsupported! {:?}", id);
        }

        Err(ScheduleError::UnimiplementedLogicError)
    }

    fn evaluate_children(
        &mut self,
        pftype: &str,
        req_holder: &mut ReqHolder,
        req_id: i32,
        nests: usize,
    ) -> Vec<(i32, Status, Option<i32>)> {
        match pftype {
            "Group" => {
                req_holder
                    .get_req(req_id)
                    .unwrap()
                    .children
                    .clone()
                    .into_iter()
                    .map(|(child_id, child_status, child_cost)| {
                        let (child_cost, child_status) = match self.satisfy_requirements(
                            req_holder,
                            Some(req_id),
                            child_id,
                            nests + 1,
                        ) {
                            Ok(res) => res,
                            Err(e) => {
                                //This is an error that occurred in a child req
                                //We need to pass this error up the tree
                                panic!("Error in child req (req_id: {}): {:?}", child_id, e);
                            }
                        };
                        (child_id, child_status, child_cost)
                    })
                    .collect()
            }
            "Class" => {
                req_holder
                    .get_req(req_id)
                    .unwrap()
                    .children
                    .clone()
                    .into_iter()
                    .map(|(child_id, child_status, child_cost)| {
                        let (child_cost, child_status) =
                            match self.satisfy_prereq(req_holder, req_id, child_id, nests + 1) {
                                Ok(res) => res,
                                Err(e) => {
                                    //This is an error that occurred in a child req
                                    //We need to pass this error up the tree
                                    panic!("Error in child req (req_id: {}): {:?}", child_id, e);
                                }
                            };
                        (child_id, child_status, child_cost)
                    })
                    .collect()
            }
            _ => panic!("Invalid pftype for evaluate_children"),
        }
    }

    /*
       Note: When the parent inside this function is already mutably borrowed.
    */
    /*
    fn evaluate_prereq(
        &mut self,
        req_holder: &mut ReqHolder,
        parent: Rc<RefCell<Req>>,
        req: Rc<RefCell<Req>>,
        nests: usize,
    ) -> Result<i32, ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        println!("{}Evaluating prereq (req_id: {})", &spacing, req.id);

        let pftype = req.borrow().pftype.clone();
        let logic_type = req.borrow().logic_type.clone();
        if pftype.eq("Group") {
            //If this is a group, we have to run logic that's different from our evaluate_prereq.
            //let cost = self.satisfy_requirements(req_holder, req_id, nests)?;
            if let Some(logic_type) = logic_type {
                match logic_type.as_str() {
                    "AND" => {}
                    "OR" => {
                        //This logic is pretty much identical to a Class's OR logic in evaluate prereq. Optimize in the future?
                        let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);

                        for (internal_indice, child) in req.children.iter_mut().enumerate() {
                            let mut child_req = req_holder.get_req(child.0).unwrap();
                            child_req.in_analysis = true;

                            let cost = match self.evaluate_prereq(
                                req_holder,
                                req,
                                child_req,
                                nests + 1,
                            ) {
                                Ok(cost) => cost,
                                Err(e) => {
                                    match e {
                                        ScheduleError::PrereqError => {
                                            //If a single child in this GroupAND has a prereqError, the whole
                                            //group must be thrown away.
                                            println!("{}This Group (req_id: {}) has a child (req_id: {}) with a PrereqError!",
                                            &spacing, req.borrow().id, child.0);

                                            child.1 = Unsuitable;
                                            return Err(e);
                                        }
                                        _ => panic!(
                                            "GroupAND recieved a child with an invalid error!"
                                        ),
                                    }
                                }
                            };
                            child.2 = Some(cost);
                            if minimal_cost.1 > cost {
                                minimal_cost = (internal_indice, cost);
                            };
                        }

                        //Now we return the minimal cost found and set it either to
                        //desirable or selected. TODO
                        println!("{}Minimal Cost: {:?}", &spacing, &minimal_cost);
                        req.borrow_mut().children[minimal_cost.0].1 = Selected;
                    }
                    _ => panic!("eval_prereq(): Invalid logic type for Group {:?}", req),
                }
            } else {
                panic!("eval_prereq(): Group has no logic type! {:?}", req);
            }

        //--------------------------CLASS--------------------------
        } else if pftype.eq("Class") {
            if let Some(logic_type) = logic_type {
                //Do something
            } else {
                //No logic type, so we need to evaluate this parent.

                for parent in &mut req.borrow_mut().parents {
                    let parent_req = req_holder.get_req(parent.0).unwrap();
                    println!("{}borrowed parent (req_id: {})", &spacing, parent.0);
                    parent_req.borrow_mut().in_analysis = true;

                    let mut cost = i32::MAX;
                    //So we need to NOT evalute classes.
                    if parent_req.borrow().pftype.eq("Class") {
                        continue;
                    }
                    if parent.1 == Unchecked {
                        //If this is unchecked, this means it's been unevaluated.
                        //We need to evaluate it.
                        //todo:
                        cost = match self.satisfy_requirements(
                            req_holder,
                            parent_req.clone(),
                            nests + 1,
                        ) {
                            Ok(cost) => cost,
                            Err(e) => {
                                //for now, we just return our error. But I imagine this will be implemented in the future.
                                return Err(e);
                            }
                        };

                        //So this parent will have mutated this req's status and set it in the req_holder but THIS FUNCTION OWNS it officially. So we'll want to rerun the logic here. The other req doesn't matter, because it's just chillin
                        //in the req_holder.
                        //We'll actually wanna get this value from our req_holder
                        //TODO: Check if this is still necessary
                        let new_status = (parent_req
                            .borrow()
                            .children
                            .iter()
                            //this could be bad for it.
                            .find(|x| x.0 == req.clone().borrow().id)
                            .unwrap()
                            .1)
                            .to_owned();
                        parent.1 = new_status;
                        //Now that we have a status
                    }

                    //check the status.
                    match parent.1 {
                        Unsuitable => {
                            panic!("Unhandled behavior! parent found this req unsuitable!")
                        }
                        Unchecked => {
                            //If this req is unchecked, we can just return unchecked.
                            panic!(
                                "{}This req (req_id: {}) is unchecked after parent evaluation!",
                                &spacing,
                                req.borrow().id
                            );
                        }
                        Checked => {
                            //This means that it was not selected, and we should do something.
                            //Set this req to desirable.
                            for child in parent_req.borrow_mut().children.iter_mut() {
                                if child.0 == req.borrow().id {
                                    child.1 = Desirable;
                                }
                            }
                        }
                        Desirable => {
                            //TODO
                            println!(
                                "{}This req (req_id: {}) is desirable!",
                                &spacing,
                                req.borrow().id
                            );
                            //return the difference between this class's credits and the cost.
                            panic!("unimiplemented!");
                        }
                        Selected => {
                            //If this req is selected, we can just return selected.
                            println!(
                                "{}This req (req_id: {}) is selected!",
                                &spacing,
                                req.borrow().id
                            );
                        }
                    }

                    return Ok(req.borrow().class.as_ref().unwrap().credits.unwrap() - cost);
                }
            }
        } else {
            panic!("This reqs pftype is currently unsupported! {:?}", req);
        }
        //TODO: this is -1 because it should not reach this point for now.
        Ok(-1)
    }
    */
    fn reconsider_prereq(&self, req: &mut Req, child_id: i32, nests: usize) {
        for child in req.children.iter_mut() {
            if child.0 == child_id {
                child.1 = Desirable;
            }
        }
    }

    /*

    fn evaluate_group_AND_req(
        &mut self,
        req_holder: &mut ReqHolder,

        req: Req,
        carried_result: &mut Result<i32, ScheduleError>,
        nests: usize,
    ) {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        let mut carried_result: Result<i32, ScheduleError> = Ok(0);
        let req_id = req.borrow().id;

        for child in &mut req.borrow_mut().children {
            let child_req = req_holder.get_req(child.0).unwrap();
            child_req.borrow_mut().in_analysis = true;

            match self.satisfy_requirements(req_holder, child_req.clone(), nests + 1) {
                Ok(cost) => {
                    child.1 = Selected;
                    child.2 = Some(cost);

                    //This parent also needs to be selected in this child's parents.
                    for parent in &mut child_req.borrow_mut().parents {
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
                            &spacing, req.borrow().id, child.0);

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

    fn evaluate_group_OR_req(
        &mut self,
        req_holder: &mut ReqHolder,
        req: Rc<RefCell<Req>>,
        carried_result: &mut Result<i32, ScheduleError>,
        nests: usize,
    ) {
        let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);
        let req_id = req.borrow().id;

        for (internal_indice, child) in req.borrow_mut().children.iter_mut().enumerate() {
            //Check to see if the children were already checked.
            //If they were checked, we just need to continue
            if child.1 != Unchecked {
                //Figure out if this child is the minimal cost
                if let Some(child_cost) = child.2 {
                    minimal_cost = if child_cost < minimal_cost.1 {
                        (internal_indice, child_cost)
                    } else {
                        minimal_cost
                    };
                }
                continue;
            }

            let mut child_req = req_holder.get_req(child.0).unwrap().clone();
            child_req.borrow_mut().in_analysis = true;

            match self.satisfy_requirements(req_holder, child_req.clone(), nests + 1) {
                Ok(result) => {
                    child.2 = Some(result);
                    minimal_cost = if result < minimal_cost.1 {
                        (internal_indice, result)
                    } else {
                        minimal_cost
                    };

                    child.1 = Checked;
                    //also check the parent
                    for parent in &mut child_req.borrow_mut().parents {
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

        //Now we just need to select the minimal cost child.
        let selected_child = &mut req.borrow_mut().children[minimal_cost.0];
        selected_child.1 = Selected;
        for parent in &mut req_holder
            .get_req(selected_child.0)
            .unwrap()
            .borrow_mut()
            .parents
        {
            if parent.0 == req.borrow().id {
                parent.1 = Selected;
                break;
            }
        }
        *carried_result = Ok(minimal_cost.1);
    }

    fn evaluate_class_AND_req(
        &mut self,
        req_holder: &mut ReqHolder,
        req: Rc<RefCell<Req>>,
        carried_result: &mut Result<i32, ScheduleError>,
        nests: usize,
    ) {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        let req_id = req.borrow().id;

        for child in &mut req.borrow_mut().children {
            let child_req = req_holder.get_req(child.0).unwrap().clone();
            child_req.borrow_mut().in_analysis = true;
            match self.satisfy_requirements(req_holder, child_req.clone(), nests + 1) {
                Ok(cost) => {
                    child.2 = Some(cost);
                    child.1 = Selected;

                    //This parent also needs to be selected in this child's parents.
                    for parent in &mut child_req.borrow_mut().parents {
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
                            &spacing, req.borrow().id, child.0);

                            child.1 = Unsuitable;
                            *carried_result = Err(e);
                            break;
                        }
                        _ => panic!("GroupAND recieved a child with an invalid error!"),
                    }
                }
            }
        }
    }

    fn evaluate_class_OR_req(
        &mut self,
        req_holder: &mut ReqHolder,
        req: Rc<RefCell<Req>>,
        carried_result: &mut Result<i32, ScheduleError>,
        nests: usize,
    ) {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();

        let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);
        for (internal_indice, child) in req.borrow_mut().children.iter_mut().enumerate() {
            let child_req = req_holder.get_req(child.0).unwrap().clone();
            child_req.borrow_mut().in_analysis = true;

            let cost =
                match self.evaluate_prereq(req_holder, req.clone(), child_req.clone(), nests + 1) {
                    Ok(cost) => cost,
                    Err(e) => {
                        match e {
                            ScheduleError::PrereqError => {
                                //this means that this child is not suitable for use
                                //however since this is OR logic, we can keep evaluating children.
                                //unlike groupAND.
                                child.1 = Unsuitable;
                                continue;
                            }
                            _ => panic!("GroupOR recieved a child with an invalid error!"),
                        }
                    }
                };
            child.2 = Some(cost);

            if minimal_cost.1 > cost {
                minimal_cost = (internal_indice, cost);
            };
        }

        println!("{}Minimal Cost: {:?}", &spacing, &minimal_cost);
        req.borrow_mut().children[minimal_cost.0].1 = Selected;
        let child_id_in_req_holder = req.borrow_mut().children[minimal_cost.0].0;

        //Note that we aren't copying this and setting the value to it. We are
        //going straight to the value in self.reqs and changing it.
        let req_id = req.borrow().id;
        for parent in &mut req_holder
            .get_req(child_id_in_req_holder)
            .unwrap()
            .borrow_mut()
            .parents
        {
            if parent.0 == req_id {
                parent.1 = Selected;
                break;
            }
        }

        //If this has none of the prior conditions have been met, this child
    }
    */
    /*
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
        let current_req = req_holder.get_req(id).unwrap();

        if let Some(_class) = &current_req.class {
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
            for child in &mut current_req.children {
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
        println!(
            "{}This component (req_id: {}) is not a class.",
            &spacing, &id
        );
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
    */
}
