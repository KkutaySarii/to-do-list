// to_do klasör altında ki functions.rs dosyasındaki fonksiyonlarımı
//test fonksiyonlarımda kullanmak istiyorum, onları nasıl çağırırım

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_item() {
        let mut items = Vec::new();
        //add_item(String::from("Buy milk"), false, &mut items); // add_item, functions.rs dan çağrılcak
        //add_item(String::from("Buy eggs"), true, &mut items);

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].data, "Buy milk");
        assert_eq!(items[0].completed, false);
        assert_eq!(items[1].data, "Buy eggs");
        assert_eq!(items[1].completed, true);
    }
}
