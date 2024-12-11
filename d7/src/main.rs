use std::fs::read_to_string;

type Solution = u64;
#[derive(Clone)]
struct Equation{
    left: Solution,
    right: Vec<Solution>
}
impl Equation{
    fn new(s:&str)->Self{
        let [left_s,right_s]: [&str;2] = s.split(':').collect::<Vec<&str>>().try_into().unwrap();
        let left = left_s.parse().unwrap();
        let right: Vec<Solution> = right_s.split(' ').filter(|num_s| num_s.len() > 0).map(|num_s| num_s.parse().unwrap()).collect();
        debug_assert!(!&right.is_empty());
        // println!("{right:?}");
        Self{left,right}
    }
    fn solve(&self, funcs: &[fn(Solution,Solution)->Solution])->Option<Solution>{
        let computations = Self::compute_step(self.right[0],self.right[1], &self.right[2..], funcs);
        let is_solved = computations.into_iter().any(|v| v == self.left);
        if is_solved{
            Some(self.left)
        }else{
            None
        }
        
    }
    fn compute_step(prev: Solution, current: Solution, remaining: &[Solution], funcs: &[fn(Solution,Solution)->Solution])->Vec<Solution>{
        #[cfg(debug_assertions)]
        println!("R: {remaining:?}");
        let output = funcs.iter().map(|f| f(prev,current)).collect::<Vec<Solution>>();
        if let Some(&next) = remaining.first(){
            let advanced_output = output.iter().flat_map(|&v| Self::compute_step(v, next, &remaining[1..], funcs)).collect();
            return advanced_output;
        }
        output
    }
}

fn generic_solve(eqs: &[Equation], funcs: &[fn(Solution,Solution)->Solution])->Solution{
    eqs.iter().filter_map(|equation| equation.solve(&funcs))
        .sum()
}
fn solve1(eqs: &[Equation],)->Solution{
    let funcs = [
        |x:Solution, y:Solution| x+y, 
        |x:Solution, y:Solution| x*y
    ];
    generic_solve(eqs, &funcs)
}
fn solve2(eqs: &[Equation],)->Solution{
    let funcs = [
        |x:Solution, y:Solution| x+y, 
        |x:Solution, y:Solution| x*y, 
        |x:Solution, y:Solution| format!("{x}{y}").parse::<Solution>().unwrap(), 
    ];
    generic_solve(eqs, &funcs)
}
fn make_equations(s:String)->Vec<Equation>{
    s.lines().map(|line| Equation::new(line)).collect()
}
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let eqs = make_equations(s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&eqs);
    assert_eq!(solution1,3245122495150);
    let s1_end = std::time::Instant::now();
    let solution2 = solve2(&eqs);
    assert_eq!(solution2,105517128211543);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Parse file time: {:?}",file_end-start);
    println!("P1 time: {:?}",s1_end-file_end);
    println!("P2 time: {:?}",s2_end-s1_end);
    println!("Total time: {:?}",s2_end-start);
}   

#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn solve_test1(){
        let file_name = "TestData1.txt";
        let s = read_to_string(file_name).unwrap();
        let eqs = make_equations(s);
        let answer = solve1(&eqs);
        let expected = 3749;
        assert_eq!(answer,expected)
    }

    #[test]
    fn solve_minimal_case(){
        let s = "190: 10 19".to_string();
        let eqs = make_equations(s);
        let answer = solve1(&eqs);
        let expected = 190;
        assert_eq!(answer,expected)
    }
}