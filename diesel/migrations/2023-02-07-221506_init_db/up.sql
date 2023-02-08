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
    subject CHARACTER VARYING (50),
    course_no CHARACTER VARYING (50),
    credits INTEGER,
    type CHARACTER VARYING (50) DEFAULT 'class' NOT NULL,
    title CHARACTER VARYING (50),
    description TEXT,
    options JSON
);

CREATE TABLE component (
    id SERIAL PRIMARY KEY,
    title CHARACTER VARYING (50) NOT NULL,
    description TEXT,
    type CHARACTER VARYING (10) NOT NULL,
    class INTEGER REFERENCES class(id),
    options JSON
);

CREATE TABLE component_to_component (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER REFERENCES component(id),
    child_id INTEGER REFERENCES component(id)
);

CREATE TABLE degrees (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING (50) NOT NULL,
    type CHARACTER VARYING (50) NOT NULL,
    code CHARACTER VARYING (50) NOT NULL,
    description TEXT,
    subdivision_id INTEGER REFERENCES subdivision(id) NOT NULL,
    components INTEGER[]
);