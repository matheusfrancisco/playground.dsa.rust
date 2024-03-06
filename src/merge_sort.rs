fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sorting() {
        let mut arr = vec![5, 4, 1, 2, 3];
        assert_eq!(merge_sort(&mut arr), vec![1, 2, 3, 4, 5]);
    }
}
