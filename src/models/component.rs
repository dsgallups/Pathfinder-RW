use crate::schema::components;
use diesel::PgConnection;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Component {
    pub id: i32,
    pub name: String,
    pub pftype: String
}

impl Component {
    pub fn find(id: &i32, conn: &mut PgConnection) -> Result<Component, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        components::table.find(id).first(conn)
    }

    pub fn destroy(id: &i32, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::delete(components::table.find(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn update(id: &i32, new_university: &NewComponent, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

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
    pub pftype: Option<String>
}


impl NewComponent {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Component, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(components::table)
            .values(self)
            .get_result(conn)
    }
    pub fn create_class_component(&self, conn: &mut PgConnection) -> Result<Component, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(components::table)
            .values(NewComponent {
                name: self.name.to_owned(),
                pftype: Some("class".to_string())
            })
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComponentList(pub Vec<Component>);

impl ComponentList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::components::dsl::*;

        let result =
            components
                .limit(10)
                .load::<Component>(conn)
                .expect("Error loading components");

        ComponentList(result)
    }
}
