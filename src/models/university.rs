use crate::schema::universities;

#[derive(Queryable, Serialize, Deserialize)]
pub struct University {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Insertable, Deserialize)]
#[table_name="universities"]
pub struct NewUniversity {
    pub name: Option<String>,
    pub description: Option<String>
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