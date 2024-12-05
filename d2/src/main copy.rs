use std::fs::read_to_string;

#[derive(Clone, Copy,PartialEq, Eq)]
enum IncDec{
    Inc,
    Dec,
    Invalid
}
impl IncDec{
    fn new(a:i32,b:i32)->Self{
        match b-a{
            -3..=-1 => Self::Inc,
            1..=3 => Self::Dec,
            _ => Self::Invalid
        }
    }
}
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
    fn flexible_new<Q: Iterator<Item=i32> + Clone>(nums: Q)->Self{
        let intermediary_nums = nums.collect::<Vec<i32>>();
        if ReportQuality::new(Report::new(intermediary_nums.clone().into_iter())).is_good(){
            return Self::Good;
        }
        for idx_to_ignore in 0..intermediary_nums.len(){
            let candidate = intermediary_nums
                .clone()
                .iter()
                .enumerate()
                .filter(|(idx,_)| *idx !=idx_to_ignore )
                .map(|(_,val)| *val)
                .collect::<Vec<i32>>();
            let report = Report::new(candidate.into_iter());
            if ReportQuality::new(report).is_good(){
                return Self::MostlyGood;
            }
        }   
        return Self::Bad;
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
            let nums = line
                .split(" ")
                .map(|n|n.parse::<i32>().unwrap());
            let report = Report::new(nums);
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
    assert_eq!(n_safe_lines1,472); // Known answer
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
    #[test]
    fn solves_test_case1(){
        let expected = 2;
        let file_name = "TestData.txt";
        let answer = solve1(file_name);
        assert_eq!(expected,answer)
    }
    #[test]
    fn solves_test_case2(){
        let expected = 4;
        let file_name = "TestData.txt";
        let answer = solve2(file_name);
        assert_eq!(expected,answer)
    }
}
