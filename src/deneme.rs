// //modüllere ayrılmadan önceki hali

// use std::io;

// struct Item {
//     data: String,
//     completed: bool,
// }

// fn add_item(data: String, completed: bool, items: &mut Vec<Item>) {
//     let item = Item { data, completed };
//     items.push(item);
// }

// fn delete_item(index: usize, items: &mut Vec<Item>) -> Item {
//     let item: Item = items.remove(index);
//     item
// }

// fn see_list(items: &[Item]) {
//     for item in items {
//         println!("-Data:{}-Completed:{}\n", item.data, item.completed);
//     }
// }

// fn main() {
//     let mut items: Vec<Item> = Vec::new();
//     let mut completed_items: Vec<Item> = Vec::new();
//     println!("TO DO APP\n1-Add Item\n2-Delete Item\n3-Complete Item\n4-See To Do List\n5-See Completed Items\n6-Exit\n");
//     loop {
//         let mut select = String::new();
//         println!("Please select the action you want to do: ");
//         io::stdin()
//             .read_line(&mut select)
//             .expect("Failed to read line");
//         let select: i32 = match select.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         match select {
//             1 => {
//                 println!("Enter item data: ");
//                 let mut data = String::new();
//                 io::stdin()
//                     .read_line(&mut data)
//                     .expect("Failed to read line");
//                 add_item(data, false, &mut items);
//             }
//             2 => {
//                 println!("Enter index of item to delete:");

//                 let mut index_input = String::new();
//                 io::stdin()
//                     .read_line(&mut index_input)
//                     .expect("Failed to read line");

//                 let index: usize = match index_input.trim().parse() {
//                     Ok(num) => num,
//                     Err(_) => continue,
//                 };

//                 let item = delete_item(index, &mut items);
//                 println!(
//                     "Deleted Item\n-Data:{}-Completed:{}\n",
//                     item.data, item.completed
//                 );
//             }
//             3 => {
//                 println!("Enter index of item to complete:");

//                 let mut index_input = String::new();
//                 io::stdin()
//                     .read_line(&mut index_input)
//                     .expect("Failed to read line");

//                 let index: usize = match index_input.trim().parse() {
//                     Ok(num) => num,
//                     Err(_) => continue,
//                 };
//                 let item = delete_item(index, &mut items);
//                 add_item(item.data, true, &mut completed_items)
//             }
//             4 => see_list(&items),
//             5 => see_list(&completed_items),
//             6 => {
//                 println!("You left the menu");
//                 break;
//             }
//             _ => println!("Wrong choice, try again"),
//         }
//     }
// }
