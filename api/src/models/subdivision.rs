use crate::models::university::University;
use crate::schema::subdivisions;
use diesel::PgConnection;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(University))]
#[diesel(table_name = subdivisions)]
pub struct Subdivision {
    pub id: i32,
    pub name: String,
    pub university_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct SubdivisionList(pub Vec<Subdivision>);

impl SubdivisionList {
    pub fn list(conn: &mut PgConnection) -> Self {
        use crate::schema::subdivisions::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = subdivisions
            .limit(10)
            .load::<Subdivision>(conn)
            .expect("Error Loading Subdivisions");

        SubdivisionList(result)
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name= subdivisions)]
pub struct NewSubdivision {
    pub name: String,
}

impl NewSubdivision {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Subdivision, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(subdivisions::table)
            .values(self)
            .get_result(conn)
    }
}
