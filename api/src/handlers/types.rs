use crate::models::class::Class;
use actix_web::{error, Result};
use derive_more::Display;
use serde::Serialize;
use std::fmt;
use thiserror::Error;
//use std::str::FromStr;
//use strum_macros::EnumString;

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
    Degree((&'a str, &'a str, &'a str, &'a str)),
}

#[derive(Debug)]
pub enum ComponentLogic {
    AND,
    OR,
    NONE,
}

#[derive(Debug, Serialize)]
pub struct Schedule {
    pub periods: Vec<Period>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            periods: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Period {
    pub year: u32,
    pub time: String,
    pub classes: Vec<Req>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Req {
    pub id: i32,
    pub name: String,
    pub pftype: String,
    pub class: Option<Class>,
    pub logic_type: Option<String>,
    pub children: Vec<(i32, Status)>,
    pub parents: Vec<(i32, Status)>,
    pub in_analysis: bool,
}

impl Req {
    pub fn str(&self) -> String {
        format!(
            "{:12}: logic_type: {:60?}, children:{:?}, parents: {:?}",
            self.name, self.logic_type, self.children, self.parents
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Status {
    Unchecked,
    Checked,
    Unsuitable,
    Desirable,
    Selected,
}

#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Diesel Error")]
    DieselError(#[from] diesel::result::Error),

    #[error("Component not found")]
    AssociationError,

    #[error("Prereq is invalid for this Degree")]
    PrereqError,
}
impl error::ResponseError for ScheduleError {}
/*
impl From<diesel::result::Error> for ScheduleError {
    fn from(error: diesel::result::Error) -> Self {
        ScheduleError::DieselError(error)
    }
}
*/
