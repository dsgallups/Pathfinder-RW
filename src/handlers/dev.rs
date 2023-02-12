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

struct CatalogMaker<'a> {
    conn: &'a mut PgConnection
}

impl CatalogMaker<'_> {

    pub fn new(conn: &mut PgConnection) -> Self {
        
        Self { conn }
    }
    pub fn reg(&self, name: &str) -> Component {
        
        //so check if it exists, if not, make it.
        match Component::find()
    }
    pub fn class(name: &str, credits: i32) {

    }
}

enum LogicalType <'a> {
    AND(Vec<&'a str>),
    OR(Vec<&'a str>)
}

use LogicalType::{AND, OR};

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

    push_classes(&mut pg_pool, &mut components);
    push_components(&mut pg_pool, &mut components);

    let c = CatalogMaker::new(&mut pg_pool);
    create_components(
        vec![
            (
                c::reg("CNIT CORE"),
                AND(vec![
                    c::class("CNIT 18000", 3),
                    c::class("CNIT 15501", 3),
                    c::class("CNIT 17600", 3),
                    c::class("CNIT 24200", 3)
                ]),
                "requirement"
            )
        ]
    )
    /*
    create_components(
        &mut pg_pool,
        vec![
            (
                c::reg("CNIT CORE"), 
                AND(vec![
                    c::class("CNIT 18000", 3),
                    c::class("CNIT 15501", 3),
                    c::class("CNIT 17600", 3),
                    c::class("CNIT 24200", 3),
                ]),
                "requirement"
            )
            (
                c::reg("CNIT 18000"),
                AND(vec![
                    c::reg("CNIT 15501"),
                    c::class("CNIT 24500", 3)
                ]),
                "requisite"
            )
        ]
    )
    
    */

    //populate our logical components
    parse_component_associations(
        &mut pg_pool,
        &mut components,
        "requirement",
        vec![
        ("CNIT CORE", AND(vec![
            "CNIT 18000",
            "CNIT 15501",
            "CNIT 17600",
            "CNIT 24200",
            "CNIT 25501",
            "CNIT 27000",
            "CNIT 27200",
            "CNIT 28000",
            "CNIT 32000",
            "CNIT 48000"
        ])),
        ("CNIT DB PROGRAMMING", OR(vec![
            "CNIT 37200",
            "CNIT 39200"
        ])),
        ("CNIT SYS/APP DEV", OR(vec![
            "CNIT 31500",
            "CNIT 32500"
        ])),
        ("GENERAL BUSINESS SELECTIVE", OR(vec![
            "IET 10400",
            "IT 10400",
            "TLI 11100",
            "TLI 15200"
        ])),
        ("UNIV CORE", AND(vec![
            "SCLA 10100",
            "SCLA 10200",
            "TECH 12000",
            "MA 16010",
            "MA 16020",
            "OLS 25200",
            "TLI 11200",
            "PHIL 15000",
            "COMSEL 00000",
            "ECONSEL 00000",
            "SCISEL 00000",
            "LABSCISEL 00000",
            "ACCSEL 00000",
            "STATSEL 00000",
            "SPEAKSEL 00000",
            "WRITINGSEL 00000",
            "HUMSEL 00000",
            "BEHAVSCISEL 00000",
            "FOUNDSEL 00000"
        ])),
        ("CNIT/SAAD INTERDISC", AND(vec![
            "INTERDISC 00000",
            "INTERDISC 00000"
        ])),
        ("CSEC INTERDISC", AND(vec![
            "INTERDISC 00000",
            "INTERDISC 00000"
        ])),
        ("NENT INTERDISC", AND(vec![
            "INTERDISC 00000",
            "INTERDISC 00000"
        ])),
        ("CNIT IT SELECTIVES", AND(vec![
            "ITSEL 00000",
            "ITSEL 00000"
        ])),
        ("NENT IT SELECTIVES", AND(vec![
            "ITSEL 00000",
            "ITSEL 00000"
        ]))
    ]);


    parse_component_associations(
        &mut pg_pool,
        &mut components,
        "requisite",
        vec![
            ("CNIT 27000", AND(vec![
                "CNIT 17600",
                "CNIT 15501"
            ])),
            ("CNIT 27200", AND(vec![
                "CNIT 15501",
                "CNIT 18000"
            ])),
            ("CNIT 28000", AND(vec![
                "CNIT 18000"
            ])),
            ("CNIT 25501", AND(vec![
                "CNIT 15501"
            ])),
            ("CNIT 24200", AND(vec![
                "CNIT 17600"
            ])),
            ("CNIT 34010", AND(vec![
                "CNIT 24200"
            ])),
            ("CNIT 34400", AND(vec![
                "CNIT 24200",
                "CNIT 27000"
            ])),
            ("CNIT 32000", AND(vec![
                "TECH 12000"
            ])),
            ("CNIT 37000", AND(vec![
                "CNIT 24200",
                "CNIT 27000"
            ])),
            ("CNIT 32200", AND(vec![
                "CNIT 27000"
            ])),
            ("CNIT 31500", AND(vec![
                "CNIT 25501"
            ])),
            ("CNIT 34220", OR(vec![
                "CNIT 34000",
                "CNIT 34010"
            ])),
            ("CNIT 47000", AND(vec![
                "CNIT 45500",
                "CNIT 32000"
            ])),
            ("CNIT 48000", AND(vec![
                "CNIT 28000"
            ])),
            ("CNIT 47100", AND(vec![
                "CNIT 45500",
                "CNIT 37000"
            ])),
            ("CNIT 34000", AND(vec![
                "CNIT 24200"
            ])),
            ("CNIT 34500", AND(vec![
                "CNIT 24200",
                "CNIT 24000"
            ])),
            ("CNIT 34600", AND(vec![
                "CNIT 24000",
                "CNIT 24200"
            ]))
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

fn push_classes(conn: &mut PgConnection, class_components: &mut Vec<Component>) {

    let classes = vec![("CNIT 15501", 3),
        ("CNIT 17600", 3),
        ("CNIT 18000", 3),
        ("CNIT 24200", 3),
        ("CNIT 25501", 3),
        ("CNIT 27000", 3),
        ("CNIT 27200", 3),
        ("CNIT 28000", 3),
        ("CNIT 32000", 3),
        ("CNIT 48000", 3),
        ("CNIT 37200", 3),
        ("CNIT 39200", 3),
        ("CNIT 31500", 3),
        ("CNIT 32500", 3),
        ("CNIT 38000", 3),
        ("CGT 25600", 3),
        ("CNIT 32200", 3),
        ("CNIT 24000", 3),
        ("CNIT 34500", 3),
        ("CNIT 34600", 3),
        ("CNIT 34400", 3),
        ("CNIT 34000", 3),
        ("CNIT 34010", 1),
        ("CNIT 34210", 1),
        ("CNIT 34220", 2),
        ("CNIT 37000", 3),
        ("CNIT 42000", 3),
        ("CNIT 42200", 3),
        ("CNIT 45500", 3),
        ("CNIT 47000", 3),
        ("CNIT 47100", 3),
        ("ITSEL 00000", 3),
        ("CSECSEL 00000", 3),
        ("SCLA 10100", 3),
        ("SCLA 10200", 3),
        ("TECH 12000", 3),
        ("MA 16010", 4),
        ("MA 16020", 4),
        ("OLS 25200", 3),
        ("TLI 11200", 3),
        ("COMSEL 00000", 3),
        ("ECONSEL 00000", 3),
        ("SCISEL 00000", 4),
        ("LABSCISEL 00000", 4),
        ("ACCSEL 00000", 3),
        ("STATSEL 00000", 3),
        ("SPEAKSEL 00000", 3),
        ("WRITINGSEL 00000", 3),
        ("INTERDISC 00000", 3),
        ("IET 10400", 3),
        ("IT 10400", 3),
        ("TLI 11100", 3),
        ("TLI 15200", 3),
        ("PHIL 15000", 3),
        ("HUMSEL 00000", 3),
        ("BEHAVSCISEL 00000", 3),
        ("FOUNDSEL 00000", 3),
        ("FREE 00000", 3),
        ("SAADSEL 00000", 3)
    ];

    for class in classes {
        //Make the components first
        let class_component = create_class_component(conn, class.0);
        //make the classes in the class table

        let _db_class = create_class_from_component(conn, &class_component, class);
        class_components.push(class_component);
    }


}

fn create_class_component (conn: &mut PgConnection, name: &str) -> Component {
    let new_component = NewComponent {
        name: Some(name.to_string()),
        pftype: Some("class".to_string())
    };

    match new_component.create_class_component(conn) {
        Ok(comp) => {
            println!("Created Class Component: {:?}", comp);
            return comp;
        }
        Err(e) => {panic!("Error creating class component: {}", e)}
    }
}

fn create_class_from_component(conn: &mut PgConnection, component: &Component, class: (&str, i32)) -> Class {

    let new_simple_class = SimpleClass {
        name: Some(class.0.to_string()),
        credits: Some(class.1),
        component_id: Some(component.id)
    };

    match new_simple_class.create(conn) {
        Ok(class) => {
            println!("Created Class: {:?}", &class);
            return class;
        }
        Err(e) => {panic!("Error creating class: {}", e)}
    }


}

fn push_components(conn: &mut PgPooledConnection, components: &mut Vec<Component>) {
    let component_strs = vec![
        "CNIT CORE",
        "CNIT DB PROGRAMMING",
        "CNIT SYS/APP DEV",
        "GENERAL BUSINESS SELECTIVE",
        "UNIV CORE",
        "CNIT/SAAD INTERDISC",
        "CSEC INTERDISC",
        "NENT INTERDISC",
        "CNIT IT SELECTIVES",
        "NENT IT SELECTIVES",
        "SAAD IT SELECTIVES",
        "CSEC SELECTIVES",
        "SAAD SELECTIVES"
    ];

    for comp in component_strs {
        let new_component = NewComponent {
            name: Some(comp.to_string()),
            pftype: Some("logical".to_string())
        };
        match new_component.create(conn) {
            Ok(comp) => {
                println!("Created {:?}", &comp);
                components.push(comp);
            }
            Err(e) => {panic!("Error creating component: {}", e)}
        }
    }

}


fn parse_component_associations(
    conn: &mut PgConnection,
    components: &mut Vec<Component>,
    assoc_type: &str,
    values: Vec<(&str, LogicalType)>
) {

    for val in values {
        //we make component to component deals
        let parent_str = val.0;
        let children = val.1;

        match children {
            LogicalType::AND(sub_components) => {
                create_component_assoc(
                    conn,
                    components,
                    parent_str, 
                    sub_components, 
                    "AND", 
                    assoc_type
                );
            }
            LogicalType::OR(sub_components) => {
                create_component_assoc(
                    conn,
                    components,
                    parent_str,
                    sub_components,
                    "OR",
                    assoc_type
                );
            }
        }

        

    }

}

fn create_component_assoc(
    conn: &mut PgConnection,
    components: &Vec<Component>,
    parent_str: &str, 
    children_strs: Vec<&str>, 
    logic_type: &str,
    assoc_type: &str
) {

    let parent_i = components
    .iter()
    .position(|v| v.name.eq(parent_str))
    .unwrap();

    let comp = &components[parent_i];

    for child_str in children_strs {

        let child_i = components
            .iter()
            .position(|v| v.name.eq(child_str))
            .unwrap();
        
        let child_component = &components[child_i];

        let new_component_assoc = NewComponentAssoc {
            parent_id: comp.id,
            child_id: child_component.id,
            relationship_type: assoc_type.to_string(),
            logic_type: logic_type.to_string()
        };
        match new_component_assoc.create(conn) {
            Ok(new_assoc) => {
                println!("Created Component Association: {:?}", new_assoc);

            }
            Err(e) => {panic!("Error Creating Component Association: {}", e)}
        }
    }
}