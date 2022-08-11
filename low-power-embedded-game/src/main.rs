#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

#[derive()]
pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        (0 - self.0).abs() + (0 - self.1).abs()
    }
}


fn main() {
    let mut even_ints = evens(0_u8..);
    assert_eq!(even_ints.next(), Some(0));
    assert_eq!(even_ints.next(), Some(2));
    assert_eq!(even_ints.next(), Some(4));
    assert_eq!(even_ints.next(), Some(6));

    assert_eq!(Position(3, 4).manhattan(), 7);

}
