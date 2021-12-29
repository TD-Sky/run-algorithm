macro_rules! get_then_inc {
    ($num: ident) => {{
        let past = $num;
        $num += 1;
        past
    }};
}

#[allow(dead_code)]
pub enum Order {
    TB,
    BT,
}

pub use Order::{BT, TB};

pub struct MergeSort<'a, T: Ord + Clone> {
    aux: Vec<T>,
    arr: &'a mut [T],
}

impl<'a, T> MergeSort<'a, T>
where
    T: Ord + Clone,
{
    fn sort_bottom_to_top(&mut self) {
        let len = self.arr.len();
        // 子数组的长度，同一循环内子数组按 size 切分
        for size in (0u32..).map(|n| 2usize.pow(n)).take_while(|&sz| sz < len) {
            // 两两归并子数组，故 low 为左数组首索引
            for low in (0..(len - size)).step_by(size * 2) {
                // 两两归并时，右子数组长度可能为 [0, size]
                self.merge(low, low + size - 1, (low + size * 2 - 1).min(len - 1));
            }
        }
    }

    fn sort_top_to_bottom(&mut self, low: usize, high: usize) {
        if low != high {
            let mid = (low + high) / 2;
            self.sort_top_to_bottom(low, mid);
            self.sort_top_to_bottom(mid + 1, high);
            self.merge(low, mid, high);
        }
    }

    fn merge(&mut self, low: usize, mid: usize, high: usize) {
        let mut left = low;
        let mut right = mid + 1;

        // 把辅助数组的元素有序放回原数组
        // right 或 left 的自增代表消耗子数组
        for i in low..=high {
            self.arr[i] = if left > mid {
                self.aux[get_then_inc!(right)].clone()
            } else if right > high {
                self.aux[get_then_inc!(left)].clone()
            } else if self.aux[left] > self.aux[right] {
                self.aux[get_then_inc!(right)].clone()
            } else {
                self.aux[get_then_inc!(left)].clone()
            };
        }
    }
}

#[allow(dead_code)]
impl<'a, T> MergeSort<'a, T>
where
    T: Ord + Clone,
{
    pub fn new(arr: &'a mut [T]) -> Self {
        let mut aux = Vec::with_capacity(arr.len());
        aux.clone_from_slice(arr);

        Self { aux, arr }
    }

    pub fn run(mut self, order: Order) {
        match order {
            TB => self.sort_top_to_bottom(0, self.arr.len() - 1),
            BT => self.sort_bottom_to_top(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::MergeSort;

    #[test]
    fn test_bt() {
        let mut arr = [7, 5, 9, 8, 2, 4, 3, 10, 16, 13, 17, 14, 6u32];
        MergeSort::new(&mut arr).run(super::BT);
        assert_eq!(arr, [2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 16, 17]);
    }

    #[test]
    fn test_empty() {
        let mut arr: [u32; 0] = [];
        MergeSort::new(&mut arr).run(super::BT);
    }

    #[test]
    fn test_tb() {
        let mut arr = [7, 5, 9, 8, 2, 4, 3, 10, 16, 13, 17, 14, 6u32];
        MergeSort::new(&mut arr).run(super::TB);
        assert_eq!(arr, [2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 16, 17]);
    }
}
