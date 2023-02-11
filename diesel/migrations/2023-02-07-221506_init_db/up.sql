CREATE TABLE university (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    description TEXT
);

CREATE TABLE subdivision (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    university INTEGER REFERENCES university(id)
);

CREATE TABLE class (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    description TEXT,
    credits INTEGER,
    pftype CHARACTER VARYING (50) DEFAULT 'class' NOT NULL,
    subject CHARACTER VARYING (50),
    course_no CHARACTER VARYING (50),
    options TEXT
    --options JSON
);

CREATE TABLE component (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50),
    class INTEGER REFERENCES class(id)
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
CREATE TABLE component_to_component (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER REFERENCES component(id) NOT NULL,
    child_id INTEGER REFERENCES component(id) NOT NULL
);

CREATE TABLE degree (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    pftype CHARACTER VARYING (50) NOT NULL,
    code CHARACTER VARYING (50) NOT NULL,
    description TEXT,
    subdivision INTEGER REFERENCES subdivision(id)
);

CREATE TABLE degree_to_component (
    id SERIAL PRIMARY KEY,
    degree INTEGER REFERENCES component(id) NOT NULL,
    component INTEGER REFERENCES component(id) NOT NULL
);