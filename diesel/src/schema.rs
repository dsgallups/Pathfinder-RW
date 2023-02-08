// @generated automatically by Diesel CLI.

diesel::table! {
    class (id) {
        id -> Int4,
        subject -> Nullable<Varchar>,
        course_no -> Nullable<Varchar>,
        credits -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Varchar,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        options -> Nullable<Json>,
    }
}

diesel::table! {
    component (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        class -> Nullable<Int4>,
        options -> Nullable<Json>,
    }
}

diesel::table! {
    component_to_component (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        child_id -> Nullable<Int4>,
    }
}

diesel::table! {
    degrees (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        code -> Varchar,
        description -> Nullable<Text>,
        subdivision_id -> Int4,
        components -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    subdivision (id) {
        id -> Int4,
        name -> Varchar,
        university -> Nullable<Int4>,
    }
}

diesel::table! {
    university (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(component -> class (class));
diesel::joinable!(degrees -> subdivision (subdivision_id));
diesel::joinable!(subdivision -> university (university));

diesel::allow_tables_to_appear_in_same_query!(
    class,
    component,
    component_to_component,
    degrees,
    subdivision,
    university,
);
