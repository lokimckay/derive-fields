#![allow(unused)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    CategoryA,
    CategoryB(SubCategory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubCategory {
    SubStructA(SubStructA),
    SubStructB(SubStructB),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubStructA {
    a: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubStructB {
    b: u32,
}
