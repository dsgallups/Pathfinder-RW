use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse,
};
use serde_json::json;

use crate::{models::{
        component::{
            Component,
            NewComponent
        },
        associations::{
            NewComponentAssoc
        }
}};

use crate::db_connection::{ PgPool };
use diesel::{PgConnection, r2d2::{PooledConnection, ConnectionManager}};
use crate::handlers::pg_pool_handler;
use std::str;

use std::process::{Command, Output};

enum LogicalType<'a> {
    AND(Vec<InstatiationType<'a>>),
    OR(Vec<InstatiationType<'a>>)
}
enum ParsedLogicType {
    AND(Vec<usize>),
    OR(Vec<usize>)
}

enum InstatiationType<'a> {
   SimpleClass(&'a str),
   Class((&'a str, i32)),
   Reg(&'a str),
   
}

use LogicalType::{AND, OR};
use InstatiationType::{SimpleClass, Class, Reg};

struct CatalogMaker {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    components: Vec<Component>
}

impl CatalogMaker {

    pub fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        
        Self { conn, components: Vec::new() }
    }

    fn check_for_component(&mut self, name: &str) -> Option<usize> {
        let in_self = self.components
        .iter()
        .position(|v| v.name.eq(name));

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
    pub fn reg(&mut self, name: &str) -> usize {
        
        if let Some(index) = self.check_for_component(name) {
            return index;
        }

        let new_component = NewComponent {
            name: Some(name.to_string()),
            pftype: Some("logical".to_string())
        };

        match new_component.create(&mut self.conn) {
            Ok(component) => {
                self.components.push(component);
                return self.components.len() - 1;
            }
            Err(e) => {panic!("Error: {}", e)}
        }
        
    }
    pub fn c(&mut self, name: &str) -> usize {
        self.class(name, 3)
    }
    pub fn class(&mut self, name: &str, credits: i32) -> usize {

        //Make a class, then make a component for the class and return its index
        //however, first check for its existence
        let mut index = usize::MAX;

        if let Some(index) = self.check_for_component(name) {
            return index;
        }

        let new_component = NewComponent {
            name: Some(name.to_string()),
            pftype: Some("class".to_string())
        };

        match new_component.create_class_component(&mut self.conn) {
            Ok(comp) => {
                println!("Created Class Component: {:?}", comp);
                self.components.push(comp);
                index = self.components.len() - 1;
            }
            Err(e) => {panic!("Error creating class components: {}", e)}
        }

        index

    }

    fn create_component_assoc(
        &mut self, 
        parent_indice: usize, 
        parsed_children: ParsedLogicType,
        association_type: &str
    ) {
        let parent = &self.components[parent_indice];

        let logic_type = match parsed_children {
            ParsedLogicType::AND(_) => {
                "AND"
            },
            ParsedLogicType::OR(_) => {
                "OR"
            }
        };

        match parsed_children {
            ParsedLogicType::AND(children_indices) | 
            ParsedLogicType::OR(children_indices) => {
                for child_i in children_indices {
                    let child = &self.components[child_i];
        
                    let new_component_assoc = NewComponentAssoc {
                        parent_id: parent.id,
                        child_id: child.id,
                        association_type: association_type.to_string(),
                        logic_type: logic_type.to_string()
                    };
                    match new_component_assoc.create(&mut self.conn) {
                        Ok(new_assoc) => {
                            println!("Created Component Association: {:?}", new_assoc);
                        }
                        Err(e) => {panic!("Error Creating Component Association: {}", e)}
                    }
                }
            }
        }

    }

    fn comp_list(&mut self, logic_type: LogicalType) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();

        ret

    }


    fn get_catalog(&mut self) {
        //first we get parse self.cs
        let catalog = vec![
            (
                Reg("CNIT CORE"),
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
                    SimpleClass("CNIT 48000")
                ]),
                "requirement"
            ),
            (
                Reg("CNIT DB PROGRAMMING"),
                OR(vec![
                    SimpleClass("CNIT 37200"),
                    SimpleClass("CNIT 39200")
                ]),
                "requirement"
            ),
            (
                Reg("CNIT SYS/APP DEV"),
                OR(vec![
                    SimpleClass("CNIT 31500"),
                    SimpleClass("CNIT 32500")
                ]),
                "requirement"
            ),
            (
                Reg("GENERAL BUSINESS SELECTIVE"),
                OR(vec![
                    SimpleClass("IET 10400"),
                    SimpleClass("IT 10400"),
                    SimpleClass("TLI 11100"),
                    SimpleClass("TLI 15200")
                ]),
                "requirement"
            ),
            (
                Reg("UNIV CORE"),
                AND(vec![
                    SimpleClass("SCLA 10100"),
                    SimpleClass("SCLA 10200"),
                    SimpleClass("TECH 12000"),
                    SimpleClass("MA 16010"),
                    SimpleClass("MA 16020"),
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
                    SimpleClass("FOUNDSEL 00000")
                ]),
                "requirement"
            ),
            (
                Reg("CNIT/SAAD INTERDISC"), 
                AND(vec![
                    SimpleClass("INTERDISC 00000")
                ]),
                "requirement"
            ),
            (
                SimpleClass("CNIT 27000"),
                AND(vec![
                    SimpleClass("CNIT 17600"),
                    SimpleClass("CNIT 15501")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 28000"),
                AND(vec![
                    SimpleClass("CNIT 18000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 25501"),
                AND(vec![
                    SimpleClass("CNIT 15501")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 24200"),
                AND(vec![
                    SimpleClass("CNIT 17600")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 34010"),
                AND(vec![
                    SimpleClass("CNIT 24200")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 34400"),
                AND(vec![
                    SimpleClass("CNIT 24200"),
                    SimpleClass("CNIT 27000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 32000"),
                AND(vec![
                    SimpleClass("TECH 12000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 37000"),
                AND(vec![
                    SimpleClass("CNIT 24200"),
                    SimpleClass("CNIT 27000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 32200"),
                AND(vec![
                    SimpleClass("CNIT 27000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 31500"),
                AND(vec![
                    SimpleClass("CNIT 25501")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 34220"),
                OR(vec![
                    SimpleClass("CNIT 34000"),
                    SimpleClass("CNIT 34010")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 47000"),
                AND(vec![
                    SimpleClass("CNIT 32000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 48000"),
                AND(vec![
                    SimpleClass("CNIT 28000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 47100"),
                AND(vec![
                    SimpleClass("CNIT 45500"),
                    SimpleClass("CNIT 37000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 34000"),
                AND(vec![
                    SimpleClass("CNIT 24200")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 34500"),
                AND(vec![
                    SimpleClass("CNIT 24200"),
                    SimpleClass("CNIT 24000")
                ]),
                "requisite"
            ),
            (
                SimpleClass("CNIT 34600"),
                AND(vec![
                    SimpleClass("CNIT 24000"),
                    SimpleClass("CNIT 24200")
                ]),
                "requisite"
            ),
        ];

        let mut parsed_assocs: Vec<(usize, ParsedLogicType, &str)> = Vec::new();

        for item in catalog {
            let parent_component = item.0;
            let logical_type = item.1;
            let association_type = item.2;

            //so first we will parse the logicaltype into parsed type
            let mut indices: Vec<usize> = Vec::new();
            match &logical_type {
                AND(components) | OR(components) => {
                    for comp in components {
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
                    
                        }
                    }
                }
            }

            let parsed_logical_type = match &logical_type {
                AND(_) => {ParsedLogicType::AND(indices)}
                OR(_) => {ParsedLogicType::OR(indices)}
            };

            //now we can pass this (hopefully) to parse_assocs
            let parent_component_indice = match &parent_component {
                SimpleClass(c) => {
                    self.c(c)
                }
                Class(c) => {
                    self.class(c.0, c.1)
                }
                Reg(c) => {
                    self.reg(c)
                }
            };
            
            parsed_assocs.push((parent_component_indice, parsed_logical_type, association_type));
        }

        for association in parsed_assocs {
            self.create_component_assoc(association.0, association.1, association.2);
        }
    

    }
}



pub async fn reset_and_pop_db(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {

    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => {p}
        Err(e) => {
            return e;
        }
    };

    let reset_output = reset_all_tables();
    println!("-------------------------------------\nTables Reset! Output:\n\n{}\n-------------------------------------", str::from_utf8(&reset_output.stdout).unwrap());

    let mut components: Vec<Component> = Vec::new();

    let mut c = CatalogMaker::new(pg_pool);
    

    return HttpResponse::Ok().json(json!({"name": "hi"}));
}

fn reset_all_tables() -> Output {
    Command::new("diesel")
        .arg("database")
        .arg("reset")
        .output()
        .expect("Failed to reset tables. (diesel/src/dev.rs)")
}



