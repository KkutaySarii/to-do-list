use super::structs::Item;
use std::io;
pub fn take_input(message: &str) -> String {
    println!("{message}");
    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");
    data
}
pub fn add_item(data: String, completed: bool, items: &mut Vec<Item>) {
    if items.iter().any(|item| item.data == data) {
        println!("Item with this data already exists");
    } else {
        let item = Item { data, completed };
        items.push(item);
    }
}
pub fn see_list(items: &[Item]) {
    for item in items {
        println!("-Data:{}-Completed:{}\n", item.data, item.completed);
    }
}
pub fn delete_item(data: String, items: &mut Vec<Item>) -> Item {
    let index = items
        .iter()
        .position(|item| item.data == data)
        .expect("Item not found");
    let item = items.remove(index);
    item
}
