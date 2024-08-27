#![allow(unused_variables)]

use crate::sorts::quick_sort::{quick_sort, quick_sort_into};
use crate::sorts::quick_sort_unsafe::quick_sort as quick_sort_unsafe;

mod helpers;
mod sorts;

fn main() {
    println!("Start");

    quick_sort_example();

    println!("End");
}

fn quick_sort_example() {
    let numbers: Vec<u8> = vec![2, 4, 1, 7, 8, 3, 9, 6, 5];
    let mut numbers2: Vec<u8> = vec![2, 4, 1, 7, 8, 3, 9, 6, 5];
    let mut numbers3: Vec<u8> = vec![2, 4, 1, 7, 8, 3, 9, 6, 5];

    quick_sort(&mut numbers2);
    quick_sort_unsafe(&mut numbers3);
    let sorted_numbers = quick_sort_into(&numbers);

    assert_ne!(numbers, sorted_numbers);

    assert_eq!(numbers, vec![2, 4, 1, 7, 8, 3, 9, 6, 5]);
    assert_eq!(numbers2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(numbers3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(sorted_numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

// Random:

/// I though what a good idea it will be to write a nice swap function that
/// is generic over T. Then stuff happened... 😄 Functions were discovered.
/// Pain was felt... and then problems were solved.
/// Then hit 2: vec.swap. :'( That is the idiomatic way to go.
/// Anyway, at least I gained some information along the way.
#[inline]
#[allow(dead_code)]
fn swap<T>(a: &mut T, b: &mut T) -> () {
    // I don't want to copy the values. Why would I do that? Waste of resources.
    // Like I could say T: Copy + Clone, or just Clone, and
    // let aa = a.clone();
    // *a = *b;
    // *b = aa;
    // But this would require this clone call. Instead,
    // I could use the swap provided by the standard library:

    std::mem::swap(a, b);

    // another way:
    // (leaving it here because it's an example I want to have around)
    // unsafe {
    //     let tmp = std::ptr::read(a);
    //     std::ptr::write(a, std::ptr::read(b));
    //     std::ptr::write(b, tmp);
    // }
}

#[cfg(test)]
mod test {
    use crate::swap;

    #[test]
    fn test_swap() {
        let mut x = 5;
        let mut y = 3;

        swap(&mut x, &mut y);

        assert_eq!(x, 3);
        assert_eq!(y, 5);
    }

    #[test]
    fn swap_vector_elements() {
        let mut a = vec![1, 2, 3];

        let (mut_slice_a, mut_slice_b) = a.split_at_mut(1);
        swap(&mut mut_slice_a[0], &mut mut_slice_b[0]);

        assert_eq!(a, vec![2, 1, 3]);

        // This just feels like cheating:
        a.swap(1, 2);
        // Too easy, too simple, too fast.
        // I don't know if I should love or hate it. 😅
        assert_eq!(a, vec![2, 3, 1]);
    }
}
