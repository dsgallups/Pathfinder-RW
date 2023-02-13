use crate::schema::{
    components_to_components,
    degrees_to_components,
    components,
    degrees
};

use crate::models::{
    component::{
        Component
    },
    degree::Degree
};
use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::pg::Pg;
use diesel::prelude::*;


#[derive(Debug, Identifiable, PartialEq, Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Component, foreign_key=parent_id))]
#[diesel(table_name=components_to_components)]
pub struct ComponentToComponent {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
    pub logic_type: String,
    pub association_type: String
}

impl ComponentToComponent {
    pub fn get_assoc_from_parent(&self, component: &Component, conn: &mut PgConnection) -> Result<Vec<ComponentToComponent>, diesel::result::Error> {
        use crate::schema::components_to_components::dsl::*;

        let res = components_to_components
            .filter(parent_id.eq(component.id))
            .load::<ComponentToComponent>(conn);

        res
    }

    pub fn get_assoc_from_child(&self, component: &Component, conn: &mut PgConnection) -> Result<Vec<ComponentToComponent>, diesel::result::Error> {
        use crate::schema::components_to_components::dsl::*;

        let res = components_to_components
            .filter(child_id.eq(component.id))
            .load::<ComponentToComponent>(conn);

        res
    }

    pub fn get_children(&self, component: &Component, conn: &mut PgConnection) -> Result<Vec<Component>, diesel::result::Error> {
        use crate::schema::components::dsl::*;

        let assoc = self.get_assoc_from_parent(component, conn)?;

        let child_ids = assoc
            .into_iter()
            .map(|child| child.id)
            .collect::<Vec<i32>>();

        let res = components
            .filter(id.eq_any(child_ids))
            .load::<Component>(conn);
        
        res

    }

    pub fn get_parents(&self, component: &Component, conn: &mut PgConnection) -> Result<Vec<Component>, diesel::result::Error> {
        use crate::schema::components::dsl::*;

        let assoc = self.get_assoc_from_child(component, conn)?;

        let parent_ids = assoc
            .into_iter()
            .map(|child| child.id)
            .collect::<Vec<i32>>();

        let res = components
            .filter(id.eq_any(parent_ids))
            .load::<Component>(conn);
        
        res
        
    }


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
    pub fn get_degrees(component: &Component, conn: &mut PgConnection) -> Result<Vec<Degree>, diesel::result::Error> {
        DegreeToComponent::belonging_to(component)
            .inner_join(degrees::table)
            .select(degrees::all_columns)
            .load::<Degree>(conn)
    }
    pub fn get_component_ids(degree: &Degree, conn: &mut PgConnection) -> Result<Vec<DegreeToComponent>, diesel::result::Error> {
        DegreeToComponent::belonging_to(degree)
            .load::<DegreeToComponent>(conn)
    }
    pub fn get_degree_ids(component: &Component, conn: &mut PgConnection) -> Result<Vec<DegreeToComponent>, diesel::result::Error> {
        DegreeToComponent::belonging_to(component)
            .load::<DegreeToComponent>(conn)
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
        diesel::insert_into(degrees_to_components::table)
            .values(self)
            .get_result(conn)
    }
}
