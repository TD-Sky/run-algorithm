use super::RBTMap;

#[test]
fn test_insert() {
    let mut map: RBTMap<&str> = RBTMap::new();

    map.insert(0, "Marry");
    map.insert(1, "Mike");
    map.insert(2, "John");
    map.insert(3, "Peter");

    assert_eq!(
        map.preorder().unwrap().collect::<Vec<_>>(),
        vec![(1, &"Mike"), (0, &"Marry"), (3, &"Peter"), (2, &"John")]
    );
}

/* #[test]
fn test_pop_min() {
    let mut map: RBTMap<&str> = RBTMap::new();

    map.insert(0, "Marry");
    map.insert(1, "Mike");
    map.insert(2, "Peter");

    assert_eq!(map.pop_min().unwrap(), "Marry");
} */
