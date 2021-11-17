use super::BSTMap;

#[test]
fn insert_and_query() {
    let mut bst: BSTMap<&str> = BSTMap::new();
    bst.insert(1, "Mike");
    bst.insert(2, "Jason");
    bst.insert(3, "Mary");
    assert!(bst.contains_key(2));
}

#[test]
fn insert_and_get() {
    let mut bst: BSTMap<&str> = BSTMap::new();
    bst.insert(1, "Mike");
    bst.insert(2, "Jason");
    bst.insert(3, "Mary");
    assert_eq!(bst.get(4), None);
    assert_eq!(bst.get(1), Some(&"Mike"));
}

#[test]
fn in_traversal() {
    let mut bst: BSTMap<&str> = BSTMap::new();
    bst.insert(5, "Eson");
    bst.insert(4, "Frank");
    bst.insert(3, "Mary");
    bst.insert(7, "Klein");
    bst.insert(6, "Mike");
    bst.insert(8, "Jason");
    assert_eq!(
        bst.inorder().unwrap().collect::<Vec<_>>(),
        vec![&"Mary", &"Frank", &"Eson", &"Mike", &"Klein", &"Jason"]
    )
}

#[test]
fn from_preorder() {
    let bst = BSTMap::from_preorder(vec![(2, "Jason"), (1, "Mike"), (3, "Mary")]);
    assert_eq!(
        bst.preorder().unwrap().collect::<Vec<_>>(),
        vec![(2, &"Jason"), (1, &"Mike"), (3, &"Mary")]
    )
}
