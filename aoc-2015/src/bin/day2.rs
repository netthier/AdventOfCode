fn main() {
    let input = include_str!("../../inputs/day2");
    let parsed = input.lines().map(|e| {
        e.split('x')
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    });

    let part_one = parsed
        .clone()
        .map(|sides| {
            let areas = vec![
                2 * sides[0] * sides[1],
                2 * sides[1] * sides[2],
                2 * sides[2] * sides[0],
            ];
            areas.iter().sum::<usize>() + areas.iter().min().unwrap() / 2
        })
        .sum::<usize>();
    println!("Part 1: {}", part_one);

    let part_two = parsed
        .map(|mut sides| {
            let max = *sides.iter().max().unwrap();
            let volume = sides.iter().product::<usize>();
            sides.remove(sides.iter().position(|e| *e == max).unwrap());
            sides.iter().sum::<usize>() * 2 + volume
        })
        .sum::<usize>();
    println!("Part 2: {}", part_two);
}
