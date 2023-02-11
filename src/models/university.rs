use crate::schema::universities;

#[derive(Queryable, Serialize, Deserialize)]
pub struct University {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

impl University {
    pub fn find(id: &i32) -> Result<University, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let mut conn = establish_connection();

        universities::table.find(id).first(&mut conn)
    }

    pub fn destroy(id: &i32) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::universities::dsl;
        use crate::db_connection::establish_connection;

        let mut conn = establish_connection();

        diesel::delete(universities::table.find(id))
            .execute(&mut conn)?;

        Ok(())
    }
}

#[derive(Insertable, Deserialize)]
#[table_name="universities"]
pub struct NewUniversity {
    pub name: Option<String>,
    pub description: Option<String>
}


impl NewUniversity {
    pub fn create(&self) -> Result<University, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let mut conn = establish_connection();

        diesel::insert_into(universities::table)
            .values(self)
            .get_result(&mut conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UniversityList(pub Vec<University>);

impl UniversityList {
    pub fn list() -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::universities::dsl::*;
        use crate::db_connection::establish_connection;

        let mut conn = establish_connection();

        let result =
            universities
                .limit(10)
                .load::<University>(&mut conn)
                .expect("Error loading universities");

        UniversityList(result)
    }
}
