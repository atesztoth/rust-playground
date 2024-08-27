// The algorithm provides stable sorting.

pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    for i in 0..list.len() - 1 {
        let mut did_swap = false;
        for j in 0..list.len() - i - 1 {
            if list[j] <= list[j + 1] {
                continue;
            }

            list.swap(j, j + 1);
            did_swap = true;
        }

        if !did_swap {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sorts::bubble_sort::bubble_sort;

    #[test]
    fn bubble_test() {
        let mut vec = vec![1, 9, 8, 2, 7, 3, 6, 4, 5];
        bubble_sort(&mut vec);

        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn bubble_test_1() {
        let mut vec = vec![1];
        bubble_sort(&mut vec);

        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn bubble_test_2() {
        let mut vec = vec![2, 1];
        bubble_sort(&mut vec);

        assert_eq!(vec, vec![1, 2]);
    }
}
