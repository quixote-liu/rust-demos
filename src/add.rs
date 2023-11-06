use std::ops::Add;

#[derive(Debug)]
pub struct Point<T: Add<Output = T>> {
    pub x: T,
    pub y: T,
}

impl <T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

