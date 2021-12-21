#[allow(dead_code)]
pub fn binary_search<T: Ord>(seq: &[T], x: T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = seq.len();
    while low < high {
        let mid: usize = (low + high) / 2;
        if x > seq[mid] {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    (x == seq[low]).then_some(low)
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn basic() {
        let mut arr: [i32; 5] = [85, 63, 24, 56, 45];
        arr.sort();
        let index: usize = binary_search(&arr, 24).unwrap();
        assert_eq!(index, 0);
    }
}
