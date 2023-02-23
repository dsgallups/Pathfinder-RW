use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::{
    handlers::types::{InstantiationType, LogicalType, ParsedLogicType},
    models::{
        associations::{NewComponentAssoc, NewDegreeToComponent},
        class::{self, NewClass},
        component::{Component, NewComponent},
        degree::{self, NewDegree},
    },
};

use InstantiationType::{Class, Degree, Group, SimpleClass};
use LogicalType::{AND, OR};
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
            println!(
                "Found Component internally: {:?}",
                &self.components[component_i]
            );
            return Some(component_i);
        }

        //so check if it exists in the db, if so, add it to our components.
        if let Ok(component) = Component::find_by_name(name, &mut self.conn) {
            println!("Found Component in DB: {:?}", &component);
            self.components.push(component);
            return Some(self.components.len() - 1);
        };
        None
    }

    fn parse_catalog_item(
        &mut self,
        instantiation_type: &InstantiationType,
        logic_type: &Option<&LogicalType>,
    ) -> usize {
        //Here we will take our component types by instantiation, and then match it to
        //the logical type
        match *instantiation_type {
            Group(name) => {
                //So then we make this group based on its logic_type
                self.create_component(name, "Group", logic_type)
            }
            SimpleClass(name) => self.create_class(name, 3, logic_type),
            Class((name, credits)) => self.create_class(name, credits, logic_type),
            _ => {
                panic!("Degree component not implemented")
            }
        }
    }

    fn create_component(
        &mut self,
        name: &str,
        pftype: &str,
        logic_type: &Option<&LogicalType>,
    ) -> usize {
        if let Some(index) = self.check_for_component(name) {
            //ensure that this group has the same logical type.
            match &self.components[index].logic_type {
                //if has a logic type
                Some(_component_logic) => match logic_type {
                    Some(_l) => {
                        panic!("Error setting logical type for group. Already set!")
                    }
                    None => {}
                },
                //if no logic type exists in our memory
                None => {
                    //if the value has no logical type
                    //then give it one
                    if let Some(l) = logic_type {
                        //update the component in the array
                        match Component::update_logic_type(
                            &self.components[index].id,
                            l.to_string(),
                            &mut self.conn,
                        ) {
                            Ok(_) => {
                                //this is manually set instead of querying the DB for its full update.
                                //This is for speed purposes and I don't wanna do like 200 queries.
                                self.components[index].logic_type = Some(l.to_string());
                                println!(
                                    "Updated Component Logic Type: {:?}",
                                    &self.components[index]
                                );
                            }
                            Err(e) => {
                                println!("ERROR UPDATING COMPONENT LOGIC TYPE: {e:?}");
                            }
                        }
                    }
                }
            }
            return index;
        }

        let logic_type_str = logic_type.map(|l| l.to_string());

        //So if there's no component, we make one based on its pftype
        let new_component = match pftype {
            "Group" => NewComponent {
                name: Some(name.to_string()),
                pftype: Some("Group".to_string()),
                logic_type: logic_type_str,
            },
            "Class" => NewComponent {
                name: Some(name.to_string()),
                pftype: Some("Class".to_string()),
                logic_type: logic_type_str,
            },
            _ => panic!("Component has an invalid pftype!"),
        };

        match new_component.create(&mut self.conn) {
            Ok(component) => {
                println!("Created Component: {:?}", &component);
                self.components.push(component);
                self.components.len() - 1
            }
            Err(e) => {
                panic!("Error: {e}")
            }
        }
    }

    //This function should be allowed to edit the logical type associated with a class.
    fn create_class(
        &mut self,
        name: &str,
        credits: i32,
        logic_type: &Option<&LogicalType>,
    ) -> usize {
        //Make a class, then make a component for the class and return its index
        //however, first check for its existence
        let new_component_indice = self.create_component(name, "Class", logic_type);

        match class::Class::find_by_name(name, &mut self.conn) {
            Ok(class) => {
                println!("Found existing class: {class:?}");
            }
            Err(_) => {
                let comp = &self.components[new_component_indice];

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
                        println!("Created Class: {class:?}")
                    }
                    Err(e) => {
                        panic!("Error creating class: {e}")
                    }
                }
            }
        }

        new_component_indice
    }

    fn create_component_assoc(
        &mut self,
        parent_indice: usize,
        parsed_children_indices: ParsedLogicType,
    ) {
        let parent = &self.components[parent_indice];

        match parsed_children_indices {
            ParsedLogicType::AND(children_indices) | ParsedLogicType::OR(children_indices) => {
                for child_i in children_indices {
                    let child = &self.components[child_i];

                    let new_component_assoc = NewComponentAssoc {
                        parent_id: parent.id,
                        child_id: child.id,
                    };
                    match new_component_assoc.create(&mut self.conn) {
                        Ok(new_assoc) => {
                            println!("Created component association: {new_assoc:?}\n\nParent: {parent:?}\nChild: {child:?}\n\n");
                        }
                        Err(e) => {
                            panic!("Error creating component association: {e}")
                        }
                    }
                }
            }
        }
    }

    pub fn gen_test_catalog(&mut self) {
        let catalog = vec![
            (
                Group("CALC 1"),
                OR(vec![Class(("MA 16010", 5)), Class(("MA 16200", 4))]),
            ),
            (
                Group("CNIT CORE"),
                AND(vec![
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
                Group("CNIT TEST"),
                AND(vec![
                    SimpleClass("CNIT 15501"),
                    SimpleClass("CNIT 17600"),
                    SimpleClass("CNIT 24200"),
                    SimpleClass("CNIT 27000"),
                    SimpleClass("CNIT 47100"),
                    SimpleClass("TECH 12000"),
                    SimpleClass("CNIT 45500"),
                    SimpleClass("CNIT 37000"),
                    SimpleClass("CNIT 32000"),
                    SimpleClass("CNIT 47000"),
                    SimpleClass("CNIT 34400"),
                ]),
            ),
            (
                SimpleClass("CNIT 25501"),
                AND(vec![SimpleClass("CNIT 15501")]),
            ),
            (
                SimpleClass("CNIT 24200"),
                AND(vec![SimpleClass("CNIT 17600")]),
            ),
            (
                Class(("CNIT 34010", 1)),
                AND(vec![SimpleClass("CNIT 24200")]),
            ),
            (
                SimpleClass("CNIT 34400"),
                AND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 32000"),
                AND(vec![SimpleClass("TECH 12000")]),
            ),
            (
                SimpleClass("CNIT 37000"),
                AND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 32200"),
                AND(vec![SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 31500"),
                AND(vec![SimpleClass("CNIT 25501")]),
            ),
            (
                SimpleClass("CNIT 34220"),
                OR(vec![SimpleClass("CNIT 34000"), Class(("CNIT 34010", 1))]),
            ),
            (
                SimpleClass("CNIT 47000"),
                AND(vec![SimpleClass("CNIT 32000")]),
            ),
            (
                SimpleClass("CNIT 47100"),
                AND(vec![SimpleClass("CNIT 45500"), SimpleClass("CNIT 37000")]),
            ),
            (
                SimpleClass("CNIT 48000"),
                AND(vec![SimpleClass("CNIT 28000")]),
            ),
            (
                SimpleClass("CNIT 34000"),
                AND(vec![SimpleClass("CNIT 24200")]),
            ),
            (
                SimpleClass("CNIT 27000"),
                AND(vec![SimpleClass("CNIT 17600"), SimpleClass("CNIT 15501")]),
            ),
            (
                SimpleClass("CNIT 28000"),
                AND(vec![SimpleClass("CNIT 18000")]),
            ),
            (
                Group("NETWORK ENGR GROUPED 455 PREREQ"),
                OR(vec![SimpleClass("CNIT 34500"), SimpleClass("CNIT 34400")]),
            ),
            (
                SimpleClass("CNIT 45500"),
                OR(vec![
                    SimpleClass("CNIT 34220"),
                    Group("NETWORK ENGR GROUPED 455 PREREQ"),
                ]),
            ),
            (
                Group("CALC 2"),
                OR(vec![Class(("MA 16020", 5)), Class(("MA 16600", 4))]),
            ),
            (
                Group("Req1"),
                OR(vec![Group("TwoClasses"), Class(("Normal", 3))]),
            ),
            (
                Group("TwoClasses"),
                AND(vec![Class(("ezclass1", 1)), Class(("ezclass2", 2))]),
            ),
            (Group("Req2t1"), AND(vec![Class(("Normal2t1", 3))])),
            (Group("Req2t2"), AND(vec![Class(("Normal2t2", 3))])),
            (SimpleClass("Normal2t2"), AND(vec![Group("TwoClasses")])),
            (SimpleClass("Normal2t1"), AND(vec![SimpleClass("Normal")])),
            (SimpleClass("MA 16020"), AND(vec![SimpleClass("MA 16010")])),
            (SimpleClass("MA 16600"), AND(vec![SimpleClass("MA 16200")])),
            (
                Group("Test1"),
                AND(vec![
                    Class(("1class.1", 3)),
                    Class(("1class.2", 3)),
                    Class(("2class.1", 4)),
                ]),
            ),
            (Group("Test2"), AND(vec![Class(("2class.2", 3))])),
            (
                Group("2class.2 prereqs"),
                OR(vec![SimpleClass("3class.1"), SimpleClass("2class.1")]),
            ),
            (
                SimpleClass("2class.2"),
                OR(vec![SimpleClass("dont pick me"), Group("2class.2 prereqs")]),
            ),
        ];

        let degree_requirements = vec![
            (
                Degree((
                    "TEST1",
                    "TEST MAJOR",
                    "Major",
                    "Tests CALC1 and CALC 2 Requirements",
                )),
                vec![Group("CALC 1"), Group("CALC 2")],
            ),
            (
                Degree((
                    "COMP1",
                    "Complex Test1",
                    "Major",
                    "Tests a complex scenario",
                )),
                vec![Group("Test1"), Group("Test 2")],
            ),
            (
                Degree((
                    "TEST2",
                    "TEST MAJOR",
                    "Major",
                    "Tests CALC1 and MA16020 Requirements",
                )),
                vec![Group("CALC 1"), SimpleClass("MA 16020")],
            ),
            (
                Degree((
                    "TEST3",
                    "Test MAJOR",
                    "Major",
                    "Tests only MA 16020 as a Requirement. THIS SHOULD NOT BUILD",
                )),
                vec![SimpleClass("MA 16020")],
            ),
            (
                Degree((
                    "TEST1REV",
                    "TEST MAJOR",
                    "Major",
                    "Tests CALC1 and CALC2 Requirements",
                )),
                vec![Group("CALC 2"), Group("CALC 1")],
            ),
            (
                Degree((
                    "CNITTEST",
                    "CNIT TEST CLASSES",
                    "Major",
                    "Testing the schedule for lots of prereqs",
                )),
                vec![Group("CNIT TEST")],
            ),
            (
                Degree((
                    "TEST2REV",
                    "TEST 2 REVERSE MAJOR",
                    "Major",
                    "Tests CALC1 and MA16020 Requirements",
                )),
                vec![SimpleClass("MA 16020"), Group("CALC 1")],
            ),
            (
                Degree(("TEST4", "TEST4", "Major", "Tests Req1 and Req2t1")),
                vec![Group("Req1"), Group("Req2t1")],
            ),
            (
                Degree(("TEST5", "TEST5", "Major", "Tests Req1 and Req2t2")),
                vec![Group("Req1"), Group("Req2t2")],
            ),
            (
                Degree(("CSECC", "CSEC Core", "Major", "Tests CNIT CORE")),
                vec![
                    Group("CNIT CORE"),
                    SimpleClass("TECH 12000"),
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
                ],
            ),
        ];

        self.parse_initial_catalog(catalog);
        self.parse_degree_requirements(degree_requirements);
    }

    pub fn gen_full_catalog(&mut self) {
        //first we get parse self.cs
        let catalog = vec![
            (
                Group("CNIT CORE"),
                AND(vec![
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
                OR(vec![SimpleClass("CNIT 37200"), SimpleClass("CNIT 39200")]),
            ),
            (
                Group("CNIT SYS/APP DEV"),
                OR(vec![SimpleClass("CNIT 31500"), SimpleClass("CNIT 32500")]),
            ),
            (
                Group("GENERAL BUSINESS SELECTIVE"),
                OR(vec![
                    SimpleClass("IET 10400"),
                    SimpleClass("IT 10400"),
                    SimpleClass("TLI 11100"),
                    SimpleClass("TLI 15200"),
                ]),
            ),
            (
                Group("UNIV CORE"),
                AND(vec![
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
                AND(vec![SimpleClass("INTERDISC 00000")]),
            ),
            (
                SimpleClass("CNIT 27000"),
                AND(vec![SimpleClass("CNIT 17600"), SimpleClass("CNIT 15501")]),
            ),
            (
                SimpleClass("CNIT 28000"),
                AND(vec![SimpleClass("CNIT 18000")]),
            ),
            (
                SimpleClass("CNIT 25501"),
                AND(vec![SimpleClass("CNIT 15501")]),
            ),
            (
                SimpleClass("CNIT 24200"),
                AND(vec![SimpleClass("CNIT 17600")]),
            ),
            (
                Class(("CNIT 34010", 1)),
                AND(vec![SimpleClass("CNIT 24200")]),
            ),
            (
                SimpleClass("CNIT 34400"),
                AND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 32000"),
                AND(vec![SimpleClass("TECH 12000")]),
            ),
            (
                SimpleClass("CNIT 37000"),
                AND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 32200"),
                AND(vec![SimpleClass("CNIT 27000")]),
            ),
            (
                SimpleClass("CNIT 31500"),
                AND(vec![SimpleClass("CNIT 25501")]),
            ),
            (
                SimpleClass("CNIT 34220"),
                OR(vec![SimpleClass("CNIT 34000"), Class(("CNIT 34010", 1))]),
            ),
            (
                SimpleClass("CNIT 47000"),
                AND(vec![SimpleClass("CNIT 32000")]),
            ),
            (
                SimpleClass("CNIT 48000"),
                AND(vec![SimpleClass("CNIT 28000")]),
            ),
            (
                SimpleClass("CNIT 47100"),
                AND(vec![SimpleClass("CNIT 45500"), SimpleClass("CNIT 37000")]),
            ),
            (
                SimpleClass("CNIT 34000"),
                AND(vec![SimpleClass("CNIT 24200")]),
            ),
            (
                SimpleClass("CNIT 34500"),
                AND(vec![SimpleClass("CNIT 24200"), SimpleClass("CNIT 24000")]),
            ),
            (
                SimpleClass("CNIT 34600"),
                AND(vec![SimpleClass("CNIT 24000"), SimpleClass("CNIT 24200")]),
            ),
            (
                Group("NETWORK ENGR GROUPED 455 PREREQ"),
                OR(vec![SimpleClass("CNIT 34500"), SimpleClass("CNIT 34400")]),
            ),
            (
                SimpleClass("CNIT 45500"),
                AND(vec![
                    SimpleClass("CNIT 34220"),
                    Group("NETWORK ENGR GROUPED 455 PREREQ"),
                ]),
            ),
            (Class(("MA 16020", 5)), AND(vec![Class(("MA 16010", 5))])),
        ];

        self.parse_initial_catalog(catalog);

        let degree_requirements = vec![
            (
                Degree((
                    "CNIT",
                    "Computer and Information Technology",
                    "Major",
                    "Do things in polytech",
                )),
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
                Degree(("CSEC", "Cybersecurity", "Major", "Do hacking things")),
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
                Degree((
                    "NENT",
                    "Network Engineering Technology",
                    "Major",
                    "Do networking and be super happy absolutely",
                )),
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
                Degree((
                    "SAAD",
                    "Systems Analysis and Design",
                    "Major",
                    "dont be SAAD be happy",
                )),
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

        self.parse_degree_requirements(degree_requirements);
    }

    pub fn parse_initial_catalog(&mut self, catalog: Vec<(InstantiationType, LogicalType)>) {
        let mut parsed_assocs: Vec<(usize, ParsedLogicType)> = Vec::new();

        for item in catalog {
            let parent_component = item.0;
            let logical_type = item.1;

            //First we need to make an appropriate parent_component
            let parent_component_indice =
                self.parse_catalog_item(&parent_component, &Some(&logical_type));

            let mut indices: Vec<usize> = Vec::new();
            match &logical_type {
                AND(components) | OR(components) => {
                    self.instantiations_to_indices(&mut indices, components);
                }
            }

            let parsed_logical_type = match &logical_type {
                AND(_) => ParsedLogicType::AND(indices),
                OR(_) => ParsedLogicType::OR(indices),
            };

            parsed_assocs.push((parent_component_indice, parsed_logical_type));
        }

        for association in parsed_assocs {
            self.create_component_assoc(association.0, association.1);
        }
    }

    fn parse_degree_requirements(
        &mut self,
        degree_requirements: Vec<(InstantiationType, Vec<InstantiationType>)>,
    ) {
        //We have to repeat some code because of the borrow checker...
        for item in degree_requirements {
            let degree_in_instantiation = item.0;
            let instantiations = item.1;

            if let Degree(degree_strs) = degree_in_instantiation {
                //actually this is a code but im lazy to change it rn.
                //TODO
                let new_degree = NewDegree {
                    code: Some(degree_strs.0.to_string()),
                    name: Some(degree_strs.1.to_string()),
                    pftype: Some(degree_strs.2.to_string()),
                    description: Some(degree_strs.3.to_string()),
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
                    println!("Created degree to component association: {new_assoc:?}");
                }
                Err(e) => {
                    panic!("Error creating degree to component association: {e}")
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
            indices.push(self.parse_catalog_item(comp, &None));
        }
    }
}
