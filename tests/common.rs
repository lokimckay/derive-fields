#![allow(unused)]

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "fields-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Category {
    CategoryA,
    CategoryB(SubCategory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "fields-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubCategory {
    SubStructA(SubStructA),
    SubStructB(SubStructB),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "fields-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubStructA {
    a: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "fields-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubStructB {
    b: u32,
}
