/**
* I wanted to play some, and without copying, have a reference to my pivot element.
* This file was created only for this reason.

* The other version (quick_sort_in_place) is preferred.
*/

pub fn quick_sort<T: PartialOrd>(list: &mut [T])
{
    if list.len() < 2 {
        return;
    }
    quick_sort_core(list, 0, list.len() - 1);
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

/// Swaps the elements, and returns the index of the pivot element's proper place (and putting it there).
fn partition<T: PartialOrd>(list: &mut [T], low: usize, high: usize) -> usize {
    // Assigning list[high] to a variable would conflict with the borrow checker because
    // we would have an immutable reference the na mutable one later in the code.
    // Anyway, going unsafe here was sort of the goal, there are purposefully nothing else even
    // considered to be used, just unsafe code, direct memory accessing.

    // But why... Why will this work? Isn't pivot_ptr just pointing to an element of the list,
    // so technically something else could be put into that memory location?
    // Well, yes, but actually no. https://www.dailydot.com/unclick/well-yes-but-actually-no-meme/
    // At that position, the pivot element MUST be. By heuristics this means that the position of that element
    // should be written to by no means (ok the last step is an exception, but that is expected, and fine).

    let mut i = low;

    let list_ptr = list.as_mut_ptr();
    // Unsafe block to get a raw pointer to the pivot
    let pivot_ptr = unsafe { list_ptr.add(high) };

    unsafe {
        // Convert the raw pointer to a reference for comparison
        let pivot_value = &*pivot_ptr;

        for j in low..=high - 1 {
            let elem_ptr = list_ptr.add(j);
            if *elem_ptr < *pivot_value {
                // Swap elements using raw pointers
                std::ptr::swap(elem_ptr, list_ptr.add(i));
                i += 1;
            }
        }

        // Swap the pivot element into its correct position
        std::ptr::swap(list_ptr.add(i), pivot_ptr);
    }

    i
}

#[cfg(test)]
mod test {
    use crate::sorts::quick_sort_unsafe::quick_sort;

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
}
