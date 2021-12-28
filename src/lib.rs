pub mod collections;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.clear();

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
