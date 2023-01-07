// to_do klasör altında ki functions.rs dosyasındaki fonksiyonlarımı
//test fonksiyonlarımda kullanmak istiyorum, onları nasıl çağırırım
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Item {
    pub data: String,
    pub completed: bool,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item {{ data: {}, completed: {} }}",
            self.data, self.completed
        )
    }
}
pub fn add_item(data: String, completed: bool, items: &mut Vec<Item>) {
    if items.iter().any(|item| item.data == data) {
        println!("Item with this data already exists");
    } else {
        let item = Item { data, completed };
        items.push(item);
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

#[cfg(test)]
mod tests {
    use super::*;
    fn items_are_equal(item1: &Item, item2: &Item) -> bool {
        item1.data == item2.data && item1.completed == item2.completed
    }
    #[test]
    fn test_add_item() {
        let mut items = Vec::new();
        add_item(String::from("Buy milk"), false, &mut items); // add_item, functions.rs dan çağrılcak
        add_item(String::from("Buy eggs"), true, &mut items);

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].data, "Buy milk");
        assert_eq!(items[0].completed, false);
        assert_eq!(items[1].data, "Buy eggs");
        assert_eq!(items[1].completed, true);
    }

    #[test]
    fn test_delete_item() {
        // Create a list of items
        let mut items = vec![
            Item {
                data: "item1".to_string(),
                completed: true,
            },
            Item {
                data: "item2".to_string(),
                completed: false,
            },
            Item {
                data: "item3".to_string(),
                completed: true,
            },
        ];

        // Call the delete_item function
        let deleted_item = delete_item("item2".to_string(), &mut items);

        // Check that the correct item was deleted
        assert!(items_are_equal(
            &deleted_item,
            &Item {
                data: "item2".to_string(),
                completed: false
            }
        ));

        assert_eq!(
            items,
            vec![
                Item {
                    data: "item1".to_string(),
                    completed: true
                },
                Item {
                    data: "item3".to_string(),
                    completed: true
                },
            ]
        );
    }
}
