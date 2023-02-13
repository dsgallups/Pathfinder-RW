pub enum LogicalType<'a> {
    AND(Vec<InstantiationType<'a>>),
    OR(Vec<InstantiationType<'a>>),
}

pub enum ParsedLogicType {
    AND(Vec<usize>),
    OR(Vec<usize>),
}

#[allow(dead_code)]
pub enum InstantiationType<'a> {
    SimpleClass(&'a str),
    Class((&'a str, i32)),
    Reg(&'a str),
    Degree((&'a str, &'a str, &'a str)),
}

#[derive(Debug)]
pub enum ComponentLogic {
    AND,
    OR,
    NONE,
}

/*
impl From<diesel::result::Error> for ScheduleError {
    fn from(error: diesel::result::Error) -> Self {
        ScheduleError::DieselError(error)
    }
}
*/
