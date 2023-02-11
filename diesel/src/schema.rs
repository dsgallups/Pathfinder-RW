// @generated automatically by Diesel CLI.

diesel::table! {
    class (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        credits -> Nullable<Int4>,
        pftype -> Varchar,
        subject -> Nullable<Varchar>,
        course_no -> Nullable<Varchar>,
        options -> Nullable<Text>,
    }
}

diesel::table! {
    component (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        pftype -> Varchar,
        class -> Nullable<Int4>,
        options -> Nullable<Text>,
    }
}

diesel::table! {
    component_to_component (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
    }
}

diesel::table! {
    degree (id) {
        id -> Int4,
        name -> Varchar,
        pftype -> Varchar,
        code -> Varchar,
        description -> Nullable<Text>,
        subdivision -> Nullable<Int4>,
    }
}

diesel::table! {
    degree_to_component (id) {
        id -> Int4,
        degree -> Int4,
        component -> Int4,
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
diesel::joinable!(degree -> subdivision (subdivision));
diesel::joinable!(subdivision -> university (university));

diesel::allow_tables_to_appear_in_same_query!(
    class,
    component,
    component_to_component,
    degree,
    degree_to_component,
    subdivision,
    university,
);
