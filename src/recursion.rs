fn toh(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    return toh(n - 1) + 1 + toh(n - 1);
}

fn factorial_iter(num: i32) -> i32 {
    let mut fat = 1;
    for i in 1..=num {
        fat *= i;
    }
    fat
}

fn factorial_1(num: i32) -> i32 {
    if num > 1 {
        return num * factorial_1(num - 1);
    } else {
        return 1;
    }
}

fn fib_rec(num: i32) -> i32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        return fib_rec(num - 1) + fib_rec(num - 2);
    }
}

fn fib_mem(num: i32, memo: &mut Vec<i32>) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        _ => {
            if memo[num as usize] != -1 {
                return memo[num as usize];
            } else {
                memo[num as usize] = fib_mem(num - 1, memo) + fib_mem(num - 2, memo);
                return memo[num as usize];
            }
        }
    }
}

// Fibonacci using a loop
pub fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

fn palindrome(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }
    if array[start] == array[end] {
        return palindrome(array, start + 1, end - 1);
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_factorial() {
        let cases = vec![(5, 120), (4, 24), (3, 6), (2, 2), (1, 1), (0, 1)];
        for (num, expected) in cases {
            assert_eq!(factorial_1(num), expected);
        }
    }

    #[test]
    fn test_factorial_iter() {
        let cases = vec![(5, 120), (4, 24), (3, 6), (2, 2), (1, 1), (0, 1)];
        for (num, expected) in cases {
            assert_eq!(factorial_iter(num), expected);
        }
    }

    #[test]
    fn test_fib_rec() {
        let cases = vec![(6, 8), (5, 5), (4, 3), (3, 2), (2, 1), (1, 1)];
        for (num, expected) in cases {
            assert_eq!(fib_rec(num), expected);
        }
    }

    #[test]
    fn test_fib_mem() {
        let cases = vec![(6, 8), (5, 5), (4, 3), (3, 2), (2, 1), (1, 1)];
        for (num, expected) in cases {
            let mut v = Vec::<i32>::with_capacity((num + 1) as usize);
            for i in 0..(num + 1) {
                v.push(-1);
            }
            assert_eq!(fib_mem(num, &mut v), expected);
        }
    }

    #[test]
    fn test_palindrome() {
        let array = vec![1, 2, 3, 4, 3, 2, 1];
        assert_eq!(palindrome(&array, 0, array.len() - 1), true);
    }

    #[test]
    fn test_toh() {
        assert_eq!(toh(0), 0);
        assert_eq!(toh(1), 1);
        assert_eq!(toh(3), 7);
        assert_eq!(toh(4), 15);
    }
}
