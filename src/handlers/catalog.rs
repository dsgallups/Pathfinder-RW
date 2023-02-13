use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::{
    handlers::types::{InstantiationType, LogicalType, ParsedLogicType},
    models::{
        associations::{NewComponentAssoc, NewDegreeToComponent},
        class::NewClass,
        component::{Component, NewComponent},
        degree::{self, NewDegree},
    },
};

use InstantiationType::{Class, Degree, Reg, SimpleClass};
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
    //This is for components with only a name.
    //We don't know here exactly what its logic_type is
    pub fn reg(&mut self, name: &str) -> usize {
        if let Some(index) = self.check_for_component(name) {
            return index;
        }

        let new_component = NewComponent {
            name: Some(name.to_string()),
            pftype: Some("logical".to_string()),
            logic_type: None,
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
        self.class(name, 3)
    }

    #[allow(unused_assignments)]
    pub fn class(&mut self, name: &str, credits: i32) -> usize {
        //Make a class, then make a component for the class and return its index
        //however, first check for its existence

        if let Some(index) = self.check_for_component(name) {
            return index;
        }

        //We don't know what the component's logical type is except that it's a class here.
        let new_component = NewComponent {
            name: Some(name.to_string()),
            pftype: Some("class".to_string()),
            logic_type: None,
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

    fn create_component_assoc(&mut self, parent_indice: usize, parsed_children: ParsedLogicType) {
        let parent = &self.components[parent_indice];

        let logic_type = match parsed_children {
            ParsedLogicType::GroupAND(_) | ParsedLogicType::PrereqAND(_) => "AND",
            ParsedLogicType::GroupOR(_) | ParsedLogicType::PrereqOR(_) => "OR",
        };

        match parsed_children {
            ParsedLogicType::AND(children_indices) | ParsedLogicType::OR(children_indices) => {
                for child_i in children_indices {
                    let child = &self.components[child_i];

                    let new_component_assoc = NewComponentAssoc {
                        parent_id: parent.id,
                        child_id: child.id,
                        association_type: association_type.to_string(),
                        logic_type: logic_type.to_string(),
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
                Reg("CNIT CORE"),
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
                Reg("CNIT DB PROGRAMMING"),
                GroupOR(vec![SimpleClass("CNIT 37200"), SimpleClass("CNIT 39200")]),
            ),
            (
                Reg("CNIT SYS/APP DEV"),
                GroupOR(vec![SimpleClass("CNIT 31500"), SimpleClass("CNIT 32500")]),
            ),
            (
                Reg("GENERAL BUSINESS SELECTIVE"),
                GroupOR(vec![
                    SimpleClass("IET 10400"),
                    SimpleClass("IT 10400"),
                    SimpleClass("TLI 11100"),
                    SimpleClass("TLI 15200"),
                ]),
            ),
            (
                Reg("UNIV CORE"),
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
                Reg("CNIT/SAAD INTERDISC"),
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
                Reg("NETWORK ENGR GROUPED 455 PREREQ"),
                GroupOR(vec![SimpleClass("CNIT 34500"), SimpleClass("CNIT 34400")]),
            ),
            (
                SimpleClass("CNIT 45500"),
                PrereqAND(vec![
                    SimpleClass("CNIT 34220"),
                    Reg("NETWORK ENGR GROUPED 455 PREREQ"),
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

            //so first we will parse the logicaltype into parsed type
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

            //now we can pass this (hopefully) to parse_assocs
            let parent_component_indice = match &parent_component {
                SimpleClass(c) => self.c(c),
                Class(c) => self.class(c.0, c.1),
                Reg(c) => self.reg(c),
                &_ => {
                    panic!("nuuu")
                }
            };
            //TODO: Now we need to update the parent_component's logical type based what's identified

            parsed_assocs.push((parent_component_indice, parsed_logical_type));
        }

        for association in parsed_assocs {
            self.create_component_assoc(association.0, association.1);
        }

        let degree_requirements = vec![
            (
                Degree(("CNIT", "Computer and Information Technology", "Major")),
                vec![
                    Reg("CNIT CORE"),
                    Reg("CNIT DB PROGRAMMING"),
                    Reg("CNIT SYS/APP DEV"),
                    Reg("CNIT/SAAD INTERDISC"),
                    Reg("CNIT IT SELECTIVES"),
                    Reg("UNIV CORE"),
                    Reg("GENERAL BUSINESS SELECTIVE"),
                    SimpleClass("FREE 00000"),
                ],
            ),
            (
                Degree(("CSEC", "Cybersecurity", "Major")),
                vec![
                    Reg("CNIT CORE"),
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
                    Reg("CSEC SELECTIVES"),
                    Reg("CSEC INTERDISC"),
                    Reg("UNIV CORE"),
                ],
            ),
            (
                Degree(("NENT", "Network Engineering Technology", "Major")),
                vec![
                    Reg("CNIT CORE"),
                    SimpleClass("CNIT 31500"),
                    SimpleClass("CNIT 24000"),
                    SimpleClass("CNIT 34500"),
                    SimpleClass("CNIT 34600"),
                    SimpleClass("CNIT 34000"),
                    SimpleClass("CNIT 34210"),
                    SimpleClass("CNIT 34220"),
                    SimpleClass("CNIT 45500"),
                    Reg("NENT IT SELECTIVES"),
                    Reg("NENT INTERDISC"),
                    Reg("UNIV CORE"),
                    Reg("GENERAL BUSINESS SELECTIVE"),
                ],
            ),
            (
                Degree(("SAAD", "Systems Analysis and Design", "Major")),
                vec![
                    Reg("CNIT CORE"),
                    SimpleClass("CNIT 39200"),
                    Reg("CNIT SYS/APP DEV"),
                    SimpleClass("CNIT 38000"),
                    SimpleClass("CGT 25600"),
                    Reg("SAAD SELECTIVES"),
                    Reg("SAAD IT SELECTIVES"),
                    Reg("UNIV CORE"),
                    Reg("CNIT/SAAD INTERDISC"),
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
            match comp {
                SimpleClass(c) => {
                    indices.push(self.c(c));
                }
                Class(c) => {
                    indices.push(self.class(c.0, c.1));
                }
                Reg(c) => {
                    indices.push(self.reg(c));
                }
                Degree(_) => {
                    panic!("NOOOOO")
                }
            }
        }
    }
}