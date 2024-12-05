use std::{fs::read_to_string, num::ParseIntError};

#[derive(Clone, Copy,PartialEq, Eq, Debug)]
enum IncDec{
    Inc,
    Dec,
    Invalid
}
impl IncDec{
    fn new(a:i32,b:i32)->Self{
        match a-b{
            -3..=-1 => Self::Inc,
            1..=3 => Self::Dec,
            _ => Self::Invalid
        }
    }
    fn is_inc(&self)->bool{
        match self{
            Self::Inc => true,
            _ => false
        }
    }
    fn is_dec(&self)->bool{
        match self{
            Self::Dec => true,
            _ => false
        }
    }
    fn is_valid(&self)->bool{
        match  self {
            Self::Inc | Self::Dec => true,
            Self::Invalid => false,
        }
    }
}
#[derive(Debug)]
struct Report {
    points: Vec<IncDec>
}
impl Report {
    fn new<Q: Iterator<Item=i32> + Clone>(nums: Q)->Self{
        let points = nums
            .clone()
            .zip(nums.skip(1))
            .map(|(a,b)| IncDec::new(a,b) ).collect();
        Self { points }
        
    }
    fn double_new<Q: Iterator<Item=i32> + Clone>(nums: Q)->[Self;2]{
        let (points_a,points_b) = nums
            .clone()
            .zip(nums.clone().skip(1))
            .zip(nums.skip(2))
            .map(|((a,b),c,)| (IncDec::new(a,b),IncDec::new(a,c)) ).collect::<(Vec<IncDec>,Vec<IncDec>)>();
        [Self{points: points_a}, Self{points: points_b}]
    }
}
impl TryFrom<&str> for Report{
    type Error= ParseIntError;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
            let nums = line
                .split(" ")
                .map(|n|n.parse::<i32>()).collect::<Result<Vec<i32>,ParseIntError>>()?;
            let report = Report::new(nums.into_iter());
            Ok(report)
    }
}
enum ReportQuality{
    Good,
    MostlyGood,
    Bad
}

impl ReportQuality{
    fn new(report: Report)->Self{
        let rules_followed = report.points
            .clone()
            .into_iter()
            .zip(report.points.into_iter().skip(1))
            .all(|(a,b)| a == b && a != IncDec::Invalid );
    
        match rules_followed{
            true => Self::Good,
            false => Self::Bad,
        }
    }
    fn flexible_from_str(line:&str)->Self{
        let nums = line
            .split(" ")
            .map(|n|n.parse::<i32>().unwrap());
        Self::flexible_new(nums)

    }
    fn flexible_new<Q: Iterator<Item=i32> + Clone>(nums: Q)->Self{
        let mut has_cheated = false;
        let mut just_cheated = false;
        let [a,b] = Report::double_new(nums.clone());
        if a.points.iter().all(|p| p.is_dec()) || a.points.iter().all(|p| p.is_inc()){
            return Self::Good;
        }
        let is_mostly_good = a.points
            .clone()
            .iter()
            .zip(a.points.clone().iter().skip(1))
            .zip(b.points.iter())
            .map(|((a,b),c)|{
                println!("{a:?}, {b:?}, {c:?}");
                if just_cheated{
                    just_cheated = false;
                    *c
                }else if a == b && a.is_valid() && b.is_valid(){
                    *a
                }else if a == c && a.is_valid() && c.is_valid() && !has_cheated{
                    has_cheated = true;
                    just_cheated = true;
                    println!("Cheating");
                    *a
                }else if b == c && b.is_valid() && c.is_valid() && !has_cheated{
                    has_cheated = true;
                    just_cheated = true;
                    println!("Cheating");
                    *b
                }else{
                    IncDec::Invalid
                }
            }).enumerate().fold((true, IncDec::Invalid), |(all_the_same, inc_dec_ref),(idx, val)|{
                if idx == 0{
                    (true, val)
                }else if val == inc_dec_ref{
                    (all_the_same, val)
                }else{
                    (false,val)
                }
            }).0;

        if is_mostly_good{
            Self::MostlyGood
        }else{
            Self::Bad
        }

    }
    fn is_good(&self)->bool{
        match self{
            Self::Good => true,
            Self::Bad | Self::MostlyGood => false,
        }
    }
    fn is_not_bad(&self)->bool{
        match self{
            Self::Bad => false,
            Self::Good | Self::MostlyGood => true,
        }
    }
}


fn solve1(file_name: &str)->usize{
    read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line|{
            let report = line.try_into().unwrap();
            let report_quality = ReportQuality::new(report);
            report_quality

        }).filter(|report_quality| report_quality.is_good() )
        .count()
}

fn solve2(file_name: &str)->usize{
    read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line|{
            let nums = line
                .split(" ")
                .map(|n|n.parse::<i32>().unwrap());

            let report_quality = ReportQuality::flexible_new(nums);
            report_quality

        }).filter(|report_quality| report_quality.is_not_bad() )
        .count()
}

fn main() {
    let file_name = "Data.txt";
    let start1 = std::time::Instant::now();
    let n_safe_lines1 = solve1(file_name);
    let end1 = std::time::Instant::now();
    assert_eq!(n_safe_lines1, 472); // Known answer
    let start2 = std::time::Instant::now();
    let n_safe_lines2 = solve2(file_name);
    let end2 = std::time::Instant::now();
    assert_eq!(n_safe_lines2, 520); // Known answer
    println!("Part 1: {n_safe_lines1} in {:?}", end1-start1);
    println!("Part 2: {n_safe_lines2} in {:?}", end2-start2);
    println!("Total time: {:?}", end2-start1);
}

#[cfg(test)]
mod tests{
    use super::*;
    mod case_1{
        use super::*;

        #[test]
        fn solves_test_case1(){
            let expected = 2;
            let file_name = "TestData.txt";
            let answer = solve1(file_name);
            assert_eq!(expected,answer)
        }
        #[test]
        fn valid_reports(){
            let reports = [
                "7 6 4 2 1",
                "1 3 6 7 9",
                "1 2 3 4 5",
                "5 4 3 2 1"
            ];
            for s in reports{
                let report: Report = s.try_into().unwrap();
                let report_quality = ReportQuality::new(report);
                assert!(report_quality.is_good()) 
            }
        }
        #[test]
        fn invalid_reports(){
            let reports = [
                "3 2 1 4 5",
                "1 2 7 8 9",
                "0 0 0 0 0"
            ];
            for s in reports{
                let report: Report = s.try_into().unwrap();
                let report_quality = ReportQuality::new(report);
                assert!(!report_quality.is_good()) 
            }
        }
    }
    mod case_2{
        use super::*;

        #[test]
        fn mostly_valid_reports(){
            let reports = [
                "1 2 3 4 5",
                "1 3 2 4 5",
                "3 1 2 4 5",
                "1 2 3 4 0"
            ];
            for s in reports{
                let report_quality = ReportQuality::flexible_from_str(s);
                assert!(report_quality.is_not_bad(), "\nInput: {s}") 
            }
        }
        #[test]
        fn invalid_reports(){
            let reports = [
                "3 2 1 4 5"
            ];
            for s in reports{
                let report_quality = ReportQuality::flexible_from_str(s);
                assert!(!report_quality.is_not_bad(), "Input: {s}") 
            }
        }
    
        #[test]
        fn solves_test_case2(){
            let expected = 4;
            let file_name = "TestData.txt";
            let answer = solve2(file_name);
            assert_eq!(expected,answer)
        }

    }
}
