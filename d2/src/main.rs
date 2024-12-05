use std::{fs::read_to_string, num::ParseIntError};

fn get_numbers_from_file(file_name: &str)->Vec<Vec<i32>>{
    read_to_string(file_name)
    .unwrap()
    .lines()
    .map(|line|{
        get_numbers_from_line(line)
    }).collect::<Vec<Vec<i32>>>()
    

}
fn get_numbers_from_line(line: &str)->Vec<i32>{
        line
            .split(" ")
            .map(|n|n.parse::<i32>().unwrap()).collect()
}

#[derive(Clone,Copy,PartialEq, Eq,Debug)]
enum Validity{
    Valid,
    Invalid
}
impl Validity{
    fn new(a:i32,b:i32)->Self{
        match b-a{
            1..=3 | -3..=-1 => Self::Valid,
            _ => Self::Invalid
        }
    }
    fn is_valid(&self)->bool{
        match self{
            Self::Valid => true,
            Self::Invalid => false,
        }
    }
    fn is_invalid(&self)->bool{
        match self{
            Self::Valid => false,
            Self::Invalid => true,
        }
    }
}

struct CellPoint{
    idx: usize,
    left: Validity,
    right: Validity,
    top: Validity
}impl CellPoint{
    fn new(idx: usize, left: Option<Validity>, right: Option<Validity>, top:Option<Validity>)->Self{
        let left = left.or(Some(Validity::Valid)).unwrap();
        let right = right.or(Some(Validity::Valid)).unwrap();
        let top = top.or(Some(Validity::Valid)).unwrap();
        Self{idx,left,right,top}
    }
    fn is_valid(&self)->bool{
        match (self.left, self.right,self.top){
            (Validity::Valid, Validity::Valid, Validity::Valid) => true,
            (Validity::Valid, Validity::Valid, Validity::Invalid) => true,
            (Validity::Valid, Validity::Invalid, Validity::Valid) => true,
            (Validity::Valid, Validity::Invalid, Validity::Invalid) => true,
            (Validity::Invalid, Validity::Valid, Validity::Valid) => true,
            (Validity::Invalid, Validity::Valid, Validity::Invalid) => false,
            (Validity::Invalid, Validity::Invalid, Validity::Valid) => true,
            (Validity::Invalid, Validity::Invalid, Validity::Invalid) => false,
        }
    }
}
fn is_line_valid(line: &Vec<i32>)->bool{
    println!("____________________");
    let first_order = line.iter().zip(line.iter().skip(1)).map(|(&a,&b)| Validity::new(a,b)).collect::<Vec<Validity>>();
    let second_order = line.iter().zip(line.iter().skip(2)).map(|(&a,&b)| Validity::new(a,b)).collect::<Vec<Validity>>();
    for i in 0..first_order.len(){
        let fmin = i.max(1);
        let fmax = (i+2).min(first_order.len()-1);
        let smin = i.max(1);
        let smax = (i+1).min(second_order.len()-1);
        let f_slice = &first_order[fmin..=fmax];
        let s_slice  = &second_order[smin..=smax];
        println!("\n{f_slice:?}\n{s_slice:?}\n");
        let f_invalid = f_slice.iter().map(|f| *f).filter(|f| f.is_invalid()).collect::<Vec<Validity>>();
        let s_invalid = s_slice.iter().map(|s| *s).filter(|s| s.is_invalid()).collect::<Vec<Validity>>();
        if f_invalid.len() < 3 || s_invalid.len() < 2{
            continue;
        }
        use Validity::*;
        match (f_invalid[0],f_invalid[1],f_invalid[2], s_invalid[0],s_invalid[2]){
            (_, _, _, Invalid, Invalid)                     => return false,
            (Valid, Valid, Valid, _, _)                     => continue,
            (Invalid, Invalid, Valid, Valid, Invalid)       => continue,
            (Invalid, Invalid, _, Invalid, _)               => return false,
            (Valid, Valid, Invalid, Valid, Invalid)         => continue,
            (Valid, Invalid, Valid, Valid, Valid)           => todo!(),
            (Valid, Invalid, Valid, Valid, Invalid)         => todo!(),
            (Valid, Invalid, Valid, Invalid, Valid)         => todo!(),
            (Valid, Invalid, Invalid, Valid, Valid)         => todo!(),
            (Valid, Invalid, Invalid, Valid, Invalid)       => todo!(),
            (Valid, Invalid, Invalid, Invalid, Valid)       => todo!(),
            (Invalid, Valid, Valid, Valid, Valid)           => todo!(),
            (Invalid, Valid, Valid, Valid, Invalid)         => todo!(),
            (Invalid, Valid, Valid, Invalid, Valid)         => todo!(),
            (Invalid, Valid, Invalid, Valid, Valid)         => todo!(),
            (Invalid, Valid, Invalid, Valid, Invalid)       => todo!(),
            (Invalid, Valid, Invalid, Invalid, Valid)       => todo!(),
            (Invalid, Invalid, Valid, Valid, Valid)         => todo!(),
            (Invalid, Invalid, Invalid, Valid, Valid)       => todo!(),
            (Invalid, Invalid, Invalid, Valid, Invalid)     => todo!(),
            (Valid, Valid, Invalid, Valid, Invalid)         => todo!(),
        }
    }
    true
}


fn main() {
    let file_name = "Data.txt";
    // let start1 = std::time::Instant::now();
    // let n_safe_lines1 = solve1(file_name);
    // let end1 = std::time::Instant::now();
    // assert_eq!(n_safe_lines1, 472); // Known answer
    let start2 = std::time::Instant::now();
    // let n_safe_lines2 = solve2(file_name);
    let end2 = std::time::Instant::now();
    // assert_eq!(n_safe_lines2, 520); // Known answer
    // println!("Part 1: {n_safe_lines1} in {:?}", end1-start1);
    // println!("Part 2: {n_safe_lines2} in {:?}", end2-start2);
    // println!("Total time: {:?}", end2-start1);
}

#[cfg(test)]
mod tests{
    use super::*;
    mod case_2{
        use super::*;

        #[test]
        fn mostly_valid_reports(){
            let reports = [
                "-5 2 3 4 5",
                "0 2 3 4 5",
                "1 2 3 4 5",
                "2 2 3 4 5",
                "3 2 3 4 5",
                "9 2 3 4 5",

                "1 -2 3 4 5",
                "1 0 3 4 5",
                "1 2 3 4 5",
                "1 3 3 4 5",
                "1 4 3 4 5",
                "1 10 3 4 5",

                "1 2 -1 4 5",
                "1 2 0 4 5",
                "1 2 3 4 5",
                "1 2 4 4 5",
                "1 2 5 4 5",
                "1 2 10 4 5",

                "1 2 3 1 5",
                "1 2 3 2 5",
                "1 2 3 3 5",
                "1 2 3 4 5",
                "1 2 3 5 5",
                "1 2 3 10 5",

                "1 2 3 4 0",
                "1 2 3 4 3",
                "1 2 3 4 4",
                "1 2 3 4 5",
                "1 2 3 4 6",
                "1 2 3 4 10",
            ];
            for s in reports{
                let nums = get_numbers_from_line(s);
                let line_validity = is_line_valid(&nums);
                assert!(line_validity, "Line: {nums:?} should be valid, but is invalid.");
            }
        }
        #[test]
        fn invalid_reports(){
            let reports = [
                "1 2 2 2 5",
                "1 2 6 7 9",
                "3 2 1 2 3",
            ];
            for s in reports{
                let nums = get_numbers_from_line(s);
                let line_validity = is_line_valid(&nums);
                assert!( !line_validity, "Line: {nums:?} should be invalid, but is valid.");
            }
        }
    
        #[test]
        fn solves_test_case2(){
            let expected = 4;
            let file_name = "TestData.txt";
            // let answer = solve2(file_name);
            // assert_eq!(expected,answer)
        }

    }
}
