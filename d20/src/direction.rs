use crate::position::Position;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
    None
}
impl Direction{
    pub fn as_vector(&self)->Position<i32>{
        match self{
            Self::Up    => Position::new( 0,-1),
            Self::Down  => Position::new( 0, 1),
            Self::Left  => Position::new(-1, 0),
            Self::Right => Position::new( 1, 0),
            Self::None => Position::new(0, 0)
        }
    }
    pub fn rot_left(self)->Self{
        match self{
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Self::None => Self::None
        }
    }
    pub fn rot_right(self)->Self{
        match self{
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Self::None => Self::None
        }
    }
    pub fn flip(self)->Self{
        match self{
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Self::None => Self::None
        }
    }
}
impl PartialEq for Direction{
    fn eq(&self, other: &Self) -> bool {
        match (self,other){
            (Self::None, _) | (_, Self::None) => true,
            (Self::Up, Self::Up) | (Self::Down, Self::Down) | (Self::Left, Self::Left) |(Self::Right, Self::Right) => true,
            _ => false
        }
    }
}
