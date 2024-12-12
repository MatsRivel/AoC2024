use std::{collections::VecDeque, fmt::Display, fs::read_to_string};

type Solution = usize;
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord,Debug)]
struct Point{
    count: usize,
    id: usize,
}
impl Point{
    fn new(c:char, idx:usize)->Option<Self>{
        let count = c.to_digit(10)? as usize;
        Some(Self{ count,id: idx})
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum PointType{
    Space(Point),
    Real(Point)
}
impl PointType{
    fn is_space(&self)->bool{
        match self{
            Self::Space(_) => true,
            Self::Real(_) => false,
        }
    }
    fn count(&self)->usize{
        let point = match self{
            Self::Space(point) => point,
            Self::Real(point) => point,
        };
        point.count
    }
}
impl Display for PointType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            PointType::Space(point) => {
                let s = (0..point.count).map(|_| ".").collect::<String>();
                write!(f,"{s}")
            },
            PointType::Real(point) => {
                let s = (0..point.count).map(|_| format!("{}",point.id)).collect::<String>();
                write!(f,"{s}")
            },
        }
    }
}
fn get_data(s:&str)->Vec<PointType>{
    s.lines().flat_map(|line| line.chars().enumerate().filter_map(|(idx,c)|  {
        let is_space = idx%2 == 1;
        if let Some(block) = Point::new(c,idx/2){
            if is_space{
                Some(PointType::Space(block))
            }else{
                Some(PointType::Real(block))
            }
        }else{
            None
        }
    })).collect()

}

fn solve1(data:&Vec<PointType>)->Solution{
    let mut queue = VecDeque::new();
    for point in data.iter(){
        for _ in 0..point.count(){
            if let PointType::Real(b) = point{
                queue.push_back(Some(b.id));
            }else{
                queue.push_back(None);
            }
        }
    }
    let mut group = VecDeque::new();
    'outer: while let Some(element) = queue.pop_front(){
        if let Some(point) = element{
            #[cfg(debug_assertions)]
            print!("{point}");
            group.push_back(point);
        }else{
            'inner: loop{
                match queue.pop_back(){
                    None => break 'outer,
                    Some(None) => continue,
                    Some(Some(point)) =>  {
                        #[cfg(debug_assertions)]
                        print!("{point}");
                        group.push_back(point);
                        break 'inner;
                    }
                }
            }
        }
    }
    let output = group.iter().enumerate().map(|(idx,v)| idx*v).sum::<Solution>();
    output
}

fn solve2(data:&Vec<PointType>)->Solution{
    let mut left = VecDeque::<PointType>::new();
    let mut center = data.iter().map(|p|*p).collect::<VecDeque<PointType>>();    
    let mut right = VecDeque::<PointType>::new();
    
}

fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data);
    assert_eq!(solution1,6353658451014);
    let s1_end = std::time::Instant::now();
    let solution2 = solve2(&data);
    // assert_eq!(solution2,0);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    // println!("Part2: {solution2}");
    println!("Parse file time: {:?}",file_end-start);
    println!("P1 time: {:?}",s1_end-file_end);
    // println!("P2 time: {:?}",s2_end-s1_end);
    println!("Total time: {:?}",s2_end-start);
}   

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn solve_test1(){
        let file_name = "TestData1.txt";
        let expected = 1928;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve1(&data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test2(){
        let file_name = "TestData1.txt";
        let expected = 2858;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve2(&data);
        assert_eq!(solution1,expected)
    }

}