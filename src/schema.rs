// @generated automatically by Diesel CLI.

diesel::table! {
    class (id) {
        id -> Int4,
        subject -> Nullable<Varchar>,
        course_no -> Nullable<Varchar>,
        credits -> Nullable<Int4>,
        pftype -> Varchar,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        options -> Nullable<Json>,
    }
}

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
        component_id -> Nullable<Int4>,
    }
}

diesel::table! {
    component (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        pftype -> Varchar,
        class -> Nullable<Int4>,
        options -> Nullable<Json>,
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
    components (id) {
        id -> Int4,
        name -> Varchar,
        pftype -> Varchar,
    }
}

diesel::table! {
    components_to_components (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
        association_type -> Varchar,
        logic_type -> Varchar,
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
    subdivision (id) {
        id -> Int4,
        name -> Varchar,
        university -> Nullable<Int4>,
    }
}

diesel::table! {
    subdivisions (id) {
        id -> Int4,
        name -> Varchar,
        university_id -> Nullable<Int4>,
    }
}

diesel::table! {
    universities (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    university (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(classes -> components (component_id));
diesel::joinable!(component -> class (class));
diesel::joinable!(degree -> subdivision (subdivision));
diesel::joinable!(degrees -> subdivisions (subdivision));
diesel::joinable!(subdivision -> university (university));
diesel::joinable!(subdivisions -> universities (university_id));

diesel::allow_tables_to_appear_in_same_query!(
    class,
    classes,
    component,
    component_to_component,
    components,
    components_to_components,
    degree,
    degree_to_component,
    degrees,
    degrees_to_components,
    subdivision,
    subdivisions,
    universities,
    university,
);
