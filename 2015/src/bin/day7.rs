use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day7").trim();
  
    let mut map = input.lines().fold(HashMap::new(), |mut acc, e| {
        let split = e.split(" -> ").collect::<Vec<&str>>();
        acc.insert(split[1].to_string(), split[0].to_string());
        acc
    });

    let part_one = calc_value("a", &mut map.clone());
    println!("Part 1: {}", part_one);

    map.insert("b".to_string(), part_one.to_string());
    println!("Part 2: {}", calc_value("a", &mut map));
}

fn calc_value(key: &str, map: &mut HashMap<String, String>) -> u16 {
    if let Ok(num) = key.parse::<u16>() {
        return num;
    }

    let val = map.get(key).unwrap().to_owned();
    let words = val.split_whitespace().collect::<Vec<&str>>();

    let res = match &words[..] {
        [a] => if let Ok(num) = a.parse::<u16>() { num } else { calc_value(a, map) },
        ["NOT", a] => !calc_value(a, map),
        [a, op, b] => {
            let a = calc_value(a, map);
            let b = calc_value(b, map);
            match *op {
                "AND" => a & b,
                "OR" => a | b,
                "LSHIFT" => a << b,
                "RSHIFT" => a >> b,
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    };

    map.insert(key.to_string(), res.to_string());
    res
}
