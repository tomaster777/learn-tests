#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
