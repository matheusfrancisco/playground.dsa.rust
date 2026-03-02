use std::cmp::max;

//  this function receives two sorted vectors
// and merges them into a single sorted vector
fn merge_not_good(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let len = a.len() + b.len();
    //a [1, 3, 5] starts 0...2
    //b [2, 4, 6, 8, 9] starts 0...4
    // merged goes to 0....7 so
    let mut merged: Vec<i32> = Vec::with_capacity(len);
    let mut i = a.len();
    let mut j = b.len();
    // since we are receivin two sorted vector we can start from
    // the end of both vectors and compare the last elements
    while i > 0 && j > 0 {
        if a[i - 1] > b[j - 1] {
            merged.push(a[i - 1]);
            i -= 1;
        } else {
            merged.push(b[j - 1]);
            j -= 1;
        }
    }
    // if there are remaining elements in a or b we need to add them to merged
    while i > 0 {
        merged.push(a[i - 1]);
        i -= 1;
    }
    while j > 0 {
        merged.push(b[j - 1]);
        j -= 1;
    }
    // since we were pushing the largest elements first we need to reverse the merged vector
    merged.reverse();
    merged
}

// this merge takes 2n time and n space since we are
// creating a new vector to store the merged result
fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let len = a.len() + b.len();
    let mut merged: Vec<i32> = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    //while i < a.len() {
    //    merged.push(a[i]);
    //    i += 1;
    //}
    //while j < b.len() {
    //    merged.push(b[j]);
    //    j += 1;
    //}
    merged.extend_from_slice(&a[i..]);
    merged.extend_from_slice(&b[j..]);
    merged
}

//bad implementation of merge using recursion
// this implementation is bad because it creates a
// new vector for each recursive call and it also uses slicing which creates a new vector as well
fn merge_recursive(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }
    if a[0] < b[0] {
        let mut merged = vec![a[0]];
        merged.extend(merge_recursive(a[1..].to_vec(), b));
        merged
    } else {
        let mut merged = vec![b[0]];
        merged.extend(merge_recursive(a, b[1..].to_vec()));
        merged
    }
}

fn merge_recursive_optimized(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    fn helper(a: &[i32], b: &[i32], i: usize, j: usize, merged: &mut Vec<i32>) {
        // base case
        if i == a.len() {
            merged.extend_from_slice(&b[j..]);
            return;
        }
        if j == b.len() {
            merged.extend_from_slice(&a[i..]);
            return;
        }
        if a[i] < b[j] {
            merged.push(a[i]);
            helper(a, b, i + 1, j, merged);
        } else {
            merged.push(b[j]);
            helper(a, b, i, j + 1, merged);
        }
    }
    let mut merged = Vec::with_capacity(a.len() + b.len());
    helper(&a, &b, 0, 0, &mut merged);
    merged
}

fn merge2(a: &mut [i32], mid: usize) {
    let n = a.len();
    if mid == 0 || mid >= n {
        return;
    }
    let left = a[..mid].to_vec();
    let mut i = 0;
    let mut j = mid;
    let mut k = 0;

    while i < left.len() && j < n {
        if left[i] < a[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = a[j];
            j += 1;
        }
        k += 1;
    }
}

// worst case time complexity is O(n^2) since we are
// rotating the array for each element in the left half
// 1. Rotation-based merge (O(1) space, O(n²) worst case)
fn merge3(a: &mut [i32], mid: usize) {
    let n = a.len();
    if mid == 0 || mid >= n {
        return;
    }

    let mut i = 0;
    let mut j = mid;
    let mut left_end = mid;

    while i < left_end && j < n {
        if a[i] <= a[j] {
            i += 1;
        } else {
            a[i..=j].rotate_right(1);
            i += 1;
            left_end += 1;
            j += 1;
        }
    }
}

fn merge_inplace_insertion(a: &mut [i32], mut mid: usize) {
    let n = a.len();
    if mid == 0 || mid >= n {
        return;
    }

    if mid <= n - mid {
        // Left is smaller: insert each left[i] into the right run
        let mut i = 0;
        while i < mid {
            let x = a[i];
            let right = &a[mid..];
            let j = right.partition_point(|&r| r < x);
            if j < right.len() {
                // Rotate: move a[i] to position mid+j
                // Segment a[i..=mid+j], rotate left by (mid+j-i) puts a[i] at end
                a[i..=mid + j].rotate_left(mid + j - i);
                mid += 1; // boundary shifted
            }
            i += 1;
        }
    } else {
        // Right is smaller: insert each right[j] into the left run (from the end)
        let mut j = mid;
        while j < n {
            let x = a[j];
            let left = &a[..mid];
            let i = left.partition_point(|&l| l <= x);
            if i < mid {
                // Rotate: move a[j] to position i
                a[i..=j].rotate_right(j - i);
                mid += 1;
            }
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_not_good() {
        let a: Vec<i32> = vec![1, 3, 5];
        let b: Vec<i32> = vec![2, 4, 6, 8, 9];
        let merged = merge_not_good(a, b);
        let c: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(merged, c);
    }

    #[test]
    fn test_merge_good() {
        let a: Vec<i32> = vec![1, 3, 5];
        let b: Vec<i32> = vec![2, 4, 6, 8, 9];
        let merged = merge(a, b);
        let c: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(merged, c);
    }

    #[test]
    fn test_merge_recursive() {
        let a: Vec<i32> = vec![1, 3, 5];
        let b: Vec<i32> = vec![2, 4, 6, 8, 9];
        let merged = merge_recursive(a, b);
        let c: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(merged, c);
    }

    #[test]
    fn test_merge_recursive_optimized() {
        let a: Vec<i32> = vec![1, 3, 5];
        let b: Vec<i32> = vec![2, 4, 6, 8, 9];
        let merged = merge_recursive_optimized(a, b);
        let c: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(merged, c);
    }

    #[test]
    fn test_merge2() {
        let mut a: Vec<i32> = vec![1, 3, 5, 2, 4, 6, 8, 9];
        merge2(&mut a, 3);
        let c: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(a, c);
    }

    #[test]
    fn test_merge3() {
        let mut a: Vec<i32> = vec![1, 3, 5, 2, 4, 6, 8, 9];
        merge3(&mut a, 3);
        let c: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(a, c);
    }
}

