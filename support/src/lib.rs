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

pub mod direction{
    use std::cmp::Ordering;

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
    impl <IntType>TryFrom<(IntType,IntType)> for Position<IntType>{
        type Error = ();
        fn try_from(value: (IntType,IntType)) -> Result<Self, Self::Error> {
            let x= value.0;
            let y = value.1;
            Ok( Self::new(x, y ))
        }
    }

}

pub mod matrix{
    use std::{collections::HashMap, fmt::Display, marker::PhantomData};
    use crate::position::Position;

    pub trait GetSet<KeyType,ValueType>{
        fn get(&self, key: KeyType)-> Option<ValueType>;
        fn set(&mut self, key: KeyType, value: ValueType);
    }
    pub trait TypeToIndex<IndexType>{
        fn to_idx(&self, input: IndexType)->Option<usize>;
        fn from_idx(&self, idx: usize)->Option<IndexType>;
    }

    #[derive(Clone)]
    pub struct Matrix<DataType,IntType, StorageType, IndexType>{
        index_type: PhantomData<IndexType>,
        data_type: PhantomData<DataType>,
        int_type: PhantomData<IntType>,
        width: usize,
        height: usize,
        data: StorageType,
    }
    // First General Impl
    impl <DataType, IntType, StorageType, IndexType> Matrix<DataType, IntType, StorageType, IndexType>
{
        pub fn width(&self)->usize{
            self.width
        }
        pub fn height(&self)->usize{
            self.height
        }

    }
    impl <DataType, IntType, StorageType, IndexType> Matrix<DataType, IntType, StorageType, IndexType>
        where StorageType: Clone{
        pub fn clone_data(&self)->StorageType{
            self.data.clone()
        }
    }

    // Second General Impl
    impl <DataType, IntType, StorageType, IndexType> Matrix<DataType, IntType, StorageType, IndexType>
    where   Self: TypeToIndex<IndexType>,
            DataType: Copy + std::cmp::PartialEq,
            IndexType: Copy + std::hash::Hash + TryFrom<(IntType,IntType)> + std::cmp::Eq,
            IntType: Copy + 
                std::cmp::PartialOrd<IntType> + 
                std::ops::Add<IntType, Output = IntType> + 
                std::ops::Mul<IntType, Output = IntType> +
                std::fmt::Debug +
                TryFrom<usize>,

            usize: TryFrom<IntType>,

            <IntType as TryFrom<usize>>::Error : std::fmt::Debug,

            <usize as TryFrom<IntType>>::Error: std::fmt::Debug,
            StorageType: Clone
    {
        pub fn is_pos_valid(&self, pos: IndexType)->bool{
                self.to_idx(pos).is_some()
        }


    }
    // Implementing "TypeToindex" to allow for conversions between our "position" type and indices (when needed)
    impl <DataType, IntType, StorageType, IndexType> TypeToIndex<Position<IntType>> for Matrix<DataType, IntType, StorageType, IndexType>
    where   IntType: Copy + 
                    std::hash::Hash +
                    std::cmp::Eq +
                    std::cmp::PartialOrd<IntType> + 
                    std::ops::Add<IntType, Output = IntType> + 
                    std::ops::Mul<IntType, Output = IntType> +
                    std::fmt::Debug +
                    TryFrom<usize>,

            usize: TryFrom<IntType>,

            <IntType as TryFrom<usize>>::Error : std::fmt::Debug,

            <usize as TryFrom<IntType>>::Error: std::fmt::Debug,
    {
        fn to_idx(&self, input: Position<IntType>)->Option<usize> {
            let width = self.width().try_into().ok()?;
            let height = self.height().try_into().ok()?;
            if input.x() >= width || input.y() >= height{
                return None;
            }
            let width_as_new_type: IntType = self.width.try_into().unwrap();
            let idx = (input.y() * width_as_new_type) + input.x();
            let idx_as_usize: usize = idx.try_into().unwrap();
            Some( idx_as_usize )
        }

        fn from_idx(&self, idx: usize)->Option<Position<IntType>> {
            if idx < self.width()*self.height(){
                let y = idx / self.width;
                let x = idx % self.width;
                Some(Position::<IntType>::try_from((x.try_into().unwrap(), y.try_into().unwrap())).ok().unwrap())
            }else{
                None
            }
        }
    }
    // Matrix with underlying HashMap as storage device.
    impl <DataType, IntType, IndexType> Matrix<DataType,IntType, HashMap<IndexType,DataType>,IndexType>
        where   DataType: Copy + std::cmp::PartialEq,
                IndexType: Copy + std::hash::Hash + TryFrom<(IntType,IntType)> + std::cmp::Eq,
                IntType: Copy + 
                    std::hash::Hash +
                    std::cmp::Eq +
                    std::cmp::PartialOrd<IntType> + 
                    std::ops::Add<IntType, Output = IntType> + 
                    std::ops::Mul<IntType, Output = IntType> +
                    std::fmt::Debug +
                    TryFrom<usize>,

                usize: TryFrom<IntType>,

                <IntType as TryFrom<usize>>::Error : std::fmt::Debug,

                <usize as TryFrom<IntType>>::Error: std::fmt::Debug,
        {
        pub fn new_from_square(data:Vec<Vec<DataType>>, filter_function: fn(&DataType)->bool)->Self{
            let height = data.len();
            let width = data[0].len();
            let data = data.into_iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter(|(_,d)| filter_function(&d))
                        .filter_map(move |(x,d)| {
                            let x1: IntType = x.try_into().unwrap();
                            let y1: IntType = y.try_into().unwrap();
                            let idx_type = IndexType::try_from((x1,y1)).ok();
                            if let Some(idx) = idx_type{
                                Some((idx,d))
                            }else{
                                None
                            }
                        })
                }).collect();
            Self::new_from_flat(width, height, data)
        }

        pub fn new_from_flat(width: usize, height: usize, data:HashMap<IndexType,DataType>)->Self{
            Self{index_type: PhantomData::<IndexType>, data_type: PhantomData::<DataType>, int_type: PhantomData::<IntType>, width, height, data}
        }

        pub fn find(&self, target: &DataType)->Option<IndexType>{
            for (key,val) in self.data.iter(){
                if val == target{
                    return Some(*key);
                }
            }
            None
        }

    }
    // Implementing a generic Get/Set for the underlying data structure for the HashMap based matrix. (Needed for Display)
    impl <DataType, IntType, IndexType> GetSet< IndexType, DataType> for Matrix<DataType,IntType, HashMap<IndexType,DataType>, IndexType>
    where   Self: TypeToIndex<IndexType>,
            DataType: Copy + std::cmp::PartialEq,
            IndexType: Copy + std::hash::Hash + TryFrom<(IntType,IntType)> + std::cmp::Eq,
            IntType: Copy + 
                std::hash::Hash +
                std::cmp::Eq +
                std::cmp::PartialOrd<IntType> + 
                std::ops::Add<IntType, Output = IntType> + 
                std::ops::Mul<IntType, Output = IntType> +
                std::fmt::Debug +
                TryFrom<usize>,

            usize: TryFrom<IntType>,

            <IntType as TryFrom<usize>>::Error : std::fmt::Debug,

            <usize as TryFrom<IntType>>::Error: std::fmt::Debug,
    {
        fn get(&self,key: IndexType)->Option<DataType> {
            if let Some(value) = self.data.get(&key){
                Some(*value)
            }else{
                None
            }
        }
    
        fn set(&mut self,key: IndexType, value: DataType) {
            if let Some(_) = self.to_idx(key){
                self.data.insert(key, value);
            }
        }
    }
    
    // Matrix with underlying Vec as storage device.
    impl <DataType, IntType, IndexType> Matrix<DataType,IntType, Vec<DataType>, IndexType>
        where
            Self: TypeToIndex<IndexType>,   
            DataType: Copy + std::cmp::PartialEq,
                IntType: Copy + 
                    std::cmp::PartialOrd<IntType> + 
                    std::ops::Add<IntType, Output = IntType> + 
                    std::ops::Mul<IntType, Output = IntType> +
                    std::fmt::Debug +
                    TryFrom<usize>,

                usize: TryFrom<IntType>,

                <IntType as TryFrom<usize>>::Error : std::fmt::Debug,

                <usize as TryFrom<IntType>>::Error: std::fmt::Debug,
        {
        pub fn new_from_flat(width: usize, height: usize, data:Vec<DataType>)->Self{
            Self{index_type: PhantomData::<IndexType>, data_type: PhantomData::<DataType>, int_type: PhantomData::<IntType>, width, height, data }
        }
        pub fn new_from_square(data:Vec<Vec<DataType>>)->Self{
            let height = data.len();
            let width = data[0].len();
            let data = data.into_iter().flat_map(|row| row).collect();
            Self::new_from_flat(width, height, data)
        }

        pub fn find(&self, target: &DataType)->Option<IndexType>{
            for (idx, val) in self.data.iter().enumerate(){
                if val == target{
                    return self.from_idx(idx);
                }
            }
            None
        }
    }
    // Implementing a generic Get/Set for the underlying data structure for the HashMap based matrix. (Needed for Display)
    impl <DataType, IntType,IndexType> GetSet<IndexType,DataType> for Matrix<DataType,IntType, Vec<DataType>, IndexType>
    where   Self: TypeToIndex<IndexType>,
            IndexType: Copy + std::hash::Hash + TryFrom<(IntType,IntType)> + std::cmp::Eq,
            DataType: Copy + std::cmp::PartialEq,
            IntType: Copy + 
            std::cmp::PartialOrd<IntType> + 
            std::ops::Add<IntType, Output = IntType> + 
            std::ops::Mul<IntType, Output = IntType> +
            std::fmt::Debug +
            TryFrom<usize>,

        usize: TryFrom<IntType>,

        <IntType as TryFrom<usize>>::Error : std::fmt::Debug,

        <usize as TryFrom<IntType>>::Error: std::fmt::Debug
    {
        fn get(&self,key: IndexType)->Option<DataType> {
            let idx = self.to_idx(key)?;
            if let Some(value) = self.data.get(idx){
                Some(*value)
            }else{
                None
            }
        }
    
        fn set(&mut self,key: IndexType, value: DataType) {
            if let Some(idx) = self.to_idx(key){
                self.data[idx]= value;
            }
        }
    }
    // Semi-generic display implementation.
    impl <DataType, IntType, StorageType, IndexType>Display for Matrix<DataType,IntType, StorageType, IndexType> 
    where 
        IndexType: Copy + std::hash::Hash + TryFrom<(IntType,IntType)> + std::cmp::Eq,
        Self: GetSet<Position<IntType>,DataType>,
        DataType: Display,
        IntType: Display + TryFrom<usize>,
        usize: TryFrom<IntType>,
        <IntType as TryFrom<usize>>::Error : std::fmt::Debug,
        <usize as TryFrom<IntType>>::Error: std::fmt::Debug,
     {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut v = vec![];
            for y in 0..self.height {
                for x in 0..self.width {
                    let p = Position::new(x.try_into().unwrap(), y.try_into().unwrap());
                    let s = match self.get(p){
                        Some(data) => format!("{data}"),
                        None => format!(" ")
                    };
                    v.push(s);
                }
                v.push("\n".to_string());
            }
            let output = v.join("");
            write!(f,"{output}")
        }
         }

}


pub mod astar{
    use std::collections::HashMap;

    use crate::matrix::GetSet;
    pub trait AStartTraversible<PositionType, GoalType, ScoreType, StoredDataType, ValueType>
    where PositionType: Copy + std::hash::Hash + PartialEq<GoalType>,
            ScoreType: Copy,
            ValueType: Copy,
            StoredDataType: GetSet<PositionType,ValueType>,
        {
        fn a_start(&self, start: PositionType, goal: GoalType)->Option<Vec<GoalType>>;
        fn heuristic(&self, pos: PositionType, goal: GoalType)->ScoreType;
        fn reconstruct_path(&self, came_from: HashMap<PositionType,PositionType>, current: PositionType)->Vec<PositionType>;
    }
}
