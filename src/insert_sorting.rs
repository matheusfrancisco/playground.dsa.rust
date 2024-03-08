fn insert_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let size = arr.len();

    for i in 1..size {
        for j in 0..i {
            if arr[i] < arr[j] {
                arr.swap(i, j);
            }
        }
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_sort() {
        let mut arr = vec![5, 4, 1, 2, 3];
        assert_eq!(insert_sort(&mut arr), vec![1, 2, 3, 4, 5]);
    }
}
