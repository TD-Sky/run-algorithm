#[allow(dead_code)]
pub fn insertion<T: Ord>(arr: &mut [T]) {
    for right in 1..arr.len() {
        for left in (1..=right).rev() {
            if arr[left - 1] > arr[left] {
                arr.swap(left, left - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion;
    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        insertion(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr: [i32; 1] = [7];
        insertion(&mut arr);
        assert_eq!(arr, [7]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [char; 3] = ['a', 'b', 'c'];
        insertion(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn basic() {
        let mut arr = [2, 5, 9, 8, 7, 4, 3, 10, 16, 13];
        insertion(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 7, 8, 9, 10, 13, 16]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: [i32; 4] = [542, 542, 542, 542];
        insertion(&mut arr);
        assert_eq!(arr, [542, 542, 542, 542]);
    }
}