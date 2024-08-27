// The algorithm provides stable sorting.

pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    for i in 1..list.len() {
        let lo = i - 1; // lo: last ordered (last assumed ordered ~)

        // The point now is to find a place for our element.
        // That will be one right from the first element that smaller from it.

        // This is the position where I want my element to be put.
        let target_pos: usize = (0..=lo)
            .rev()
            .find_map(|x| if list[x] > list[i] { None } else { Some(x + 1) })
            .unwrap_or(0);

        list[target_pos..=i].rotate_right(1);
    }
}

#[cfg(test)]
mod test {
    use crate::helpers::named_number::NamedNumber;
    use crate::sorts::insertion_sort::insertion_sort;

    #[test]
    fn insertion_sort_test() {
        let mut vec = vec![12, 11, 13, 5, 6];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![5, 6, 11, 12, 13]);
    }

    #[test]
    fn insertion_sort_test_1() {
        let mut vec = vec![1];
        insertion_sort(&mut vec);

        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn insertion_sort_test_2() {
        let mut vec = vec![2, 1];
        insertion_sort(&mut vec);

        assert_eq!(vec, vec![1, 2]);
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

        insertion_sort(&mut vec);

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
