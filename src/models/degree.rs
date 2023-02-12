use crate::schema::degrees;
use diesel::PgConnection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Degree {
    pub id: i32,
    pub name: String,
    pub pftype: String,
    pub code: String,
    pub description: Option<String>,
    pub subdivision: Option<i32>
}

impl Degree {
    pub fn find(id: &i32, conn: &mut PgConnection) -> Result<Degree, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        degrees::table.find(id).first(conn)
    }

    pub fn destroy(id: &i32, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::delete(degrees::table.find(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn update(id: &i32, new_degree: &NewDegree, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        diesel::update(degrees::table.find(id))
            .set(new_degree)
            .execute(conn)?;
        Ok(())
    }


}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = degrees)]
pub struct NewDegree {
    pub name: Option<String>,
    pub description: Option<String>
}


impl NewDegree {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Degree, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(degrees::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DegreeList(pub Vec<Degree>);

impl DegreeList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::degrees::dsl::*;

        let result =
            degrees
                .limit(10)
                .load::<Degree>(conn)
                .expect("Error loading degrees");

        DegreeList(result)
    }
}
