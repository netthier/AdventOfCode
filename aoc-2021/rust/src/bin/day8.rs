use ::rand::{seq::SliceRandom, thread_rng, Rng};

fn main() {
    let input = include_str!("../../../inputs/day8");
    let parsed: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|e| {
            let tup = e.split_once('|').unwrap();
            let helper = |e: &str| {
                e.split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>()
            };
            (helper(tup.0), helper(tup.1))
        })
        .collect();

    println!(
        "Part 1: {}",
        parsed
            .iter()
            .flat_map(|e| e.1.iter().filter(|e| e.len() != 5 && e.len() != 6))
            .count()
    );

    let valid = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    let mut key: Vec<char> = ('a'..='g').collect();
    let fitness = |e: &mut dyn Iterator<Item = &String>, key: &[char]| -> usize {
        e.filter(|e| valid.contains(&decode(e, key).as_str()))
            .count()
    };

    fn decode(e: &str, key: &[char]) -> String {
        let mut vec = e
            .chars()
            .map(|e| key[(e as u8) as usize - 0x61])
            .collect::<Vec<char>>();
        vec.sort();
        vec.iter().collect::<String>()
    }

    let mut sum = 0;
    let mut rng = thread_rng();

    for (sigs, output) in parsed.iter() {
        let mut curr_fitness = fitness(&mut sigs.iter().chain(output.iter()), &key);
        let mut iter_counter = 0;
        while curr_fitness < 14 {
            if iter_counter > 100 {
                iter_counter = 0;
                key.shuffle(&mut rng);
            } else {
                iter_counter += 1;
            }
            let mut new_key = key.clone();
            new_key.swap(rng.gen_range(0..7), rng.gen_range(0..7));
            let new_fitness = fitness(&mut sigs.iter().chain(output.iter()), &new_key);
            if new_fitness > curr_fitness {
                key = new_key;
                curr_fitness = new_fitness;
            }
        }
        sum += output
            .iter()
            .map(|e| decode(e, &key))
            .map(|e| valid.iter().position(|f| &&e == f).unwrap())
            .enumerate()
            .map(|(idx, e)| e * 10_usize.pow(3 - idx as u32))
            .sum::<usize>();
    }

    println!("Part 2: {}", sum);
}
