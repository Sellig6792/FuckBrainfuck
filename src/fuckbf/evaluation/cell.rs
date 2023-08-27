// A cyclic integer is a number that is always in the range. If it is greater than the maximum value, it will be set to the minimum value. If it is less than the minimum value + the overflow, it will be set to the maximum value.

use num::PrimInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Cell {
    value: u8,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Cell { value }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn add<T: PrimInt>(&mut self, value: T) {
        let sum: usize = self.value as usize + value.to_usize().unwrap();
        self.value = (sum % (u8::MAX as usize + 1)) as u8;
    }

    pub fn sub<T: PrimInt>(&mut self, value: T) {
        let sub: isize = self.value as isize - value.to_isize().unwrap();
        self.value = if sub < 0 {
            u8::MAX - (sub.unsigned_abs() as u8 - 1)
        } else {
            sub as u8
        };
    }
}

impl PartialEq<u8> for Cell {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cell = Cell::new(5);
        cell.add(5);
        assert_eq!(cell.get_value(), 10);
    }

    #[test]
    fn test_add_overflow() {
        let mut cell = Cell::new(255);
        cell.add(1);
        assert_eq!(cell.get_value(), 0);
    }

    #[test]
    fn test_sub() {
        let mut cell = Cell::new(5);
        cell.sub(5);
        assert_eq!(cell.get_value(), 0);
    }

    #[test]
    fn test_sub_overflow() {
        let mut cell = Cell::new(5);
        cell.sub(10);
        assert_eq!(cell.get_value(), 251);
    }
}
