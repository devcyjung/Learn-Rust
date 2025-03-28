#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod rectangle_tests {

    #[test]
    fn larger_and_smaller() {
        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = super::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn not_larger_or_smaller() {
        let not_larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let not_smaller = super::Rectangle {
            width: 5,
            height: 8,
        };

        assert!(!not_smaller.can_hold(&not_larger));
        assert!(!not_larger.can_hold(&not_smaller));
    }

    #[test]
    fn equal_size() {
        let eq_1 = super::Rectangle {
            width: 10,
            height: 2,
        };
        let eq_2 = super::Rectangle {
            width: 10,
            height: 2,
        };

        assert!(eq_1.can_hold(&eq_2));
        assert!(eq_2.can_hold(&eq_1));
    }
}
