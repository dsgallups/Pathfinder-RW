use crate::schema::components;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct Component {
    pub id: i32,
    pub name: String,
    pub pftype: String,
    pub logic_type: Option<String>,
}

impl Component {
    pub fn find(id: &i32, conn: &mut PgConnection) -> Result<Component, diesel::result::Error> {
        components::table.find(id).first(conn)
    }

    pub fn find_by_name(
        name: &str,
        conn: &mut PgConnection,
    ) -> Result<Component, diesel::result::Error> {
        components::table
            .filter(components::name.eq(name))
            .first(conn)
    }

    pub fn destroy(id: &i32, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(components::table.find(id)).execute(conn)?;
        Ok(())
    }

    pub fn update(
        id: &i32,
        new_university: &NewComponent,
        conn: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(components::table.find(id))
            .set(new_university)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Debug, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = components)]
pub struct NewComponent {
    pub name: Option<String>,
    pub pftype: Option<String>,
    pub logic_type: Option<String>,
}

impl NewComponent {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Component, diesel::result::Error> {
        diesel::insert_into(components::table)
            .values(self)
            .get_result(conn)
    }
    pub fn create_class_component(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Component, diesel::result::Error> {
        diesel::insert_into(components::table)
            .values(NewComponent {
                name: self.name.to_owned(),
                pftype: Some("class".to_string()),
                logic_type: self.logic_type.to_owned(),
            })
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComponentList(pub Vec<Component>);

impl ComponentList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use crate::schema::components::dsl::*;

        let result = components
            .limit(10)
            .load::<Component>(conn)
            .expect("Error loading components");

        ComponentList(result)
    }
}
