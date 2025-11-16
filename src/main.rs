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

impl ToDoList {
    
    fn pretty_print(&self) {
        let mut out_list: Vec<String> = vec![];
        for item in &self.items {
            let completed_emoji = match item.completed {
                true => "✅",
                false => "❌"
            };
            out_list.push(format!("{} {}",item.title,completed_emoji));
        }
        println!("Tasks:");
        println!("{}", out_list.join("\n"));
    }

}

fn create_temp_list() -> ToDoList {
    let mut my_list: ToDoList = ToDoList {items: vec![].to_owned()};
    my_list.items.push(ListItem { title: "My First Item".to_string(), completed: false });
    return my_list
}

fn write_to_file(list_data: String, file_name: &str) {
    let mut file = fs::File::create(file_name).expect("to create a file");
    file.write_all(list_data.as_bytes()).expect("write to file");
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
        _ => {
            println!("The file {} could not be found, initializing an empty list.", FILE);
            let d = ToDoList { items: vec![] };           
            d
        } 
    };
    file.pretty_print();
    // println!("{:?}",file)
    // let serialized = serde_json::to_string(&my_data).expect("serialize my_list");
    // write_to_file(serialized, FILE);
    // match file {
    //     Ok(file) => file.write_all(my_list.serialize(Serializer.)),
    //     _ => println!("{:?}", file)
    // }

}
