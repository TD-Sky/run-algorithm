#[allow(dead_code)]
pub fn selection<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut min = left;
        for right in (left + 1)..len {
            if arr[min] > arr[right] {
                min = right;
            }
        }
        arr.swap(left, min);
    }
}

#[cfg(test)]
mod tests {
    use super::selection;
    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        selection(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr: [i32; 1] = [7];
        selection(&mut arr);
        assert_eq!(arr, [7]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [char; 3] = ['a', 'b', 'c'];
        selection(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn basic() {
        let mut arr: [char; 4] = ['d', 'a', 'c', 'b'];
        selection(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd']);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: [i32; 4] = [542, 542, 542, 542];
        selection(&mut arr);
        assert_eq!(arr, [542, 542, 542, 542]);
    }
}
