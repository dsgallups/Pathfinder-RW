use crate::schema::universities;
use diesel::PgConnection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct University {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

impl University {
    pub fn find(id: &i32, conn: &mut PgConnection) -> Result<University, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        universities::table.find(id).first(conn)
    }

    pub fn destroy(id: &i32, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::delete(universities::table.find(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn update(id: &i32, new_university: &NewUniversity, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::update(universities::table.find(id))
            .set(new_university)
            .execute(conn)?;
        Ok(())
    }


}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="universities"]
pub struct NewUniversity {
    pub name: Option<String>,
    pub description: Option<String>
}


impl NewUniversity {
    pub fn create(&self, conn: &mut PgConnection) -> Result<University, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(universities::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UniversityList(pub Vec<University>);

impl UniversityList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::universities::dsl::*;

        let result =
            universities
                .limit(10)
                .load::<University>(conn)
                .expect("Error loading universities");

        UniversityList(result)
    }
}
