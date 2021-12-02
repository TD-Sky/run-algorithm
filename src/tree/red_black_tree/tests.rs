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

#[test]
fn test_pop_min() {
    let mut map: RBTMap<&str> = RBTMap::new();

    map.insert(0, "Marry");
    map.insert(1, "Mike");
    map.insert(2, "Peter");

    assert_eq!(map.pop_min().unwrap(), "Marry");
    assert_eq!(map.pop_min().unwrap(), "Mike");
    assert_eq!(map.pop_min().unwrap(), "Peter");
    assert_eq!(map.pop_min(), None);
}

#[test]
fn test_remove() {
    let mut map: RBTMap<&str> = RBTMap::new();

    map.insert(0, "Marry");
    map.insert(1, "Mike");
    map.insert(2, "Peter");

    assert_eq!(map.remove(0).unwrap(), "Marry");
    assert_eq!(map.remove(4), None);
    assert_eq!(map.remove(2).unwrap(), "Peter");
    assert_eq!(map.remove(1).unwrap(), "Mike");
}
