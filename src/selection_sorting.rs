use rand::seq::SliceRandom;
use rand::thread_rng;
/*
* Selection selection_sort
* input: mutable vector of i32 [5, 4, 1, 2, 3]
* output: sorted vector of i32 [1, 2, 3, 4, 5]
* */
fn selection_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() - 1 {
        let mut smallest = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[smallest] {
                smallest = j;
            }
        }
        vec.swap(smallest, i);
    }
    vec.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![5, 4, 1, 2, 3];
        assert_eq!(selection_sort(&mut arr), vec![1, 2, 3, 4, 5]);

        let mut arr = vec![5, 4, 2, 3];
        assert_eq!(selection_sort(&mut arr), vec![2, 3, 4, 5]);

        let mut sorted_arr = Vec::new();
        let mut arr = Vec::new();
        for i in 0..100 {
            arr.push(i);
            sorted_arr.push(i);
        }

        arr.shuffle(&mut thread_rng());
        assert_eq!(selection_sort(&mut arr), sorted_arr);
    }
}
