fn main() {
    let input = include_str!("../../../inputs/day2");

    let part_one = input.lines().fold((0, 0), |acc, e| {
        let (mut horiz, mut depth) = acc;
        let mut split = e.split_whitespace();
        let dir = split.next().unwrap();
        let num = split.next().unwrap().parse::<u32>().unwrap();
        match dir {
            "forward" => horiz += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => unreachable!(),
        }
        (horiz, depth)
    });
    println!("Part 1: {}", part_one.0 * part_one.1);

    let part_two = input.lines().fold((0, 0, 0), |acc, e| {
        let (mut aim, mut horiz, mut depth) = acc;
        let mut split = e.split_whitespace();
        let dir = split.next().unwrap();
        let num = split.next().unwrap().parse::<u32>().unwrap();
        match dir {
            "forward" => {
                horiz += num;
                depth += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => unreachable!(),
        }
        (aim, horiz, depth)
    });
    println!("Part 2: {}", part_two.1 * part_two.2);
}
