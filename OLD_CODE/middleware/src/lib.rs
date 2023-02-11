use std::str;
/**
 * First, drop all the tables in our DB.
 * Then, rebuild all the tables. 
 * 
 */
use pf_diesel::dev::*;
pub fn rebuild_catalog() {
    let reset_output = reset_all_tables();
    println!("-------------------------------------\nTables Reset! Output:\n\n{}\n-------------------------------------", str::from_utf8(&reset_output.stdout).unwrap());

    insert_catalog();
}