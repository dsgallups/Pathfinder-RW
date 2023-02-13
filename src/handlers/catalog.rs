use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::{
    handlers::types::{InstantiationType, LogicalType, ParsedLogicType},
    models::{
        associations::{NewComponentAssoc, NewDegreeToComponent},
        class::NewClass,
        component::{self, Component, NewComponent},
        degree::{self, NewDegree},
    },
};

use InstantiationType::{Class, Degree, Group, SimpleClass};
use LogicalType::{GroupAND, GroupOR, PrereqAND, PrereqOR};
pub struct Catalog {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    components: Vec<Component>,
}

impl Catalog {
    pub fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self {
            conn,
            components: Vec::new(),
        }
    }

    fn check_for_component(&mut self, name: &str) -> Option<usize> {
        let in_self = self.components.iter().position(|v| v.name.eq(name));

        if let Some(component_i) = in_self {
            return Some(component_i);
        }

        //so check if it exists, if not, make it.
        if let Ok(component) = Component::find_by_name(name, &mut self.conn) {
            self.components.push(component);
            return Some(self.components.len() - 1);
        };
        None
    }

    pub fn create_component(
        &mut self,
        instantiation_type: &InstantiationType,
        logic_type: &Option<LogicalType>,
    ) -> usize {
        //Here we will take our component types by instantiation, and then match it to
        //the logical type
        match instantiation_type {
            &Group(name) => {
                //So then we make this group based on its logic_type
                self.group(name, logic_type)
            }
            &SimpleClass(name) => self.c(name),
            &Class((name, credits)) => match logic_type {
                &Some(logic) => self.create_class(name, credits, logic_type),
                &None => self.class(name, credits),
            },
        }
    }
    //In theory, groups are only made if we KNOW what their logical type is.
    pub fn group(&mut self, name: &str, logic_type: &Option<LogicalType>) -> usize {
        if let Some(index) = self.check_for_component(name) {
            //ensure that this group has the same logical type.
            match self.components[index].logic_type {
                //if has a logic type
                Some(component_logic) => match logic_type {
                    &Some(l) => {
                        panic!("Error setting logical type for group. Already set!")
                    }
                    &None => {}
                },
                //if no logic type exists in our memory
                None => {
                    //if the value has no logical type
                    //then give it one
                    if let Some(l) = logic_type {
                        Component::update_logic_type(
                            &self.components[index].id,
                            l.to_string(),
                            &mut self.conn,
                        );
                        //update the component in the array
                        //this is manually set instead of querying the DB for its full update.
                        //This is for speed purposes and I don't wanna do like 200 queries.
                        self.components[index].logic_type = Some(l.to_string());
                    }
                }
            }
            return index;
        }

        let logic_type_str = match logic_type {
            &Some(l) => Some(l.to_string()),
            &None => None,
        };

        //So if there's no component, we make one
        let new_component = NewComponent {
            name: Some(name.to_string()),
            pftype: Some("Group".to_string()),
            logic_type: logic_type_str,
        };

        match new_component.create(&mut self.conn) {
            Ok(component) => {
                println!("Created Component: {:?}", &component);
                self.components.push(component);
                self.components.len() - 1
            }
            Err(e) => {
                panic!("Error: {}", e)
            }
        }
    }

    pub fn c(&mut self, name: &str) -> usize {
        self.create_class(name, 3, None)
    }

    pub fn class(&mut self, name: &str, credits: i32) -> usize {
        self.create_class(name, credits, None)
    }
    //This function should be allowed to edit the logical type associated with a class.
    #[allow(unused_assignments)]
    pub fn create_class(
        &mut self,
        name: &str,
        credits: i32,
        logic_type: Option<LogicalType>,
    ) -> usize {
        //Make a class, then make a component for the class and return its index
        //however, first check for its existence
        //let logic_type_str = logic_type.unwrap_or("None".to_string()).to_string();

        if let Some(index) = self.check_for_component(name) {
            match self.components[index].logic_type {
                //If there is an existing logic type found for this component
                Some(component_logic_type) => {
                    //If we passed a logic type to this class.
                    if let Some(l) = logic_type {
                        panic!("Logical type passed to class, but logical type was already set!");
                        //TODO: If this is the same type, just append its children...or maybe do that
                        //elsewhere.
                    }
                }
                //If there is no logic type found for this component
                None => {
                    //If not set, let's set this logic_type if it was passed and update the db.
                    if let Some(l) = logic_type {
                        Component::update_logic_type(
                            &self.components[index].id,
                            l.to_string(),
                            &mut self.conn,
                        );
                        //update the component in the array
                        //this is manually set instead of querying the DB for its full update.
                        //This is for speed purposes and I don't wanna do like 200 queries.
                        self.components[index].logic_type = Some(l.to_string());
                    }

                    //Otherwise, if None is passed for both, we're chillin
                }
            }
            return index;
        }

        //So, if it doesn't exist, let's make a new component
        let logic_type_string = match logic_type {
            Some(logic) => Some(logic.to_string()),
            None => None,
        };

        let new_component = NewComponent {
            name: Some(name.to_string()),
            pftype: Some("Class".to_string()),
            logic_type: logic_type_string,
        };

        let mut index = usize::MAX;

        let comp = match new_component.create_class_component(&mut self.conn) {
            Ok(comp) => {
                println!("Created Class Component: {:?}", comp);
                self.components.push(comp);
                index = self.components.len() - 1;
                &self.components[index]
            }
            Err(e) => {
                panic!("Error creating class components: {}", e)
            }
        };

        let new_class = NewClass {
            name: Some(name.to_string()),
            description: None,
            credits: Some(credits),
            pftype: None,
            subject: None,
            course_no: None,
            options: None,
            component_id: Some(comp.id),
        };

        match new_class.create(&mut self.conn) {
            Ok(class) => {
                println!("Created Class: {:?}", class)
            }
            Err(e) => {
                panic!("Error creating class: {}", e)
            }
        }
        index
    }

    fn create_component_assoc(
        &mut self,
        parent_indice: usize,
        parsed_children_indices: ParsedLogicType,
    ) {
        let parent = &self.components[parent_indice];

        let logic_type = match parsed_children_indices {
            ParsedLogicType::GroupAND(_) | ParsedLogicType::PrereqAND(_) => "AND",
            ParsedLogicType::GroupOR(_) | ParsedLogicType::PrereqOR(_) => "OR",
        };

        match parsed_children_indices {
            ParsedLogicType::PrereqAND(children_indices)
            | ParsedLogicType::PrereqOR(children_indices)
            | ParsedLogicType::GroupAND(children_indices)
            | ParsedLogicType::GroupOR(children_indices) => {
                for child_i in children_indices {
                    let child = &self.components[child_i];

                    let new_component_assoc = NewComponentAssoc {
                        parent_id: parent.id,
                        child_id: child.id,
                    };
                    match new_component_assoc.create(&mut self.conn) {
                        Ok(new_assoc) => {
                            println!("Created component association: {:?}", new_assoc);
                        }
                        Err(e) => {
                            panic!("Error creating component association: {}", e)
                        }
                    }
                }
            }
        }
    }

    pub fn gen_catalog(&mut self) {
        //first we get parse self.cs
        let catalog = vec![
            (
                Group("CNIT CORE"),
                GroupAND(vec![
                    SimpleClass("CNIT 18000"),
                    SimpleClass("CNIT 15501"),
                    SimpleClass("CNIT 17600"),
                    SimpleClass("CNIT 24200"),
                    SimpleClass("CNIT 25501"),
                    SimpleClass("CNIT 27000"),
                    SimpleClass("CNIT 27200"),
                    SimpleClass("CNIT 28000"),
                    SimpleClass("CNIT 32000"),
                    SimpleClass("CNIT 48000"),
                ]),
            ),
            (
                Group("CNIT DB PROGRAMMING"),
                GroupOR(vec![SimpleClass("CNIT 37200"), SimpleClass("CNIT 39200")]),
            ),
            (
                Group("CNIT SYS/APP DEV"),
                GroupOR(vec![SimpleClass("CNIT 31500"), SimpleClass("CNIT 32500")]),
            ),
            (
                Group("GENERAL BUSINESS SELECTIVE"),
                GroupOR(vec![
                    SimpleClass("IET 10400"),
                    SimpleClass("IT 10400"),
                    SimpleClass("TLI 11100"),
                    SimpleClass("TLI 15200"),
                ]),
            ),
            (
                Group("UNIV CORE"),
                GroupAND(vec![
                    SimpleClass("SCLA 10100"),
                    SimpleClass("SCLA 10200"),
                    SimpleClass("TECH 12000"),
                    Class(("MA 16010", 5)),
                    Class(("MA 16020", 5)),
                    SimpleClass("OLS 25200"),
                    SimpleClass("TLI 11200"),
                    SimpleClass("PHIL 15000"),
                    SimpleClass("COMSEL 00000"),
                    SimpleClass("ECONSEL 00000"),
                    SimpleClass("SCISEL 00000"),
                    SimpleClass("LABSCISEL 00000"),
                    SimpleClass("ACCSEL 00000"),
                    SimpleClass("STATSEL 00000"),
                    SimpleClass("SPEAKSEL 00000"),
                    SimpleClass("WRITINGSEL 00000"),
                    SimpleClass("HUMSEL 00000"),
                    SimpleClass("BEHAVSCISEL 00000"),
                    SimpleClass("FOUNDSEL 00000"),
                ]),
            ),
            (
                Group("CNIT/SAAD INTERDISC"),
                GroupAND(vec![SimpleClass("INTERDISC 00000")]),
            ),
            (
                SimpleClass("CNIT 27000"),
                PrereqAND(vec![SimpleClass("CNIT 17600"), SimpleClass("CNIT 15501")]),
            ),
            (
                SimpleClass("CNIT 28000"),
                PrereqAND(vec![SimpleClass("CNIT 18000")]),
            ),
            (
                SimpleClass("CNIT 25501"),
                PrereqAND(vec![SimpleClass("CNIT 15501")]),
            ),
            (
                SimpleClass("CNIT 24200"),
                PrereqAND(vec![SimpleClass("CNIT 17600")]),
            ),
            (
                Class(("CNIT 34010", 1)),
                PrereqAND(vec![SimpleClass("CNIT 24200")]),
            ),
            (
                SimpleClass("CNIT 34400"),
                PrereqAND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 32000"),
                PrereqAND(vec![SimpleClass("TECH 12000")]),
            ),
            (
                SimpleClass("CNIT 37000"),
                PrereqAND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 32200"),
                PrereqAND(vec![SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 31500"),
                PrereqAND(vec![SimpleClass("CNIT 25501")]),
            ),
            (
                SimpleClass("CNIT 34220"),
                PrereqOR(vec![SimpleClass("CNIT 34000"), Class(("CNIT 34010", 1))]),
            ),
            (
                SimpleClass("CNIT 47000"),
                PrereqAND(vec![SimpleClass("CNIT 32000")]),
            ),
            (
                SimpleClass("CNIT 48000"),
                PrereqAND(vec![SimpleClass("CNIT 28000")]),
            ),
            (
                SimpleClass("CNIT 47100"),
                PrereqAND(vec![SimpleClass("CNIT 45500"), SimpleClass("CNIT 37000")]),
            ),
            (
                SimpleClass("CNIT 34000"),
                PrereqAND(vec![SimpleClass("CNIT 24200")]),
            ),
            (
                SimpleClass("CNIT 34500"),
                PrereqAND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 24000")]),
            ),
            (
                SimpleClass("CNIT 34600"),
                PrereqAND(vec![SimpleClass("CNIT 24000"), SimpleClass("CNIT 24200")]),
            ),
            (
                Group("NETWORK ENGR GROUPED 455 PREREQ"),
                GroupOR(vec![SimpleClass("CNIT 34500"), SimpleClass("CNIT 34400")]),
            ),
            (
                SimpleClass("CNIT 45500"),
                PrereqAND(vec![
                    SimpleClass("CNIT 34220"),
                    Group("NETWORK ENGR GROUPED 455 PREREQ"),
                ]),
            ),
            (
                Class(("MA 16020", 5)),
                PrereqAND(vec![Class(("MA 16010", 5))]),
            ),
        ];

        let mut parsed_assocs: Vec<(usize, ParsedLogicType)> = Vec::new();

        for item in catalog {
            let parent_component = item.0;
            let logical_type = item.1;

            //First we need to make an appropriate parent_component

            //If it's reg, GroupAND, then we should make it GroupAND
            let parent_component_indice =
                self.create_component(&parent_component, &Some(logical_type));

            let mut indices: Vec<usize> = Vec::new();
            match &logical_type {
                GroupAND(components)
                | GroupOR(components)
                | PrereqAND(components)
                | PrereqOR(components) => {
                    self.instantiations_to_indices(&mut indices, components);
                }
            }

            let parsed_logical_type = match &logical_type {
                GroupAND(_) => ParsedLogicType::GroupAND(indices),
                GroupOR(_) => ParsedLogicType::GroupOR(indices),
                PrereqAND(_) => ParsedLogicType::PrereqAND(indices),
                PrereqOR(_) => ParsedLogicType::PrereqOR(indices),
            };

            //TODO: Now we need to update the parent_component's logical type based what's identified
            let parent_component = &mut self.components[parent_component_indice];

            parsed_assocs.push((parent_component_indice, parsed_logical_type));
        }

        for association in parsed_assocs {
            self.create_component_assoc(association.0, association.1);
        }

        let degree_requirements = vec![
            (
                Degree(("CNIT", "Computer and Information Technology", "Major")),
                vec![
                    Group("CNIT CORE"),
                    Group("CNIT DB PROGRAMMING"),
                    Group("CNIT SYS/APP DEV"),
                    Group("CNIT/SAAD INTERDISC"),
                    Group("CNIT IT SELECTIVES"),
                    Group("UNIV CORE"),
                    Group("GENERAL BUSINESS SELECTIVE"),
                    SimpleClass("FREE 00000"),
                ],
            ),
            (
                Degree(("CSEC", "Cybersecurity", "Major")),
                vec![
                    Group("CNIT CORE"),
                    SimpleClass("CNIT 31500"),
                    SimpleClass("CNIT 32200"),
                    SimpleClass("CNIT 34400"),
                    SimpleClass("CNIT 34010"),
                    SimpleClass("CNIT 34220"),
                    SimpleClass("CNIT 37000"),
                    SimpleClass("CNIT 42000"),
                    SimpleClass("CNIT 42200"),
                    SimpleClass("CNIT 45500"),
                    SimpleClass("CNIT 47000"),
                    SimpleClass("CNIT 47100"),
                    Group("CSEC SELECTIVES"),
                    Group("CSEC INTERDISC"),
                    Group("UNIV CORE"),
                ],
            ),
            (
                Degree(("NENT", "Network Engineering Technology", "Major")),
                vec![
                    Group("CNIT CORE"),
                    SimpleClass("CNIT 31500"),
                    SimpleClass("CNIT 24000"),
                    SimpleClass("CNIT 34500"),
                    SimpleClass("CNIT 34600"),
                    SimpleClass("CNIT 34000"),
                    SimpleClass("CNIT 34210"),
                    SimpleClass("CNIT 34220"),
                    SimpleClass("CNIT 45500"),
                    Group("NENT IT SELECTIVES"),
                    Group("NENT INTERDISC"),
                    Group("UNIV CORE"),
                    Group("GENERAL BUSINESS SELECTIVE"),
                ],
            ),
            (
                Degree(("SAAD", "Systems Analysis and Design", "Major")),
                vec![
                    Group("CNIT CORE"),
                    SimpleClass("CNIT 39200"),
                    Group("CNIT SYS/APP DEV"),
                    SimpleClass("CNIT 38000"),
                    SimpleClass("CGT 25600"),
                    Group("SAAD SELECTIVES"),
                    Group("SAAD IT SELECTIVES"),
                    Group("UNIV CORE"),
                    Group("CNIT/SAAD INTERDISC"),
                ],
            ),
        ];

        //We have to repeat some code because of the borrow checker...
        for item in degree_requirements {
            let degree_in_instantiaion = item.0;
            let instantiations = item.1;

            if let Degree(degree_strs) = degree_in_instantiaion {
                //actually this is a code but im lazy to change it rn.
                //TODO
                let new_degree = NewDegree {
                    code: Some(degree_strs.0.to_string()),
                    name: Some(degree_strs.1.to_string()),
                    pftype: Some(degree_strs.2.to_string()),
                    description: None,
                };

                let degree = new_degree.create(&mut self.conn).unwrap();
                println!("Created Degree: {:?}", &degree);
                let mut indices: Vec<usize> = Vec::new();
                self.instantiations_to_indices(&mut indices, &instantiations);
                self.add_degree_requirements(degree, indices);
            } else {
                panic!("Something's where a degree should be!!");
            }
        }
    }

    fn add_degree_requirements(&mut self, degree: degree::Degree, requirements: Vec<usize>) {
        for requirement_indice in requirements {
            let requirement = &self.components[requirement_indice];

            let degree_to_component_assoc = NewDegreeToComponent {
                degree_id: degree.id,
                component_id: requirement.id,
            };
            match degree_to_component_assoc.create(&mut self.conn) {
                Ok(new_assoc) => {
                    println!("Created degree to component association: {:?}", new_assoc);
                }
                Err(e) => {
                    panic!("Error creating degree to component association: {}", e)
                }
            }
        }
    }
    fn instantiations_to_indices(
        &mut self,
        indices: &mut Vec<usize>,
        instantiations: &Vec<InstantiationType>,
    ) {
        for comp in instantiations {
            indices.push(self.create_component(comp, &None));
        }
    }
}
