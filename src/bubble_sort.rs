fn bubble_sorting(v: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..v.len() - 1 {
        for j in 0..v.len() - 1 - i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
    v.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sorting() {
        let mut arr = vec![5, 4, 1, 2, 3];
        assert_eq!(bubble_sorting(&mut arr), vec![1, 2, 3, 4, 5]);
    }
}
