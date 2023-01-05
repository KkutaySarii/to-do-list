use crate::to_do::functions::take_input;

mod to_do;

fn main() {
    let mut items = Vec::new();
    let mut completed_items = Vec::new();
    println!("TO DO APP\n1-Add Item\n2-Delete Item\n3-Complete Item\n4-See To Do List\n5-See Completed Items\n6-Exit\n");
    loop {
        let select = take_input("Please select the action you want to do: ");
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match select {
            1 => to_do::functions::add_item(false, &mut items),
            2 => {
                let removed_item = to_do::functions::delete_item(
                    "Enter the data of the item to be removed from the list",
                    &mut items,
                );
                println!(
                    "Removed Item\n-Data:{}-Completed:{}\n",
                    removed_item.data, removed_item.completed
                );
            }
            3 => {
                let completed_item = to_do::functions::delete_item(
                    "Enter the data of the completed item",
                    &mut items,
                );
                println!(
                    "Removed Item\n-Data:{}-Completed:{}\n",
                    completed_item.data, completed_item.completed
                );
                to_do::functions::add_item(true, &mut completed_items)
            }
            4 => to_do::functions::see_list(&items),
            5 => to_do::functions::see_list(&completed_items),
            6 => {
                println!("You left the menu");
                break;
            }
            _ => println!("Wrong choice, try again"),
        }
    }
}
