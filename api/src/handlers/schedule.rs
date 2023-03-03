use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::handlers::types::{
    Req, Schedule, ScheduleError,
    Status::{self, Checked, Desirable, Selected, Unchecked, Unsuitable},
};
use crate::models::{
    associations::{ComponentToComponent, DegreeToComponent},
    class::Class,
    component::Component,
    degree::Degree,
};
use rustc_hash::FxHashMap as HashMap;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct ReqHolder {
    reqs: HashMap<i32, RefCell<Req>>,
}

impl ReqHolder {
    fn new() -> ReqHolder {
        ReqHolder {
            reqs: HashMap::default(),
        }
    }
    fn add_degree_req(&mut self, degree: Req) {
        self.reqs.insert(degree.id, RefCell::new(degree));
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
            RefCell::new(Req {
                id: component.id,
                name: component.name,
                pftype: component.pftype,
                class,
                logic_type: component.logic_type,
                children: Vec::new(),
                parents: Vec::new(),
                in_analysis: false,
            }),
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
            child_req.borrow_mut().parents.push((parent_id, Unchecked));
            println!("{spacing}Gave child (req_id: {child_id}) a parent (req_id: {parent_id})");
            println!("child = {:?}", child_req);
            //child_req.parents.push((parent_id, Unchecked));
        }

        if let Some(parent_req) = self.get_req(parent_id) {
            parent_req
                .borrow_mut()
                .children
                .push((child_id, Unchecked, None));
            println!("{spacing}Gave parent (req_id: {parent_id}) a child(req_id: {child_id})");
            println!("parent = {:?}", parent_req);
        }

        Ok(())
    }

    #[inline(always)]
    fn get_req(&self, id: i32) -> Option<&RefCell<Req>> {
        self.reqs.get(&id)
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
                req.borrow().children.clone()
            };

            for child in &req_children {
                self.display_graph(child.0, displayed_reqs);
            }
        }
    }
}

//This is unused for now...
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
        req_holder.display_graph(root_id, &mut Vec::new());
        println!("------------------------------------------End Reqs------------------------------------------");
        //return Ok(Schedule::new());
        //turn the requirements graph into a schedule
        self.analyze_requirements_graph(&mut req_holder)?;

        println!("\n\nanalyze_requirements_graph() finished.");
        println!("------------------------------------------Begin Reqs------------------------------------------");
        req_holder.display_graph(root_id, &mut Vec::new());
        println!("------------------------------------------End Reqs------------------------------------------");

        Ok(Schedule::new())
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
        println!("Root component: {:?}", &req_holder.get_req(root_id));

        self.associate_components(req_holder, root_id, root_components, 0)?;

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
        self.satisfy_group(req_holder, None, root_req_id, 0)?;
        //since this root component has a logic type of AND, all of its requirements MUST
        //be fulfilled

        Ok(())
    }

    /*
        This function satisfies the requirements of the given req.
        Note that THIS FUNCTION SHOULD ONLY MUTATE THE REQ, NOT ITS CHILDREN OR PARENTS.
    */
    #[allow(unreachable_code)]
    fn satisfy_group(
        &mut self,
        req_holder: &mut ReqHolder,
        parent_req_id: Option<i32>,
        req_id: i32,
        nests: usize,
    ) -> Result<(Option<i32>, Status), ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();

        let logic_type = {
            let req = req_holder.get_req(req_id).unwrap().borrow();
            println!("{}Satisfying group: {:?}", &spacing, &req);
            if !req.pftype.eq("Group") {
                panic!("{}This req is not a group! {:?}", &spacing, req_id);
            }
            match req.logic_type {
                Some(ref logic_type) => logic_type.clone(),
                None => {
                    panic!(
                        "{}This reqs pftype is currently unsupported! {:?}",
                        &spacing, req_id
                    );
                }
            }
        };

        let parent_req_id = match parent_req_id {
            Some(id) => id,
            None => {
                //This should only occur for the root component.
                if req_id != -1 {
                    panic!("{}No parent req id! {:?}", &spacing, &req_id);
                }

                let mut req_children = self.evaluate_children(req_holder, req_id, nests);
                self.select_all_valid_children(req_holder, req_id, &mut req_children, nests);
                let mut req = req_holder.get_req(req_id).unwrap().borrow_mut();
                req.children = req_children;
                let cost = req
                    .children
                    .iter()
                    .fold(0, |acc, child| acc + child.2.unwrap_or(0));

                return Ok((Some(cost), Checked));
            }
        };

        match logic_type.as_str() {
            "AND" => {
                println!("{}AND", &spacing);
                //After cycling the children, and no error performed, return an Ok to the parent.
                //For now, we will set the status of our parent as checked, because
                let mut req_children = self.evaluate_children(req_holder, req_id, nests);
                self.select_all_valid_children(req_holder, req_id, &mut req_children, nests);
                let mut req = req_holder.get_req(req_id).unwrap().borrow_mut();
                let cost = req
                    .children
                    .iter()
                    .fold(0, |acc, child| acc + child.2.unwrap_or(0));
                req.children = req_children;
                for parent in &mut req.parents {
                    if parent.0 == parent_req_id {
                        parent.1 = Checked;
                        return Ok((Some(cost), Checked));
                    }
                }
            }
            "OR" => {
                println!("{}OR", &spacing);
                //After cycling the children, and no error performed, return an Ok to the parent.
                //For now, we will set the status of our parent as checked, because
                let req_children = self.evaluate_children(req_holder, req_id, nests);

                let mut req = req_holder.get_req(req_id).unwrap().borrow_mut();
                req.children = req_children;

                let mut minimal_cost: (usize, i32) = (usize::MAX, i32::MAX);
                let mut child_id = i32::MAX;
                for (internal_indice, child) in req.children.iter().enumerate() {
                    if let Some(cost) = child.2 {
                        if cost < minimal_cost.1 {
                            minimal_cost = (internal_indice, cost);
                            child_id = child.0;
                        }
                    }
                    println!("{}Child: {:?}", &spacing, child);
                }
                println!("{}Minimal cost: {:?}", &spacing, minimal_cost);
                //Finding the minimal cost means we can select that (until further logic implemented)
                let status = Selected;
                req.children[minimal_cost.0].1 = status.clone();
                drop(req);

                //Modify the parent status of the child. MAKE SURE this is correct.
                self.modify_parent_status(req_holder, status, req_id, child_id, nests);

                let mut req = req_holder.get_req(req_id).unwrap().borrow_mut();

                for parent in &mut req.parents {
                    if parent.0 == parent_req_id {
                        parent.1 = Checked;
                        return Ok((Some(minimal_cost.1), Checked));
                    }
                }
            }
            _ => panic!("Invalid logic type for Group {:?}", req_id),
        }

        //No need to set in_analysis to false, because our clone never had in_analysis set to true.
        //req_holder.get_req(req_id).unwrap().in_analysis = false;
        //*req_holder.get_req(req.id).unwrap() = req;

        Err(ScheduleError::UnimiplementedLogicError)
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
            "{}Modifying parent (req_id: {}) of child (req_id: {})",
            &spacing, parent_id, id
        );
        let req = req_holder.get_req(id).unwrap();

        for parent in &mut req.borrow_mut().parents {
            if parent.0 == parent_id {
                parent.1 = status;
                break;
            }
        }
    }

    fn modify_child_status(
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
            "{}Modifying child (req_id: {}) of parent (req_id: {})",
            &spacing, id, parent_id
        );
        let req = req_holder.get_req(parent_id).unwrap();

        for child in &mut req.borrow_mut().children {
            if child.0 == id {
                child.1 = status;
                break;
            }
        }
    }

    fn evaluate_parent_req_from_class(
        &mut self,
        req_holder: &mut ReqHolder,
        class_id: i32,
        parent_id: i32,
        nests: usize,
    ) {
        //assume it's a group
        let res = self.satisfy_group(req_holder, Some(class_id), parent_id, nests);
    }

    fn evaluate_children(
        &mut self,
        req_holder: &mut ReqHolder,
        req_id: i32,
        nests: usize,
    ) -> Vec<(i32, Status, Option<i32>)> {
        let children = req_holder
            .get_req(req_id)
            .unwrap()
            .borrow()
            .children
            .clone();

        children
            .into_iter()
            .map(|(child_id, child_status, child_cost)| {
                let pftype = req_holder
                    .get_req(child_id)
                    .unwrap()
                    .borrow()
                    .pftype
                    .clone();

                let (child_cost, child_status) = if pftype.eq("Group") {
                    match self.satisfy_group(req_holder, Some(req_id), child_id, nests + 1) {
                        Ok(res) => res,
                        Err(e) => {
                            //This is an error that occurred in a child req
                            //We need to pass this error up the tree
                            panic!("Error in child req (req_id: {}): {:?}", child_id, e);
                        }
                    }
                } else if pftype.eq("Class") {
                    match self.satisfy_class(req_holder, req_id, child_id, nests + 1) {
                        Ok(res) => res,
                        Err(e) => {
                            //This is an error that occurred in a child req
                            //We need to pass this error up the tree
                            panic!("Error in child req (req_id: {}): {:?}", child_id, e);
                        }
                    }
                } else {
                    panic!("This reqs pftype is currently unsupported! {:?}", child_id);
                };

                (child_id, child_status, child_cost)
            })
            .collect()
    }

    fn select_all_valid_children(
        &mut self,
        req_holder: &mut ReqHolder,
        req_id: i32,
        children: &mut Vec<(i32, Status, Option<i32>)>,
        nests: usize,
    ) {
        for child in children {
            if child.1 == Checked {
                child.1 = Selected;
                //Select the parent in the child
                self.modify_parent_status(req_holder, Selected, req_id, child.0, nests);
            }
        }
    }

    fn satisfy_class(
        &mut self,
        req_holder: &mut ReqHolder,
        parent_req_id: i32,
        req_id: i32,
        nests: usize,
    ) -> Result<(Option<i32>, Status), ScheduleError> {
        let spaces = 4 * nests;
        let spacing = (0..=spaces).map(|_| " ").collect::<String>();
        let extra_space = (0..=4).map(|_| " ").collect::<String>();
        println!(
            "{}Satisfying prereq (req_id: {}) of parent (req_id: {})",
            &spacing, req_id, parent_req_id
        );

        let (logic_type, mut parents) = {
            let mut req = req_holder.get_req(req_id).unwrap().borrow_mut();
            req.in_analysis = true;
            println!("\n{}Evaluating class: \n{}{:?}", &spacing, &spacing, &req);

            (req.logic_type.clone(), req.parents.clone())
        };

        //todo, I feel like this is really funky because of the chance that there's multiple parents.
        let mut additional_cost = 0;
        let additional_status = Unchecked;
        for (parent_id, parent_status) in parents {
            if parent_id == parent_req_id {
                continue;
            }
            {
                let parent_req = req_holder.get_req(parent_id).unwrap().borrow();
                if !parent_req.pftype.eq("Group") {
                    continue;
                }
            }
            //So if not the parent, we need to check out some stuff
            println!("parent to evaluate: {:?}", parent_id);

            if parent_status == Unchecked {
                let result =
                    match self.satisfy_group(req_holder, Some(req_id), parent_id, nests + 1) {
                        Ok(res) => res,
                        Err(e) => {
                            //This is an error that occurred in a child req
                            //We need to pass this error up the tree
                            panic!(
                                "Error in evaluating parent for child class (req_id: {}): {:?}",
                                req_id, e
                            );
                        }
                    };
                if additional_cost < result.0.unwrap() {
                    additional_cost = result.0.unwrap();
                }
            }
        }
        //our parents will have been mutated
        parents = req_holder.get_req(req_id).unwrap().borrow().parents.clone();
        for (parent_id, parent_status) in parents {
            if parent_id == parent_req_id {
                continue;
            }
            {
                let parent_req = req_holder.get_req(parent_id).unwrap().borrow();
                if !parent_req.pftype.eq("Group") {
                    continue;
                }
            }
            match parent_status {
                Unchecked => {
                    panic!("This should never happen!");
                }
                Unsuitable => {
                    panic!("This should never happen! (Unsuitable child)");
                }
                Checked => {
                    //Ask for a reconsideration by setting parent to desirable
                    self.modify_parent_status(req_holder, Desirable, parent_id, req_id, nests);
                    self.modify_child_status(req_holder, Desirable, req_id, parent_id, nests);
                }
                _ => return Err(ScheduleError::UnimiplementedLogicError),
            }
        }

        //if the logic type is none, it's probably a class.
        if logic_type.is_none() {
            //We need this class to be evaluated by all of its parents before doing anything

            let status = Checked;
            self.modify_parent_status(req_holder, status.clone(), parent_req_id, req_id, nests);

            let req = req_holder.get_req(req_id).unwrap().borrow();
            let credits = req.class.as_ref().unwrap().credits.unwrap();
            return Ok((Some(credits), status));
        }

        let logic_type = logic_type.unwrap();

        let req_children = self.evaluate_children(req_holder, req_id, nests);
        let mut req = req_holder.get_req(req_id).unwrap().borrow_mut();
        req.children = req_children;
        match logic_type.as_str() {
            "AND" => {
                /*for (child_id, child_status, child_cost) in children {
                    if child_status == Status::Unchecked {
                        status = Status::Unchecked;
                    }
                    if let Some(child_cost) = child_cost {
                        credits += child_cost;
                    }
                }*/
                let credits = req.class.as_ref().unwrap().credits.unwrap();
                let status = Checked;
                drop(req);
                self.modify_parent_status(req_holder, status.clone(), parent_req_id, req_id, nests);
                Ok((Some(credits), status))
            }
            "OR" => {
                let status = Unchecked;
                let credits = 0;
                /*for (child_id, child_status, child_cost) in req_children {
                    if child_status == Status::Checked {
                        status = Status::Checked;
                    }
                    if let Some(child_cost) = child_cost {
                        credits += child_cost;
                    }
                }*/

                Ok((Some(credits), status))
            }
            _ => panic!("Invalid logic type for Class {:?}", req_id),
        }

        //Err(ScheduleError::UnimiplementedLogicError)
    }
}
