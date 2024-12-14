pub mod cache{
    use std::collections::HashMap;
    use std::hash::Hash;
    struct Cache<IdNum,CacheNum>{
        cache: HashMap::<IdNum,[Option<CacheNum>;75]>
    }
    impl <IdNum:Copy+Eq+Hash,CacheNum:Copy>Cache<IdNum,CacheNum>{
        fn new()->Self{
            let cache = HashMap::new();
            Self{cache}
        }
        fn get(&self,key:&IdNum, iteration: &usize)->Option<CacheNum>{
            if let Some(arr) = self.cache.get(key){
                arr[*iteration]
            }else{
                None
            }
        }
        fn insert(&mut self, key:&IdNum, iteration: &usize, value: CacheNum)->Option<CacheNum>{
            if let Some(arr) = self.cache.get_mut(key){
                if let Some(previously_existing) = arr[*iteration]{
                    Some(previously_existing)
                }else{
                    arr[*iteration] = Some(value);
                    None
                }
            }else{
                let mut new_arr = [None;75];
                new_arr[*iteration] = Some(value);
                self.cache.insert(*key, new_arr);
                None
            }
        }
    }
}

pub mod position{
    use std::{fmt::Display, ops::{Add, Mul, Sub}};


    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Position<IntType>{
        pub x: IntType,
        pub y: IntType
    }
    impl <IntType>Position<IntType>{
        pub fn new(x:IntType,y:IntType)->Self{
            Self{x,y}
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

    }
    impl <IntType>Add for Position<IntType>
        where IntType: Add<Output=IntType>{
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x+rhs.x, self.y+rhs.y)
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
}

pub mod matrix{
    use crate::position::Position;
    pub struct Matrix<T>{
        width: usize,
        height: usize,
        data: Vec<T>,
        iter_idx: usize,
    }
    impl <T:Copy>Matrix<T>{
        pub fn new_from_square(data:Vec<Vec<T>>)->Self{
            let height = data.len();
            let width = data[0].len();
            let data = data.into_iter().flat_map(|row| row).collect();
            Self::new_from_flat(width, height, data)
        }
        pub fn new_from_flat(width: usize, height: usize, data:Vec<T>)->Self{
            Self { width, height, data , iter_idx:0}
        }
        fn pos_to_idx(&self,pos:Position<usize>)->Option<usize>{
            if pos.x() >= self.width || pos.y() >= self.height{
                return None;
            }
            Some(pos.y()*self.width+pos.x())
        }
        pub fn idx_to_pos(&self,idx:usize)->Option<Position<usize>>{
            if idx < self.data.len(){
                let y = idx / self.width;
                let x = idx % self.width;
                Some(Position::new(x, y))
            }else{
                None
            }
        }
        pub fn get(&self, pos: Position<usize>)->Option<T>{
            let idx = self.pos_to_idx(pos)?;
            self.data.get(idx).copied()
        }
        pub fn set(&mut self, pos: Position<usize>, value: T){
            if let Some(idx) = self.pos_to_idx(pos){
                self.data[idx] = value;
            }
        }
        pub fn width(&self)->usize{
            self.width
        }
        pub fn height(&self)->usize{
            self.height
        }
        pub fn clone_data(&self)->Vec<T>{
            self.data.clone()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
