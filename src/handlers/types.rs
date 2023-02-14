use std::fmt;
use std::str::FromStr;
use strum_macros::EnumString;

pub enum LogicalType<'a> {
    GroupAND(Vec<InstantiationType<'a>>),
    GroupOR(Vec<InstantiationType<'a>>),
    PrereqAND(Vec<InstantiationType<'a>>),
    PrereqOR(Vec<InstantiationType<'a>>),
}

impl fmt::Display for LogicalType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogicalType::GroupAND(_) => write!(f, "GroupAND"),
            LogicalType::GroupOR(_) => write!(f, "GroupOR"),
            LogicalType::PrereqAND(_) => write!(f, "PrereqAND"),
            LogicalType::PrereqOR(_) => write!(f, "PrereqOR"),
        }
    }
}

pub enum ParsedLogicType {
    GroupAND(Vec<usize>),
    GroupOR(Vec<usize>),
    PrereqAND(Vec<usize>),
    PrereqOR(Vec<usize>),
}

impl fmt::Display for ParsedLogicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsedLogicType::GroupAND(_) => write!(f, "GroupAND"),
            ParsedLogicType::GroupOR(_) => write!(f, "GroupOR"),
            ParsedLogicType::PrereqAND(_) => write!(f, "PrereqAND"),
            ParsedLogicType::PrereqOR(_) => write!(f, "PrereqOR"),
        }
    }
}

#[allow(dead_code)]
pub enum InstantiationType<'a> {
    SimpleClass(&'a str),
    Class((&'a str, i32)),
    Group(&'a str),
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
