// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
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
    components (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        class -> Nullable<Int4>,
    }
}

diesel::table! {
    components_to_components (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
    }
}

diesel::table! {
    degrees (id) {
        id -> Int4,
        name -> Varchar,
        pftype -> Varchar,
        code -> Varchar,
        description -> Nullable<Text>,
        subdivision -> Nullable<Int4>,
    }
}

diesel::table! {
    degrees_to_components (id) {
        id -> Int4,
        degree -> Int4,
        component -> Int4,
    }
}

diesel::table! {
    subdivisions (id) {
        id -> Int4,
        name -> Varchar,
        university -> Nullable<Int4>,
    }
}

diesel::table! {
    universities (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(components -> classes (class));
diesel::joinable!(degrees -> subdivisions (subdivision));
diesel::joinable!(subdivisions -> universities (university));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    components,
    components_to_components,
    degrees,
    degrees_to_components,
    subdivisions,
    universities,
);
