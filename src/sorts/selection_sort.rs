/// Sorts in place. This sort is not stable.
pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    // looks for the minimum values at each iteration and swaps it.
    if list.len() < 2 {
        return;
    }

    for i in 0..list.len() - 1 {
        let mut min_index = i;

        for j in i + 1..list.len() {
            if list[j] >= list[min_index] {
                continue;
            }

            min_index = j;
        }

        if min_index == i {
            continue;
        }

        list.swap(i, min_index);
    }
}

/// Stable selection sort. This is basically also a schoolbook example.
/// It gets stable by preventing equal values to ever switch order.
/// Does it by putting the next element into the current smallest position
/// and shifting everything towards the end by one.
pub fn selection_sort_stable<T: PartialOrd>(list: &mut [T]) {
    // looks for the minimum values at each iteration and swaps it.
    if list.len() < 2 {
        return;
    }

    for i in 0..list.len() - 1 {
        let mut min_index = i;

        for j in i + 1..list.len() {
            if list[j] >= list[min_index] {
                continue;
            }

            min_index = j;
        }

        if min_index == i {
            continue;
        }

        // Using rotations here, so I can avoid having to place Copy requirement on T.
        list[i..=min_index].rotate_right(1);
    }
}

#[cfg(test)]
mod test {
    use crate::helpers::named_number::NamedNumber;
    use crate::sorts::selection_sort::{selection_sort, selection_sort_stable};

    #[test]
    fn selection_sort_test() {
        let mut vec = vec![1, 9, 8, 2, 7, 3, 6, 4, 5];
        selection_sort(&mut vec);

        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn selection_sort_test_1() {
        let mut vec = vec![1];
        selection_sort(&mut vec);

        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn selection_sort_test_2() {
        let mut vec = vec![2, 1];
        selection_sort(&mut vec);

        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn selection_sort_test2() {
        let mut vec = vec![
            NamedNumber::new(String::from("One"), 1),
            NamedNumber::new(String::from("Niner"), 9),
            NamedNumber::new(String::from("Eight"), 8),
            NamedNumber::new(String::from("Two"), 2),
            NamedNumber::new(String::from("Seven"), 7),
            NamedNumber::new(String::from("Tree"), 3),
            NamedNumber::new(String::from("Six"), 6),
            NamedNumber::new(String::from("Fower_a"), 4),
            NamedNumber::new(String::from("Fower_b"), 4),
            NamedNumber::new(String::from("Fife"), 5),
        ];
        selection_sort(&mut vec);

        assert_eq!(
            vec,
            vec![
                NamedNumber::new(String::from("One"), 1),
                NamedNumber::new(String::from("Two"), 2),
                NamedNumber::new(String::from("Tree"), 3),
                NamedNumber::new(String::from("Fower_a"), 4),
                NamedNumber::new(String::from("Fower_b"), 4),
                NamedNumber::new(String::from("Fife"), 5),
                NamedNumber::new(String::from("Six"), 6),
                NamedNumber::new(String::from("Seven"), 7),
                NamedNumber::new(String::from("Eight"), 8),
                NamedNumber::new(String::from("Niner"), 9),
            ]
        );
    }

    #[test]
    fn sorting_unstability() {
        let mut vec = vec![
            NamedNumber::new(String::from("Fower_a"), 4),
            NamedNumber::new(String::from("Fife"), 5),
            NamedNumber::new(String::from("Tree"), 3),
            NamedNumber::new(String::from("Two"), 2),
            NamedNumber::new(String::from("Fower_b"), 4),
            NamedNumber::new(String::from("One"), 1),
        ];

        selection_sort(&mut vec);

        assert_eq!(
            vec,
            vec![
                NamedNumber::new(String::from("One"), 1),
                NamedNumber::new(String::from("Two"), 2),
                NamedNumber::new(String::from("Tree"), 3),
                // Here b got in front of A because there was a smaller element after
                // "b" in the original list.
                NamedNumber::new(String::from("Fower_b"), 4),
                NamedNumber::new(String::from("Fower_a"), 4),
                NamedNumber::new(String::from("Fife"), 5),
            ]
        );
    }

    #[test]
    fn sorting_stability() {
        let mut vec = vec![
            NamedNumber::new(String::from("Fower_a"), 4),
            NamedNumber::new(String::from("Fife"), 5),
            NamedNumber::new(String::from("Tree"), 3),
            NamedNumber::new(String::from("Two"), 2),
            NamedNumber::new(String::from("Fower_b"), 4),
            NamedNumber::new(String::from("One"), 1),
        ];

        selection_sort_stable(&mut vec);

        assert_eq!(
            vec,
            vec![
                NamedNumber::new(String::from("One"), 1),
                NamedNumber::new(String::from("Two"), 2),
                NamedNumber::new(String::from("Tree"), 3),
                NamedNumber::new(String::from("Fower_a"), 4),
                NamedNumber::new(String::from("Fower_b"), 4),
                NamedNumber::new(String::from("Fife"), 5),
            ]
        );
    }
}
