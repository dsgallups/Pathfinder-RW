pub enum LogicalType<'a> {
    AND(Vec<InstantiationType<'a>>),
    OR(Vec<InstantiationType<'a>>)
}

pub enum ParsedLogicType {
    AND(Vec<usize>),
    OR(Vec<usize>)
}

#[allow(dead_code)]
pub enum InstantiationType<'a> {
   SimpleClass(&'a str),
   Class((&'a str, i32)),
   Reg(&'a str),
   Degree((&'a str, &'a str, &'a str))
}

pub enum ComponentLogic {
    AND,
    OR
}