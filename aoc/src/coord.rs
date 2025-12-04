use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: Ord> Ord for Coordinate<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl<T: Ord> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Coordinate<usize> {
    #[must_use]
    pub fn adjacent(self) -> [Option<Self>; 4] {
        let up = (self.y > 0).then(|| self - Self { x: 0, y: 1 });
        let right = (self.x < usize::MAX).then(|| self + Self { x: 1, y: 0 });
        let down = (self.y < usize::MAX).then(|| self + Self { x: 0, y: 1 });
        let left = (self.x > 0).then(|| self - Self { x: 1, y: 0 });
        [up, right, down, left]
    }

    #[must_use]
    pub fn surrounding(self) -> [Option<Self>; 8] {
        let up = (self.y > 0).then(|| self - Self { x: 0, y: 1 });
        let topright = (self.x < usize::MAX && self.y > 0)
            .then(|| self + Self { x: 1, y: 0 } - Self { x: 0, y: 1 });
        let right = (self.x < usize::MAX).then(|| self + Self { x: 1, y: 0 });
        let bottomright =
            (self.x < usize::MAX && self.y < usize::MAX).then(|| self + Self { x: 1, y: 1 });
        let down = (self.y < usize::MAX).then(|| self + Self { x: 0, y: 1 });
        let bottomleft = (self.x > 0 && self.y < usize::MAX)
            .then(|| self - Self { x: 1, y: 0 } + Self { x: 0, y: 1 });
        let left = (self.x > 0).then(|| self - Self { x: 1, y: 0 });
        let topleft = (self.x > 0 && self.y > 0).then(|| self - Self { x: 1, y: 1 });
        [
            up,
            topright,
            right,
            bottomright,
            down,
            bottomleft,
            left,
            topleft,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let coord = Coordinate::new(3, 5);
        assert_eq!(coord.x, 3);
        assert_eq!(coord.y, 5);
    }

    #[test]
    fn test_add() {
        let a = Coordinate::new(2, 3);
        let b = Coordinate::new(5, 7);
        assert_eq!(a + b, Coordinate::new(7, 10));
    }

    #[test]
    fn test_sub() {
        let a = Coordinate::new(8, 6);
        let b = Coordinate::new(2, 4);
        assert_eq!(a - b, Coordinate::new(6, 2));
    }

    #[test]
    fn test_adjacent() {
        let coord = Coordinate::new(1, 1);
        let adj = coord.adjacent();

        assert_eq!(adj[0], Some(Coordinate::new(1, 0))); // up
        assert_eq!(adj[1], Some(Coordinate::new(2, 1))); // right
        assert_eq!(adj[2], Some(Coordinate::new(1, 2))); // down
        assert_eq!(adj[3], Some(Coordinate::new(0, 1))); // left
    }

    #[test]
    fn test_adjacent_at_zero() {
        let coord = Coordinate::new(0, 0);
        let adj = coord.adjacent();

        assert_eq!(adj[0], None); // up
        assert_eq!(adj[1], Some(Coordinate::new(1, 0))); // right
        assert_eq!(adj[2], Some(Coordinate::new(0, 1))); // down
        assert_eq!(adj[3], None); // left
    }

    #[test]
    fn test_surrounding() {
        let coord = Coordinate::new(1, 1);
        let surr = coord.surrounding();

        assert_eq!(surr[0], Some(Coordinate::new(1, 0))); // up
        assert_eq!(surr[1], Some(Coordinate::new(2, 0))); // topright
        assert_eq!(surr[2], Some(Coordinate::new(2, 1))); // right
        assert_eq!(surr[3], Some(Coordinate::new(2, 2))); // bottomright
        assert_eq!(surr[4], Some(Coordinate::new(1, 2))); // down
        assert_eq!(surr[5], Some(Coordinate::new(0, 2))); // bottomleft
        assert_eq!(surr[6], Some(Coordinate::new(0, 1))); // left
        assert_eq!(surr[7], Some(Coordinate::new(0, 0))); // topleft
    }

    #[test]
    fn test_ord() {
        let a = Coordinate::new(1, 2);
        let b = Coordinate::new(1, 3);
        let c = Coordinate::new(2, 1);

        assert!(a < b);
        assert!(b < c);
    }
}
