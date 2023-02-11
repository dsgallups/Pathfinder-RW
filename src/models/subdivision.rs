use crate::schema::subdivisions;
use diesel::PgConnection;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "subdivisions"]
pub struct Subdivision {
    pub id: i32,
    pub name: String,
    pub university: Option<i32>
}
/*
    impl Subdivision {
        
        pub fn create(&self, conn: &mut PgConnection) -> Result<Subdivision, diesel::result::Error> {
            use diesel::RunQueryDsl;

            diesel::insert_into(subdivisions::table)
                .values(self)
                .get_result(conn)

        }
        
    }
*/

#[derive(Serialize, Deserialize)]
pub struct SubdivisionList (pub Vec<Subdivision>);

impl SubdivisionList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::subdivisions::dsl::*;

        let result = subdivisions
            .limit(10)
            .load::<Subdivision>(conn)
            .expect("Error Loading Subdivisions");

        SubdivisionList(result)
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="subdivisions"]
pub struct NewSubdivision {
    pub name: String
}

impl NewSubdivision {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Subdivision, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(subdivisions::table)
            .values(self)
            .get_result(conn)

    }
}