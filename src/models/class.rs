use crate::schema::classes;
use diesel::PgConnection;
use crate::models::component::Component;
use diesel::prelude::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Component))]
#[diesel(table_name = classes)]
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

        classes::table.find(id).first(conn)
    }

    pub fn destroy(id: &i32, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {

        diesel::delete(classes::table.find(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn update(id: &i32, new_university: &NewClass, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {

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

        diesel::insert_into(classes::table)
            .values(self)
            .get_result(conn)
    }

    pub fn new_simple_class(&self, conn: &mut PgConnection, passed_name: String, passed_credits: i32, passed_component_id: i32) -> Result<Class, diesel::result::Error> {
        use crate::schema::classes::dsl::*;

        diesel::insert_into(classes)
            .values(NewClass {
                name: Some(passed_name),
                description: None,
                credits: Some(passed_credits),
                pftype: None,
                subject: None,
                course_no:None,
                options: None,
                component_id: Some(passed_component_id)
            })
            .get_result(conn)
    }
}

#[derive(Debug, Insertable, Deserialize, Associations)]
#[diesel(belongs_to(Component))]
#[diesel(table_name = classes)]
pub struct SimpleClass {
    pub name: Option<String>,
    pub credits: Option<i32>,
    pub component_id: Option<i32>
}

impl SimpleClass {
    pub fn create(&self, conn: &mut PgConnection) -> Result<Class, diesel::result::Error> {

        diesel::insert_into(classes::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassList(pub Vec<Class>);

impl ClassList {
    pub fn list(conn: &mut PgConnection) -> Self {

        use crate::schema::classes::dsl::*;

        let result =
            classes
                .limit(10)
                .load::<Class>(conn)
                .expect("Error loading classes");

        ClassList(result)
    }
}
