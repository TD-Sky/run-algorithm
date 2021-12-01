use super::RBTMap;

#[test]
fn test_insert() {
    let mut map: RBTMap<&str> = RBTMap::new();

    map.insert(0, "Marry");
    map.insert(1, "Mike");
    map.insert(2, "Peter");

    assert_eq!(
        map.preorder().unwrap().collect::<Vec<_>>(),
        vec![(0, &"Marry"), (1, &"Mike"), (2, &"Peter")]
    );
}
