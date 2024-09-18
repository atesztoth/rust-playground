use std::cmp;

// Classic, O(n) space complexity
pub fn merge_sort<T: PartialOrd + Clone>(list: &[T]) -> Vec<T> {
    if list.len() < 2 {
        return list.to_vec();
    }

    let (left, right) = list.split_at(list.len() / 2);

    merge(&merge_sort(left), &merge_sort(right))
}

fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let left: Vec<T> = left.to_vec();
    let right: Vec<T> = right.to_vec();
    let mut result: Vec<T> = Vec::with_capacity(left.len() + right.len());

    // left tracker, right tracker
    let mut lt = 0usize;
    let mut rt = 0usize;

    let boundary = cmp::max(left.len(), right.len());

    while lt < boundary || rt < boundary {
        let le = left.get(lt);
        let re = right.get(rt);

        match (le, re) {
            (Some(lv), Some(rv)) => {
                if lv <= rv {
                    result.push(lv.clone());
                    lt += 1;
                } else {
                    result.push(rv.clone());
                    rt += 1;
                }
            }
            (Some(v), None) => {
                result.push(v.clone());
                lt += 1;
            }
            (None, Some(v)) => {
                result.push(v.clone());
                rt += 1;
            }
            _ => break,
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::helpers::named_number::NamedNumber;
    use crate::sorts::merge_sort::merge_sort;

    #[test]
    fn merge_sort_test() {
        let vec = vec![3, 7, 8, 5, 4, 2, 6, 1];
        let res = merge_sort(&vec);

        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn merge_sort_test_2() {
        let vec = vec![12, 11, 13, 5, 6, 7];
        let res = merge_sort(&vec);

        assert_eq!(res, vec![5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn merge_sort_test_3() {
        let vec = vec![12];
        let res = merge_sort(&vec);

        assert_eq!(res, vec![12]);
    }

    #[test]
    fn merge_sort_test_4() {
        let vec = vec![1200, 12];
        let res = merge_sort(&vec);

        assert_eq!(res, vec![12, 1200]);
    }

    #[test]
    fn sorting_stability() {
        let vec = vec![
            NamedNumber::new(String::from("Fower_a"), 4),
            NamedNumber::new(String::from("Fife"), 5),
            NamedNumber::new(String::from("Tree"), 3),
            NamedNumber::new(String::from("Two"), 2),
            NamedNumber::new(String::from("Fower_b"), 4),
            NamedNumber::new(String::from("One"), 1),
        ];

        let sorted = merge_sort(&vec);

        assert_eq!(
            sorted,
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
