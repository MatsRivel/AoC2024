use std::{fs::read_to_string, usize};

use position::Position;
use vector::Vector;

mod position{
    use std::{fmt::Display, ops::{Add, Sub}};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Position{
        x: usize,
        y: usize
    }
    impl Position{
        pub fn new(x:usize,y:usize)->Self{
            Self{x,y}
        }
        fn adjusted(&self, dy:i32,dx:i32)->Option<Self>{
            match (self.y != 0 || dy != -1) && (self.x != 0 || dx != -1){
                true => {
                    let new_x = self.x as i32 + dx;
                    let new_y = self.y as i32 + dy;
                    debug_assert!(new_x >= 0);
                    debug_assert!(new_y >= 0);
                    Some(Self::new((self.x as i32 + dx) as usize, (self.y as i32 +dy) as usize))},
                false => None,
            }
        }
        pub fn neighbours(&self)->[Option<Position>;4]{
            [[-1,0],[1,0],[0,-1],[0,1]].into_iter().map(|[dy,dx]| self.adjusted(dy, dx)).collect::<Vec<Option<Position>>>().try_into().unwrap()
        }
        pub fn x(&self)->usize{
            self.x
        }
        pub fn y(&self)->usize{
            self.y
        }
    }
    impl Add for Position{
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x+rhs.x, self.y+rhs.y)
        }
    }
    impl Sub for Position{
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.x-rhs.x, self.y-rhs.y)
        }
    }
    impl Display for Position{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"(x: {}, y: {})", self.x(),self.y())
        }
    }
}

mod vector{
    use std::ops::{Add, Mul};

    use crate::position::Position;

    #[derive(Debug, Clone, Copy)]
    pub struct Vector{
        step: Position
    }
    impl Vector{
        pub fn new(step: Position)->Self{
            Self{step}
        }
        pub fn x(&self)->usize{
            self.step.x()
        }
        pub fn y(&self)->usize{
            self.step.y()
        }
        pub fn step(&self)->Position{
            self.step
        }
        pub fn multiply_by(&self, v: usize)->Self{
            let y = self.y()*v;
            let x = self.x()*v;
            Self::new(Position::new(x, y))
        }
        
    }
    impl Add for Vector{
        type Output= Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.step() + rhs.step())
        }
    }

}
#[derive(Debug)]
struct DataPoint{
    pub a: Vector,
    pub b: Vector,
    pub target: Position
}
impl DataPoint{
    fn new(a: Vector,b: Vector,target: Position)->Self{
        Self { a, b, target }
    }
}
fn line_to_vec(line:&str)->Vector{
    let mid = "Button A: ".len();
    let (_,line) = line.split_at(mid);
    let (x_side, y_side) = line.split_once(", ").unwrap();
    let x = x_side.strip_prefix("X+").unwrap().parse::<usize>().unwrap();
    let y = y_side.strip_prefix("Y+").unwrap().parse::<usize>().unwrap();
    Vector::new(Position::new(x, y))
}
fn line_to_pos(line:&str)->Position{
    let mid = "Prize: ".len();
    let (_,line) = line.split_at(mid);
    let (x_side, y_side) = line.split_once(", ").unwrap();
    let x = x_side.strip_prefix("X=").unwrap().parse::<usize>().unwrap();
    let y = y_side.strip_prefix("Y=").unwrap().parse::<usize>().unwrap();
    Position::new(x, y)
}
fn get_data(s:&str)->Vec<DataPoint>{
    let mut data = Vec::new();
    let mut lines = s.lines();
    while let Some(a_str) = lines.next(){
        let b_str = lines.next().unwrap();
        let prize_str = lines.next().unwrap();
        let _ = lines.next();
        let a = line_to_vec(a_str);
        let b = line_to_vec(b_str);
        let target = line_to_pos(prize_str);
        let datapoint = DataPoint::new(a,b,target);
        data.push(datapoint);
    }
    data
    
}

fn how_many_vectors_fit_before_target(v: &Vector, target: &Position)->usize{
    let max_x = target.x()/v.x();
    let max_y = target.y()/v.y();
    max_x.min(max_y)
}
fn math_solve((focus,focus_cost): (&Vector, usize), (other, other_cost): (&Vector, usize), target: &Position)->Option<usize>{
    // How many steps can we take using the focus-vector before being out of bounds in any direction.
    let mut min = usize::MAX;
    for focus_count in 0..how_many_vectors_fit_before_target(focus, target){
        let start = focus.multiply_by(focus_count);
        let new_target = *target-start.step();
        let other_count = how_many_vectors_fit_before_target(other, &new_target);
        let final_position = (start + other.multiply_by(other_count)).step();
        let price = (focus_count * focus_cost) + (other_count*other_cost);
        if final_position == *target && min > price{
            min = price
        }
    }
    if min == usize::MAX{
        None
    }else{
        Some(min)
    }
}
fn intersection_between_two_lines(left: Vector, right: Vector)->Option<Position>{
    let a = left.x() as i64;
    let b = left.y() as i64;
    let c = right.x() as i64;
    let d = right.y() as i64;
    let x = (d-c)/(a-b);
    let y = a*x + c;
    if x < 0 || y < 0{
        None
    }else{
        Some(Position::new(x as usize, y as usize))

    }
}
const A_COST: usize = 3;
const B_COST: usize = 1;
fn solve1(data:&Vec<DataPoint>)->usize{
    let mut total = 0;
    for case in data{
        // println!("{data:?}");
        let a = &case.a;
        let b = &case.b;
        let target = case.target;
        // if let Some(intersection) = intersection_between_two_lines(*a, *b){
        //     println!("a: {}, b: {}",a.step(), b.step() );
        //     println!("Intersection: {intersection:?}, Target:{target:?}");
        //     let a_count = how_many_vectors_fit_before_target(a, &intersection);
        //     let b_count = how_many_vectors_fit_before_target(b, &(target-intersection));
        //     if (a.multiply_by(a_count) + b.multiply_by(b_count)).step() == target{
        //         total += a_count*A_COST + b_count*B_COST
        //     }
        // }
    }
    total
}
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data);
    println!("{solution1}");
    let s1_end = std::time::Instant::now();
    assert_eq!(solution1, 37128);
    // let solution2 = solve2(&data);
    // assert_eq!(solution2,284973560658514);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    // println!("Part2: {solution2}");
    println!("Parse file time: {:?}", file_end - start);
    println!("P1 time: {:?}",s1_end-file_end);
    println!("P2 time: {:?}", s2_end - s1_end);
    println!("Total time: {:?}", s2_end - start);
}
#[cfg(test)]
mod tests{
    use std::fs::read_to_string;

    use crate::{get_data, solve1};

    #[test]
    fn solve_test1_1(){
        let expected = 480;
        let file_name = "TestData1.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data);
        assert_eq!(solution,expected)
    }
    #[test]
    fn solve_test1_2(){
        let expected = 280;
        let file_name = "TestData2.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data);
        assert_eq!(solution,expected)

    }
}
