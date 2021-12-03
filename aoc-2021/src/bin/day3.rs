fn main() {
    let input = include_str!("../../inputs/day3");
    let lines = input.lines().count();
    let bits = input.lines().next().unwrap().chars().count();
    let mask = usize::MAX ^ ((1 << bits) - 1);

    let fold_fn = |mut acc: Vec<usize>, e: usize| {
        for (idx, bit) in acc.iter_mut().enumerate() {
            *bit += (e >> ((bits - 1) - idx)) & 1;
        }
        acc
    };

    let part_one = input
        .lines()
        .map(|e| usize::from_str_radix(e, 2).unwrap())
        .fold(vec![0; bits], fold_fn)
        .iter()
        .map(|e| if *e >= lines / 2 { 1 } else { 0 })
        .enumerate()
        .map(|(idx, e)| e << (bits - (idx + 1)))
        .sum::<usize>();

    println!("Part 1: {}", part_one * !(part_one | mask));

    let mut o2: Vec<usize> = input
        .lines()
        .map(|e| usize::from_str_radix(e, 2).unwrap())
        .collect();
    let mut co2 = o2.clone();

    for idx in 0..bits {
        if o2.len() > 1 {
            let len = o2.len();
            let acc = o2.iter().copied().fold(vec![0; bits], fold_fn);
            o2.retain(|e| (e >> (bits - (idx + 1))) & 1 == (acc[idx] * 2 >= len) as usize);
        }

        if co2.len() > 1 {
            let len = co2.len();
            let acc = co2.iter().copied().fold(vec![0; bits], fold_fn);
            co2.retain(|e| (e >> (bits - (idx + 1))) & 1 == (acc[idx] * 2 < len) as usize);
        }
    }
    println!("Part 2: {}", o2[0] * co2[0]);
}
