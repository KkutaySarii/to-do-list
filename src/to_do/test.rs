#[cfg(test)]
pub mod tests {
    use crate::to_do::add_item;
    use crate::to_do::delete_item;
    use crate::to_do::Item;
    use std::fmt;
    fn items_are_equal(item1: &Item, item2: &Item) -> bool {
        item1.data == item2.data && item1.completed == item2.completed
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
