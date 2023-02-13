use crate::schema::{
    components_to_components,
    degrees_to_components,
    components
};

use crate::models::{
    component::{
        Component
    },
    degree::Degree
};
use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;


#[derive(Debug, Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Component, foreign_key=child_id))]
#[diesel(table_name=components_to_components)]
pub struct ComponentToComponent {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
    pub logic_type: String,
    pub association_type: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=components_to_components)]
pub struct NewComponentAssoc {
    pub parent_id: i32,
    pub child_id: i32,
    pub association_type: String,
    pub logic_type: String
}
impl NewComponentAssoc {
    pub fn create(&self, conn: &mut PgConnection) -> Result<ComponentToComponent, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(components_to_components::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Debug, Identifiable, PartialEq, Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Degree))]
#[diesel(belongs_to(Component))]
#[diesel(table_name=degrees_to_components)]
pub struct DegreeToComponent {
    pub id: i32,
    pub degree_id: i32,
    pub component_id: i32
}

impl DegreeToComponent {
    pub fn get_components(degree: &Degree, conn: &mut PgConnection) -> Result<Vec<Component>, diesel::result::Error> {
        DegreeToComponent::belonging_to(degree)
            .inner_join(components::table)
            .select(components::all_columns)
            .load::<Component>(conn)
    }
}


#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=degrees_to_components)]
pub struct NewDegreeToComponent {
    pub degree_id: i32,
    pub component_id: i32
}

impl NewDegreeToComponent {
    pub fn create(&self, conn: &mut PgConnection) -> Result<DegreeToComponent, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(degrees_to_components::table)
            .values(self)
            .get_result(conn)
    }
}
