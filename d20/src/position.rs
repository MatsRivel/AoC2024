use std::{fmt::Display, ops::{Add, Mul, Sub}};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Position<IntType>{
    pub x: IntType,
    pub y: IntType
}
impl <IntType>Position<IntType>{
    pub fn new(x:IntType,y:IntType)->Self{
        Self{x,y}
    }

}
impl <IntType:Eq>Position<IntType>{

    pub fn is_equal_to(&self, other: &Self)->bool{
        self.x == other.x && self.y == other.y
    }
}
impl <IntType>Position<IntType>
where IntType: Copy{
    pub fn x(&self)->IntType{
        self.x
    }
    pub fn y(&self)->IntType{
        self.y
    }
}
impl <IntType>Position<IntType>
 where  i32: From<IntType> + Into<IntType>,
 IntType: From<i32> + Into<i32> + PartialEq + Copy + std::fmt::Debug + Add
 {
    fn adjusted(&self, dy:i32,dx:i32)->Option<Self>{
        match (self.y != 0.into() || dy != -1) && (self.x != 0.into() || dx != -1){
            true => {
                let new_x = i32::from(self.x ) + dx;
                let new_y = i32::from(self.y ) + dy;
                debug_assert!(new_x >= 0);
                debug_assert!(new_y >= 0);
                Some(Self::new((self.x.into() + dx).into(), (self.y.into()+dy).into()))},
            false => None,
        }
    }
    pub fn neighbours(&self)->[Option<Position<IntType>>;4]{
        [[-1,0],[1,0],[0,-1],[0,1]].into_iter().map(|[dy,dx]| self.adjusted(dy, dx)).collect::<Vec<Option<Position<IntType>>>>().try_into().unwrap()
    }
    pub fn abs_diff(&self,other:&Self)->i32{
        (i32::from(self.x())-i32::from(other.x())).abs() + (i32::from(self.y())-i32::from(other.y())).abs()
    }

}
impl <IntType>Add for Position<IntType>
    where IntType: Add<Output=IntType>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl <IntType>Sub for Position<IntType>
where IntType: Sub<Output=IntType>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x-rhs.x, self.y-rhs.y)
    }
}
impl <IntType>Mul<IntType> for Position<IntType>
where IntType: Copy + Mul<Output=IntType>{
    type Output = Self;

    fn mul(self, rhs: IntType) -> Self::Output {
        Self::new(self.x()*rhs, self.y()*rhs)
    }
}
impl <IntType>Display for Position<IntType>
where IntType: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"(x: {}, y: {})", self.x,self.y)
    }
}
impl <IntType>TryFrom<(usize,usize)> for Position<IntType>
where IntType: TryFrom<usize>,
        usize: TryFrom<IntType>{
    type Error = <IntType as TryFrom<usize>>::Error;
    fn try_from(value: (usize,usize)) -> Result<Self, Self::Error> {
        let x= value.0.try_into()?;
        let y = value.1.try_into()?;
        Ok( Self::new(x, y ))
    }
}
#[cfg(test)]
mod pos_tests{
    use crate::{IntType, direction::Direction};
    use super::*;
    use test_case::test_case;
    #[test_case( Position::new(0,0), Direction::Up, Position::new(0,-1) ; "OrigoUp")]
    #[test_case( Position::new(0,0), Direction::Down, Position::new(0,1) ; "OrigoDown")]
    #[test_case( Position::new(0,0), Direction::Left, Position::new(-1,0) ; "OrigoLeft")]
    #[test_case( Position::new(0,0), Direction::Right, Position::new(1,0) ; "OrigoRight")]
    #[test_case( Position::new(1,2), Direction::Up, Position::new(1,1) ; "Up")]
    #[test_case( Position::new(1,2), Direction::Down, Position::new(1,3) ; "Down")]
    #[test_case( Position::new(1,2), Direction::Left, Position::new(0,2) ; "Left")]
    #[test_case( Position::new(1,2), Direction::Right, Position::new(2,2) ; "Right")]
    fn test_pos_dir_interactions(start: Position<IntType>, dir: Direction, end: Position<IntType>){
        assert_eq!(end,start+dir.as_vector())
    }

    #[test_case( Position::new(1,2), Position::new(2,2), false ; "NotEq")]
    #[test_case( Position::new(1,2), Position::new(1,2), true ; "Eq")]
    fn equals_test(start: Position<IntType>, end: Position<IntType>, is_equal: bool){
        assert_eq!(start == end, is_equal);
        assert!(start.is_equal_to(&end) == is_equal);
        assert!(end.is_equal_to(&start) == is_equal);
    }
}
