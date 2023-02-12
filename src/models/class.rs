use crate::schema::classes;
use diesel::PgConnection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub credits: Option<i32>,
    pub pftype: String,
    pub subject: Option<String>,
    pub course_no: Option<String>,
    pub options: Option<String>,
    pub component_id: Option<i32>
}

impl Class {
    pub fn find(id: &i32, conn: &mut PgConnection) -> Result<Class, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        classes::table.find(id).first(conn)
    }

    pub fn destroy(id: &i32, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::delete(classes::table.find(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn update(id: &i32, new_university: &NewClass, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::update(classes::table.find(id))
            .set(new_university)
            .execute(conn)?;
        Ok(())
    }


}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = classes)]
pub struct NewClass {
    pub name: Option<String>,
    pub description: Option<String>,
    pub credits: Option<i32>,
    pub pftype: Option<String>,
    pub subject: Option<String>,
    pub course_no: Option<String>,
    pub options: Option<String>,
    pub component_id: Option<i32>
}


impl NewClass {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Class, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(classes::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassList(pub Vec<Class>);

impl ClassList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::classes::dsl::*;

        let result =
            classes
                .limit(10)
                .load::<Class>(conn)
                .expect("Error loading classes");

        ClassList(result)
    }
}
