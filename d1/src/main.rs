use std::{collections::HashMap, fs::read_to_string};
fn read_file_to_sored_vec(file_name: &str)->(Vec<i32>,Vec<i32>){
    let file = read_to_string(file_name).unwrap();
    let unordered_vectors = file
        .split_terminator("\r\n")
        .map(|line| {
            let mut pair_iter = line.split("   ")
                .filter(|l| !l.is_empty())
                .map(|s| {
                    s.parse::<i32>().unwrap()
            });
            let a = pair_iter.next().unwrap_or(0);
            let b = pair_iter.next().unwrap_or(0);
            debug_assert!(a!=0 || b != 0,"If both a and b are zero we've mis-parsed something.");
            (a,b)
        }).collect::<(Vec<i32>,Vec<i32>)>();
    return unordered_vectors;
}
fn solve1(file_name: &str)->i32{
    let (mut left_vec, mut right_vec) = read_file_to_sored_vec(file_name);
    left_vec.sort();
    right_vec.sort();
    let solution = left_vec.iter().zip(right_vec.iter()).map(|(a,b)| {
        let diff = (a-b).abs();
        // println!("{a}   {b}   --> {}",diff);
        diff
        })
        .sum();
    solution
}
fn solve2(file_name: &str)->i32{
    let (mut left_vec, mut right_vec) = read_file_to_sored_vec(file_name);
    left_vec.sort();
    // right_vec.sort();
    let mut right_count = HashMap::<i32,i32>::new();
    right_vec.into_iter().for_each(|key|{
        if let Some(mut count) = right_count.get_mut(&key){
            *count += 1;
        }else{
            right_count.insert(key, 1);
        }
    });

    left_vec.into_iter().map(|key|{
        if let Some(&value) = right_count.get(&key){
            key * value
        }else{
            0
        }
    }).sum()


}
fn main() {
    let solution1 = solve1("Data.txt");
    assert!(solution1 > 1552637);
    println!("Part 1: {solution1}");
    let solution2 = solve2("Data.txt");
    // assert!(solution1 > 1552637);
    println!("Part 2: {solution2}");
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_data_1(){
        let file_name = "TestData1.txt";
        let expected = 11;
        let actual = solve1(file_name);
        assert_eq!(actual,expected)
    }
    #[test]
    fn test_data_2(){
        let file_name = "TestData1.txt";
        let expected = 31;
        let actual = solve2(file_name);
        assert_eq!(actual,expected)
    }
}