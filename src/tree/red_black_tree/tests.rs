use super::RBTMap;

fn u32_str_sample<'a>() -> RBTMap<'a, u32, &'a str> {
    let mut map = RBTMap::new();

    map.insert(0, "Marry");
    map.insert(1, "Mike");
    map.insert(2, "John");
    map.insert(3, "Peter");

    map
}

#[test]
fn test_insert() {
    let map = u32_str_sample();

    assert_eq!(
        map.preorder().unwrap().collect::<Vec<_>>(),
        vec![(&1, &"Mike"), (&0, &"Marry"), (&3, &"Peter"), (&2, &"John")]
    );
}

#[test]
fn test_pop_min() {
    let mut map = u32_str_sample();

    assert_eq!(map.pop_min(), Some("Marry"));
    assert_eq!(map.pop_min(), Some("Mike"));
    assert_eq!(map.pop_min(), Some("John"));
    assert_eq!(map.pop_min(), Some("Peter"));
    assert_eq!(map.pop_min(), None);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove() {
    let mut map = u32_str_sample();

    assert_eq!(map.remove(&0), Some("Marry"));
    assert_eq!(map.remove(&1), Some("Mike"));
    assert_eq!(map.remove(&20), None);
    assert_eq!(map.remove(&2), Some("John"));
    assert_eq!(map.remove(&3), Some("Peter"));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_get() {
    let map = u32_str_sample();

    assert_eq!(map.get(&0), Some(&"Marry"));
    assert_eq!(map.get(&1), Some(&"Mike"));
    assert_eq!(map.get(&20), None);
    assert_eq!(map.get(&2), Some(&"John"));
    assert_eq!(map.get(&3), Some(&"Peter"));
}
