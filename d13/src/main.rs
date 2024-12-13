use std::fs::read_to_string;

use position::Position;
use vector::Vector;
type IntType = i128;
mod position{
    use std::{fmt::Display, ops::{Add, Sub}};

    use crate::IntType;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Position{
        pub x: IntType,
        pub y: IntType
    }
    impl Position{
        pub fn new(x:IntType,y:IntType)->Self{
            Self{x,y}
        }
        fn adjusted(&self, dy:i32,dx:i32)->Option<Self>{
            match (self.y != 0 || dy != -1) && (self.x != 0 || dx != -1){
                true => {
                    let new_x = self.x as i32 + dx;
                    let new_y = self.y as i32 + dy;
                    debug_assert!(new_x >= 0);
                    debug_assert!(new_y >= 0);
                    Some(Self::new((self.x as i32 + dx) as IntType, (self.y as i32 +dy) as IntType))},
                false => None,
            }
        }
        pub fn neighbours(&self)->[Option<Position>;4]{
            [[-1,0],[1,0],[0,-1],[0,1]].into_iter().map(|[dy,dx]| self.adjusted(dy, dx)).collect::<Vec<Option<Position>>>().try_into().unwrap()
        }
        pub fn x(&self)->IntType{
            self.x
        }
        pub fn y(&self)->IntType{
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
    use std::ops::Add;

    use crate::{position::Position, IntType};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Vector{
        step: Position
    }
    impl Vector{
        pub fn new(step: Position)->Self{
            Self{step}
        }
        pub fn x(&self)->IntType{
            self.step.x()
        }
        pub fn y(&self)->IntType{
            self.step.y()
        }
        pub fn step(&self)->Position{
            self.step
        }
        pub fn multiply_by(&self, v: IntType)->Self{
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    let x = x_side.strip_prefix("X+").unwrap().parse::<IntType>().unwrap();
    let y = y_side.strip_prefix("Y+").unwrap().parse::<IntType>().unwrap();
    Vector::new(Position::new(x, y))
}
fn line_to_pos(line:&str)->Position{
    let mid = "Prize: ".len();
    let (_,line) = line.split_at(mid);
    let (x_side, y_side) = line.split_once(", ").unwrap();
    let x = x_side.strip_prefix("X=").unwrap().parse::<IntType>().unwrap();
    let y = y_side.strip_prefix("Y=").unwrap().parse::<IntType>().unwrap();
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LimitingFactor{
    X,
    Y,
    Both,
}
impl LimitingFactor{
    fn new(max_x: IntType, max_y: IntType)->Self{
        if max_x == max_y {
            Self::Both
        }else if max_x > max_y{
            Self::Y
        }else{
            Self::X
        }
    }
}
fn how_many_vectors_fit_before_target(v: &Vector, target: &Position)->IntType{
    let max_x = target.x()/v.x();
    let max_y = target.y()/v.y();

    max_x.min(max_y)
}
fn math_solve((focus,focus_cost): (&Vector, IntType), (other, other_cost): (&Vector, IntType), target: &Position)->Option<IntType>{
    // How many steps can we take using the focus-vector before being out of bounds in any direction.
    let mut min = IntType::MAX;
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
    if min == IntType::MAX{
        None
    }else{
        Some(min)
    }
}
fn intersection_between_two_lines(left: Vector, right: Vector)->Option<Position>{
    let a = left.x() as IntType;
    let b = left.y() as IntType;
    let c = right.x() as IntType;
    let d = right.y() as IntType;
    let x = (d-c)/(a-b);
    let y = a*x + c;
    if x < 0 || y < 0{
        None
    }else{
        Some(Position::new(x, y))

    }
}
fn extended_gcd(a:IntType,b:IntType)->(IntType,IntType,IntType){
    if a == 0{
        return (b,0,1);
    }
    let (gcd, x1, y1) = extended_gcd(b%a, a);
    let x = y1 - ((b/a) * x1);
    let y = x1;
    debug_assert!( gcd >= 0);
    return (gcd, x, y);
}
fn one_dimensional_find(a:IntType,b:IntType,target:IntType)->Option<(IntType,IntType)>{
    let (gcd, x, y) = extended_gcd(a, b);
    if target % gcd != 0{
        return None;
    }
    let x = x*target as IntType/gcd;
    let y = y/target as IntType/gcd;
    return Some((x as IntType,y as IntType))
}
fn ratio(a:IntType,b:IntType)->(IntType,IntType){
    let (gcd, _, _) = extended_gcd(a, b);
    return (a/gcd, b/gcd);
}
pub fn distance(left:Position, right:Position)->usize{
    ((left.x()-right.x()).abs() + (left.y()-right.y()). abs()) as usize
}
fn y_distance(left:Position, right:Position)->IntType{
    (left.y() - right.y()).abs()
}
fn math_solve2((focus,focus_cost): (&Vector, IntType), (other, other_cost): (&Vector, IntType), target: &Position)->Option<IntType>{
    let (mut focus_count, mut other_count) = one_dimensional_find(focus.x(), other.x(), target.x())?;
    // If iX + jY == N, (i-ration.0)X + (j+ratio.1)Y == N as well.
    let (other_multiplier, focus_multiplier) = ratio(focus.x(), other.x());
    loop{
        let current_position = (focus.multiply_by(focus_count)+other.multiply_by(other_count)).step();
        // if current_position.x() < 0 || current_position.y() < 0 {
        //     break None;
        // }
        println!("{current_position}");
        if y_distance(*target,current_position) == 0{
            return Some(focus_count*focus_cost + other_count+other_cost)
        }
        let mut improved = false;
        let mut next_position = (focus.multiply_by(focus_count+focus_multiplier)+other.multiply_by(other_count-other_multiplier)).step();
        while (other_count - other_multiplier >= 0) && y_distance(*target,next_position) < y_distance(*target,current_position){
            // if next_position.y() < 0{
            //     break;
            // }
            focus_count += focus_multiplier;
            other_count -= other_multiplier;
            // println!("To Focus");
            next_position = (focus.multiply_by(focus_count+focus_multiplier)+other.multiply_by(other_count-other_multiplier)).step();
            improved = true;
        }
        let mut next_position = (focus.multiply_by(focus_count-focus_multiplier)+other.multiply_by(other_count+other_multiplier)).step();
        while (focus_count - focus_multiplier >= 0) && y_distance(*target,next_position) < y_distance(*target,current_position){
            // if next_position.y() < 0{
            //     break;
            // }
            focus_count -= focus_multiplier;
            other_count += other_multiplier;
            // println!("To Other");
            next_position = (focus.multiply_by(focus_count-focus_multiplier)+other.multiply_by(other_count+other_multiplier)).step();
            debug_assert_eq!(next_position.x(), current_position.x());
            improved = true;
        }
        if !improved{ // If we can not make any more improvements, and we have not found the target yet, it is impossible.
            return None;
        }
    }
}
fn math_solve3((focus, focus_cost): (&Vector, IntType), (other, other_cost): (&Vector, IntType), target: &Position)->Option<IntType>{
    let prize = target;
    let b = other.step();
    let a = focus.step();
    let na = ((prize.x * b.y) - (prize.y * b.x)) / ((a.x * b.y) - (a.y * b.x));
    let nb = (prize.x - na * a.x) / b.x;
    let solution = Position::new(na * a.x + nb * b.x, na * a.y + nb * b.y);
    if &solution == prize{
        return Some(na*focus_cost + nb*other_cost);
    }
    None
}
const A_COST: IntType = 3;
const B_COST: IntType = 1;
fn solve1(data:&Vec<DataPoint>, solver: fn((&Vector, IntType),(&Vector, IntType),&Position)->Option<IntType>)->IntType{
    let mut total = 0;
    for case in data.iter().rev(){
        // println!("{data:?}");
        let a = &case.a;
        let b = &case.b;
        let target = case.target;
        let a_val = solver((a,A_COST),(b,B_COST),&target);
        let b_val = solver((b,B_COST),(a,A_COST),&target);
        match (a_val,b_val){
            (Some(a_cost),Some(b_cost))=> total += a_cost.min(b_cost),
            (Some(cost),None) | (None, Some(cost)) => total += cost,
            (None,None) => ()
        }
        
    }
    total
}

const ADDON: IntType = 10000000000000;
fn solve2(data:Vec<DataPoint>)->IntType{
    let first_target = data[0].target;
    let new_data: Vec<DataPoint> = data.into_iter().map(|dp| {
        let new_target = dp.target + Position::new(ADDON,ADDON);
        DataPoint::new(dp.a,dp.b,new_target)
    }).collect();
    assert_eq!(&new_data[0].target.x(), &(first_target.x() + ADDON));
    solve1(&new_data, math_solve3)
}
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data,math_solve);
    let s1_end = std::time::Instant::now();
    assert_eq!(solution1, 37128);
    let solution2 = solve2(data);
    assert_eq!(solution2,74914228471331);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Parse file time: {:?}", file_end - start);
    println!("P1 time: {:?}",s1_end-file_end);
    println!("P2 time: {:?}", s2_end - s1_end);
    println!("Total time: {:?}", s2_end - start);
}
#[cfg(test)]
mod tests{
    use std::fs::read_to_string;

    use crate::{get_data, math_solve, math_solve3, ratio, solve1};

    #[test]
    fn solve_test1_1(){
        let expected = 480;
        let file_name = "TestData1.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data,math_solve);
        assert_eq!(solution,expected)
    }
    #[test]
    fn solve_test1_2(){
        let expected = 280;
        let file_name = "TestData2.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data,math_solve);
        assert_eq!(solution,expected)
    }
    #[test]
    fn new_solver_equals_old_solver(){
        let file_name = "TestData2.txt";
        let expected = 280;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solver1_solution = solve1(&data,math_solve);
        assert_eq!(solver1_solution,expected);
        let solver2_solution = solve1(&data,math_solve3);
        assert_eq!(solver2_solution,solver1_solution)
    }
    #[test]
    fn does_substitution_stay_on_same_point(){
        let x = 10;
        let y = 50;
        let ratio = ratio(x, y);
        println!("ratio: {ratio:?}");
        let dx = x*ratio.1;
        let dy = y*ratio.0;
        println!("{dx} != {dy}");
        assert_eq!(dx,dy, "{dx} != {dy}");
    }
}
