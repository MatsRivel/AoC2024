use regex::{self, Regex};
pub const MATCHING_REGEX: &str = r"mul\((\d*),(\d*)\)";
fn solve1(s: &str)->i32{
    let regex = Regex::new(MATCHING_REGEX).unwrap();
    regex.captures_iter(&s).map(|capture| {
        let (_full_string,[left,right]) = capture.extract();
        let l_num = left.parse::<i32>().unwrap();
        let r_num = right.parse::<i32>().unwrap();
        // print!("{l_num}*{r_num} + ");
        l_num*r_num

    }).sum::<i32>()
}
const FILTER_REGEX: &str = r"do\(\)(.*?)don't\(\)";
fn solve2(s: &str)->i32{
    let new_s = format!("do(){s}don't()").lines().map(|s| s.to_string()).collect::<String>();
    let regex = Regex::new(FILTER_REGEX).unwrap();
    regex.captures_iter(&new_s).map(|capture| {
        let (_full_string,[inner_s]) = capture.extract();
        solve1(inner_s)
    }).sum::<i32>()
}
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = std::fs::read_to_string(file_name).unwrap();
    let solution1 = solve1(&s);
    assert_eq!(solution1,182619815);
    let s = std::fs::read_to_string(file_name).unwrap();
    let solution2 = solve2(&s);
    assert_eq!(solution2,80747545);
    let end = std::time::Instant::now();
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Total time: {:?}",end-start);
    
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn first_part(){
        let file_name = "TestData.txt";
        let expected = 161;
        let s = std::fs::read_to_string(file_name).unwrap();
        let solution1 = solve1(&s);
        assert_eq!(expected,solution1)
    }
    #[test]
    fn second_part(){
        let file_name = "TestData2.txt";
        let expected = 48;
        let s = std::fs::read_to_string(file_name).unwrap();
        let solution2 = solve2(&s);
        assert_eq!(expected,solution2)
    }
}