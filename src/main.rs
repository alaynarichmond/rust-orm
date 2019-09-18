#[macro_use]
extern crate rust_orm_derive;

use std::io::{Write, stdout};
use rust_orm_lib::{Table, Column, Relation};

#[derive(Relation)]
struct Student {
    name: String,
    age: usize,
    major: String
}


fn main() {
    let mut stdout = stdout();
    writeln!(&mut stdout, "Hello World").unwrap();

//    let table = Table::get_table();
}