#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate prettytable;

pub mod expr;
pub mod state;
pub mod problem;
pub mod grammar;
pub mod tableau;
pub mod var;

mod grammar_test;
