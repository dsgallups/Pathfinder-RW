use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse 
};
use serde_json::json;

use crate::{models::{
        component::{
            Component,
            NewComponent
        },
        class::{
            Class,
            SimpleClass
        },
        associations::{
            NewComponentAssoc
        }
}, db_connection::PgPooledConnection};

use crate::db_connection::{ PgPool };
use diesel::PgConnection;
use crate::handlers::pg_pool_handler;
use std::str;

use std::process::{Command, Output};

enum LogicalType {
    AND(Vec<usize>),
    OR(Vec<usize>)
}

use LogicalType::{AND, OR};

struct CatalogMaker<'a> {
    conn: &'a mut PgConnection,
    components: Vec<Component>
}

impl CatalogMaker<'_> {

    pub fn new(conn: &mut PgConnection) -> Self {
        
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
        if let Ok(component) = Component::find_by_name(name, self.conn) {
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

        match new_component.create(self.conn) {
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

        match new_component.create_class_component(self.conn) {
            Ok(comp) => {
                println!("Created Class Component: {:?}", comp);
                self.components.push(comp);
                index = self.components.len() - 1;
            }
            Err(e) => {panic!("Error creating class components: {}", e)}
        }

        //create the class now
        let new_simple_class = SimpleClass {
            name: Some(name.to_string()),
            credits: Some(credits),
            component_id: Some(self.components[index].id)
        };

        match new_simple_class.create(self.conn) {
            Ok(class) => {
                println!("Created Class: {:?}", &class);
                return index
            }
            Err(e) => {panic!("Error creating class: {}", e)}
        }

    }

    pub fn parse_assocs(&self, assocs: Vec<(usize, LogicalType, &str)>) {
        for assoc in assocs {
            let parent_index = assoc.0;
            let logic_type = assoc.1;
            let association_type = assoc.2;

            let parent = &self.components[parent_index];

            match logic_type {
                AND(children) => {
                    self.create_component_assoc(
                        parent,
                        children,
                        "AND",
                        association_type
                    )
                },
                OR(children) => {
                    self.create_component_assoc(
                        parent,
                        children,
                        "OR",
                        association_type
                    )
                }
            }            
        }
    }
    fn create_component_assoc(
        &self, 
        parent: &Component, 
        children: Vec<usize>,
        logic_type: &str,
        association_type: &str
    ) {
        for child_i in children {
            let child = self.components[child_i];

            let new_component_assoc = NewComponentAssoc {
                parent_id: parent.id,
                child_id: child.id,
                association_type: association_type.to_string(),
                logic_type: logic_type.to_string()
            };
            match new_component_assoc.create(self.conn) {
                Ok(new_assoc) => {
                    println!("Created Component Association: {:?}", new_assoc);
                }
                Err(e) => {panic!("Error Creating Component Association: {}", e)}
            }
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

    let mut c = CatalogMaker::new(&mut pg_pool);
    c.parse_assocs(
        vec![
            (
                c.reg("CNIT CORE"),
                AND(vec![
                    c.c("CNIT 18000"),
                    c.c("CNIT 15501"),
                    c.c("CNIT 17600"),
                    c.c("CNIT 24200"),
                    c.c("CNIT 25501"),
                    c.c("CNIT 27000"),
                    c.c("CNIT 27200"),
                    c.c("CNIT 28000"),
                    c.c("CNIT 32000"),
                    c.c("CNIT 48000")
                ]),
                "requirement"
            ),
            (
                c.reg("CNIT DB PROGRAMMING"),
                OR(vec![
                    c.c("CNIT 37200"),
                    c.c("CNIT 39200")
                ]),
                "requirement"
            ),
            (
                c.reg("CNIT SYS/APP DEV"),
                OR(vec![
                    c.c("CNIT 31500"),
                    c.c("CNIT 32500")
                ]),
                "requirement"
            ),
            (
                c.reg("GENERAL BUSINESS SELECTIVE"),
                OR(vec![
                    c.c("IET 10400"),
                    c.c("IT 10400"),
                    c.c("TLI 11100"),
                    c.c("TLI 15200")
                ]),
                "requirement"
            ),
            (
                c.reg("UNIV CORE"),
                AND(vec![
                    c.c("SCLA 10100"),
                    c.c("SCLA 10200"),
                    c.c("TECH 12000"),
                    c.c("MA 16010"),
                    c.c("MA 16020"),
                    c.c("OLS 25200"),
                    c.c("TLI 11200"),
                    c.c("PHIL 15000"),
                    c.c("COMSEL 00000"),
                    c.c("ECONSEL 00000"),
                    c.c("SCISEL 00000"),
                    c.c("LABSCISEL 00000"),
                    c.c("ACCSEL 00000"),
                    c.c("STATSEL 00000"),
                    c.c("SPEAKSEL 00000"),
                    c.c("WRITINGSEL 00000"),
                    c.c("HUMSEL 00000"),
                    c.c("BEHAVSCISEL 00000"),
                    c.c("FOUNDSEL 00000")
                ]),
                "requirement"
            ),
            (
                c.reg("CNIT/SAAD INTERDISC"), 
                AND(vec![
                    c.c("INTERDISC 00000")
                ]),
                "requirement"
            ),
            (
                c.c("CNIT 27000"),
                AND(vec![
                    c.c("CNIT 17600"),
                    c.c("CNIT 15501")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 28000"),
                AND(vec![
                    c.c("CNIT 18000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 25501"),
                AND(vec![
                    c.c("CNIT 15501")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 24200"),
                AND(vec![
                    c.c("CNIT 17600")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 34010"),
                AND(vec![
                    c.c("CNIT 24200")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 34400"),
                AND(vec![
                    c.c("CNIT 24200"),
                    c.c("CNIT 27000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 32000"),
                AND(vec![
                    c.c("TECH 12000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 37000"),
                AND(vec![
                    c.c("CNIT 24200"),
                    c.c("CNIT 27000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 32200"),
                AND(vec![
                    c.c("CNIT 27000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 31500"),
                AND(vec![
                    c.c("CNIT 25501")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 34220"),
                OR(vec![
                    c.c("CNIT 34000"),
                    c.c("CNIT 34010")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 47000"),
                AND(vec![
                    c.c("CNIT 32000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 48000"),
                AND(vec![
                    c.c("CNIT 28000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 47100"),
                AND(vec![
                    c.c("CNIT 45500"),
                    c.c("CNIT 37000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 34000"),
                AND(vec![
                    c.c("CNIT 24200")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 34500"),
                AND(vec![
                    c.c("CNIT 24200"),
                    c.c("CNIT 24000")
                ]),
                "requisite"
            ),
            (
                c.c("CNIT 34600"),
                AND(vec![
                    c.c("CNIT 24000"),
                    c.c("CNIT 24200")
                ]),
                "requisite"
            ),
        ]
    );

    return HttpResponse::Ok().json(json!({"name": "hi"}));
}

fn reset_all_tables() -> Output {
    Command::new("diesel")
        .arg("database")
        .arg("reset")
        .output()
        .expect("Failed to reset tables. (diesel/src/dev.rs)")
}



