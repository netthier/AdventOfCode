#![feature(let_else)]

fn main() {
    let input = include_str!("../../inputs/day1").trim();
    let part_one = input
        .chars()
        .fold(0, |acc, e| if e == '(' { acc + 1 } else { acc - 1 });
    println!("Part 1: {}", part_one);
    let Err(part_two) = input.chars().try_fold((0, 0), |acc, e| if acc.1 == -1 { Err(acc.0) } else { Ok(if e == '(' { (acc.0 + 1, acc.1 + 1) } else { (acc.0 + 1, acc.1 - 1) })}) else {
        panic!("Santa never reached floor -1!");
    };
    println!("Part 2: {}", part_two);
}
