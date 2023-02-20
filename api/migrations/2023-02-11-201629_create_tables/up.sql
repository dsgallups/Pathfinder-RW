CREATE TABLE universities (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    description TEXT
);
CREATE TABLE subdivisions (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    university_id INTEGER REFERENCES universities(id)
);
CREATE TABLE components (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    pftype CHARACTER VARYING (10) NOT NULL,
    logic_type CHARACTER VARYING(10)
);
CREATE TABLE classes (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    description TEXT,
    credits INTEGER,
    pftype CHARACTER VARYING (50) DEFAULT 'class' NOT NULL,
    subject CHARACTER VARYING (50),
    course_no CHARACTER VARYING (50),
    options TEXT,
    component_id INTEGER REFERENCES components(id),
    UNIQUE (component_id) --options JSON
);
/*
 CREATE TABLE component (
 id SERIAL PRIMARY KEY,
 name CHARACTER VARYING (50) NOT NULL,
 description TEXT,
 pftype CHARACTER VARYING (10) NOT NULL,
 class INTEGER REFERENCES class(id),
 options TEXT
 );
 */
CREATE TABLE components_to_components (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER REFERENCES components(id) NOT NULL,
    child_id INTEGER REFERENCES components(id) NOT NULL
);
CREATE TABLE degrees (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (150) NOT NULL,
    pftype CHARACTER VARYING (50) NOT NULL,
    code CHARACTER VARYING (50) NOT NULL,
    description TEXT,
    subdivision INTEGER REFERENCES subdivisions(id)
);
CREATE TABLE degrees_to_components (
    id SERIAL PRIMARY KEY,
    degree_id INTEGER REFERENCES degrees(id) NOT NULL,
    component_id INTEGER REFERENCES components(id) NOT NULL
);