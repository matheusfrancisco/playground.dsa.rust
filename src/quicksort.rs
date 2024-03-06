fn quicksort(arr: &mut [i32], start: usize, end: usize) -> Vec<i32> {
    if start < end {
        let part = partition(arr, start, end);
        quicksort(arr, start, part - 1);
        quicksort(arr, part + 1, end);
    }
    arr.to_vec()
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let mut i = start;
    let pivot = end;
    for j in start..=end - 1 {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sorting() {
        let mut arr = vec![5, 4, 1, 2, 3];
        let len = arr.len();
        assert_eq!(quicksort(&mut arr, 0, len - 1), vec![1, 2, 3, 4, 5]);
    }
}
