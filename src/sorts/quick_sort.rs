use crate::helpers::slice_extension::SliceExtension;

/// Sorts the argument vector in place.
pub fn quick_sort<T: PartialOrd>(list: &mut [T]) {
    if list.len() < 2 {
        return;
    }
    quick_sort_core(list, 0, list.len() - 1);
}

pub fn quick_sort_into<T: PartialOrd + Clone>(list: &[T]) -> Vec<T> {
    let mut vec = list.to_vec();
    quick_sort(&mut vec);
    vec
}

fn quick_sort_core<T: PartialOrd>(list: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let split_point = partition(list, low, high); // AKA pivot index. The point where "partitioning" should happen.

    if split_point > 0 {
        quick_sort_core(list, low, split_point - 1);
    }
    quick_sort_core(list, split_point + 1, high);
}

/// This is based on the Lomuto algorithm.
/// Swaps the elements, and returns the index of the pivot element's proper place (and putting it there).
fn partition<T: PartialOrd>(list: &mut [T], low: usize, high: usize) -> usize {
    // Cannot be used because of the borrow checker.
    // let pivot = &list[high];
    let mut i = low;

    // Putting smaller values to the left of the pivot.
    for j in low..=high - 1 {
        if list[j] < list[high] {
            list.swap_if_needed(i, j);
            i += 1;
        }
    }

    list.swap_if_needed(i, high);
    i
}

#[cfg(test)]
mod test {
    use crate::sorts::quick_sort::{quick_sort, quick_sort_into};

    #[test]
    fn quick_sort_in_place_test() {
        // let mut numbers: Vec<u8> = vec![2, 4, 1, 7, 8, 3, 9, 6, 5];
        let mut numbers: Vec<u8> = vec![10, 7, 8, 9, 1, 5];
        quick_sort(&mut numbers);

        assert_eq!(numbers, vec![1, 5, 7, 8, 9, 10]);
    }

    #[test]
    fn quick_sort_in_place_test2() {
        let mut numbers: Vec<u8> = vec![2, 4, 1, 7, 8, 3, 9, 6, 5];

        quick_sort(&mut numbers);

        assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn quick_sort_in_place_test3() {
        let mut numbers: Vec<u8> = vec![1, 6, 7, 2, 5];

        quick_sort(&mut numbers);

        assert_eq!(numbers, vec![1, 2, 5, 6, 7]);
    }

    #[test]
    fn quick_sort_in_place_test4() {
        let mut numbers: Vec<u8> = vec![1, 2, 3, 4, 5];

        quick_sort(&mut numbers);

        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn quick_sort_empty() {
        let mut numbers: Vec<u8> = vec![];
        quick_sort(&mut numbers);

        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn quick_sort_single_element() {
        let mut numbers: Vec<u8> = vec![10];
        quick_sort(&mut numbers);

        assert_eq!(numbers, vec![10]);
    }

    #[test]
    fn quick_sort_into_test() {
        let mut numbers: Vec<u8> = vec![1, 6, 7, 2, 5];

        let results = quick_sort_into(&mut numbers);

        assert_ne!(numbers, results);
        assert_eq!(numbers, vec![1, 6, 7, 2, 5]);
        assert_eq!(results, vec![1, 2, 5, 6, 7]);
    }
}
