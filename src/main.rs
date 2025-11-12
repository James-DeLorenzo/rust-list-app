#![allow(dead_code, unused_variables)]
use serde::{Deserialize,Serialize};
use std::{fs, io::{Read, Write}, path};
// use serde_json::Result;

const FILE: &str = "my_list.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ListItem {
    title: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct ToDoList {
    items: Vec<ListItem>
}

fn create_temp_list() -> ToDoList {
    let mut my_list: ToDoList = ToDoList {items: vec![].to_owned()};
    my_list.items.push(ListItem { title: "My First Item".to_string(), completed: false });
    return my_list
}

fn write_to_file(data: String, file_name: &str) {
    let mut file = fs::File::create(file_name).expect("to create a file");
    file.write_all(data.as_bytes()).expect("write to file");
}

fn main() {
    let my_data = create_temp_list();
    let file_exists = path::Path::new(FILE).try_exists().unwrap();
    let file = match file_exists {
        true => {
            let mut data = vec![];
            _ = fs::File::open(FILE).expect("read data file").read_to_end( &mut data);
            let d : ToDoList = serde_json::from_slice(&data).expect("your mom");
            d
        }
        false => ToDoList { items: vec![] },
    };
    println!("{:?}",file)
    // let serialized = serde_json::to_string(&my_data).expect("serialize my_list");
    // write_to_file(serialized, FILE);
    // match file {
    //     Ok(file) => file.write_all(my_list.serialize(Serializer.)),
    //     _ => println!("{:?}", file)
    // }

}
