use crate::schema::{
    components_to_components
};

use crate::models::{
    component::{
        Component
    }
};
use diesel::PgConnection;

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