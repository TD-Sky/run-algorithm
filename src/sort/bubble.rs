#[allow(dead_code)]
pub fn bubble<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for out in 1..len {
        for n in 0..(len - out) {
            if arr[n] > arr[n + 1] {
                arr.swap(n, n + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble;
    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        bubble(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr: [i32; 1] = [7];
        bubble(&mut arr);
        assert_eq!(arr, [7]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [char; 3] = ['a', 'b', 'c'];
        bubble(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn basic() {
        let mut arr: [char; 4] = ['d', 'a', 'c', 'b'];
        bubble(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd']);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: [i32; 4] = [542, 542, 542, 542];
        bubble(&mut arr);
        assert_eq!(arr, [542, 542, 542, 542]);
    }
}
