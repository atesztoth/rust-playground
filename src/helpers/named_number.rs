use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct NamedNumber {
    #[allow(dead_code)]
    name: String,
    value: i8,
}

impl NamedNumber {
    pub fn new(name: String, value: i8) -> Self {
        NamedNumber { name, value }
    }
}

impl PartialEq<Self> for NamedNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.name == other.name
    }
}

impl PartialOrd for NamedNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}

#[cfg(test)]
mod test {
    use crate::helpers::named_number::NamedNumber;
    use std::cmp::Ordering;
    use std::sync::LazyLock;

    static ONE_A: LazyLock<NamedNumber> =
        LazyLock::new(|| NamedNumber::new(String::from("One_a"), 1));

    static ONE_B: LazyLock<NamedNumber> =
        LazyLock::new(|| NamedNumber::new(String::from("One_b"), 1));

    static TWO: LazyLock<NamedNumber> = LazyLock::new(|| NamedNumber::new(String::from("Two"), 2));

    #[test]
    fn partial_eq() {
        assert_eq!(*ONE_A, *ONE_B);
    }

    #[test]
    fn partial_cmp() {
        assert_eq!((*ONE_A).partial_cmp(&TWO), Some(Ordering::Less));
        assert_eq!((*TWO).partial_cmp(&ONE_A), Some(Ordering::Greater));
        assert_eq!((*TWO).partial_cmp(&TWO), Some(Ordering::Equal));
    }

    #[test]
    fn lt() {
        assert!(*ONE_A < *TWO);
        // Do not use De-Morgan here, point is using the "<" operator...
        // In analogous cases, it will not be mentioned.
        assert!(!(*TWO < *ONE_A));
    }

    #[test]
    fn le() {
        assert!(*ONE_A <= *TWO);
        assert!(*ONE_A <= *ONE_A);
        assert!(!(*TWO <= *ONE_A));
    }

    #[test]
    fn gt() {
        assert!(*TWO > *ONE_A);
        assert!(!(*ONE_A > *TWO));
    }

    #[test]
    fn ge() {
        assert!(*TWO >= *ONE_A);
        assert!(*ONE_A >= *ONE_A);
        assert!(!(*ONE_A >= *TWO));
    }
}
