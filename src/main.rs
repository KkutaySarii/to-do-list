mod to_do;

fn main() {
    let mut items = Vec::new();
    let mut completed_items = Vec::new();
    println!("TO DO APP\n1-Add Item\n2-Delete Item\n3-Complete Item\n4-See To Do List\n5-See Completed Items\n6-Exit\n");
    loop {
        let select = to_do::take_input("Please select the action you want to do: ");
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match select {
            1 => {
                let data = to_do::take_input("Enter item data: ");
                to_do::add_item(data, false, &mut items);
            }
            2 => {
                let data: String =
                    to_do::take_input("Enter the data of the item to be removed from the list");
                let removed_item = to_do::delete_item(data, &mut items);
                println!(
                    "Removed Item\n-Data:{}-Completed:{}\n",
                    removed_item.data, removed_item.completed
                );
            }
            3 => {
                let data = to_do::take_input("Enter the data of the completed item: ");
                let completed_item = to_do::delete_item(data, &mut items);
                println!(
                    "Completed Item\n-Data:{}-Completed:{}\n",
                    completed_item.data, completed_item.completed
                );
                to_do::add_item(completed_item.data, true, &mut completed_items)
            }
            4 => to_do::see_list(&items),
            5 => to_do::see_list(&completed_items),
            6 => {
                println!("You left the menu");
                break;
            }
            _ => println!("Wrong choice, try again"),
        }
    }
}
