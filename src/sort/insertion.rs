/// sort a mutable vec using in-place insertion sort algorithm.
///
/// Insertion Sort
/// George T. Heineman et al. Algorithms in a Nutshell. O'Reilly Media, Inc., 2008
#[allow(dead_code)]
pub fn sort<T>(a: &mut [T])
where
    T: PartialOrd + Copy,
{
    for i in 1..a.len() {
        insert(a, i, a[i]);
    }
}

#[allow(dead_code)]
fn insert<T>(a: &mut [T], pos: usize, value: T)
where
    T: PartialOrd + Copy,
{
    let mut i = pos - 1;
    while a[i] > value {
        a[i + 1] = a[i];
        if i == 0 {
            break;
        }
        i -= 1;
    }
    if i == 0 && a[0] > value {
        a[0] = value;
    } else {
        a[i + 1] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn basic_char() {
        let mut arr: Vec<char> = vec!['e', 'd', 'b', 'a', 'c'];
        let expected: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn already_sorted_char() {
        let mut arr: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        let expected: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn basic_number() {
        let mut arr: Vec<i32> = vec![5, 4, 2, 1, 3];
        let expected: Vec<i32> = vec![1, 2, 3, 4, 5];
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn already_sorted_number() {
        let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: Vec<i32> = vec![1, 2, 3, 4, 5];
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn basic_str() {
        let mut arr: [&str; 4] = ["xyz", "ijk", "abc", "def"];
        let expected: [&str; 4] = ["abc", "def", "ijk", "xyz"];
        sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
