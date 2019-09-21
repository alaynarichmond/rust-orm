extern crate syn;
extern crate postgres;

use::syn::{Type};
use postgres::{Connection, TlsMode};


pub struct Column {
    pub name: String,
    // typ: Type
}

pub struct Table {
    pub name: String,
    pub columns: Vec<Column>
}

impl Table {
    fn create_table(&self) {
        let mut query = "CREATE TABLE ".to_owned();
        query.push_str(&self.name);
        query.push_str(" ( ");
        for column in self.columns.iter() {
            query.push_str(&column.name);
            query.push_str(" varchar,");
        }
        // remove last comma
        query.pop();
        query.push_str(" ) ");
        println!("{}", query);

        let conn = Connection::connect("postgresql://postgres@localhost", TlsMode::None).unwrap();
        conn.execute(&query, &[]).unwrap();
    }
}

pub trait Relation {
    fn get_table() -> Table;

    fn create_table() {
        Self::get_table().create_table();
    }
}