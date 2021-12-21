fn sink<T: Ord>(arr: &mut [T], mut idx: usize, size: usize) {
    while 2 * idx <= size {
        let mut child = 2 * idx;
        // 确认子节点不在队尾，再取最大子节点
        if (child != size) && (arr[child] < arr[child + 1]) {
            child += 1;
        }
        // 下沉直到终点为止
        if arr[idx] < arr[child] {
            arr.swap(idx, child);
            idx = child;
        } else {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn heap<T: Ord>(arr: &mut [T]) {
    // 剔除哨兵位获得真长度
    let mut size = arr.len() - 1;
    // 遍历时避开叶子
    for idx in (1..=size / 2).rev() {
        sink(arr, idx, size);
    }
    while size > 1 {
        // 释放最大节点至数组末
        arr.swap(1, size);
        // 出队，收窄队列
        size -= 1;
        // 重新让堆有序
        sink(arr, 1, size);
    }
}

#[cfg(test)]
mod tests {
    use super::heap;
    #[test]
    fn empty() {
        let mut nothing: [u32; 1] = [0];
        heap(&mut nothing);
    }

    #[test]
    fn basic() {
        let mut arr: [u32; 11] = [0, 2, 5, 9, 8, 7, 4, 3, 10, 16, 13];
        heap(&mut arr);
        assert_eq!(arr, [0, 2, 3, 4, 5, 7, 8, 9, 10, 13, 16]);
    }
}
