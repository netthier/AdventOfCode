fn main() {
    let input = include_str!("../../inputs/day4");
    let nums: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<Option<u32>>>> = input
        .split("\n\n")
        .skip(1)
        .map(|e| {
            e.lines()
                .map(|e| {
                    e.split_whitespace()
                        .map(|e| Some(e.parse::<u32>().unwrap()))
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut first = None;
    let mut last = 0;
    for num in nums.iter() {
        for board in boards.iter_mut() {
            if let Some(pos) = board
                .iter()
                .enumerate()
                .filter_map(|(idx, e)| e.iter().position(|e| *e == Some(*num)).map(|e| (idx, e)))
                .next()
            {
                board[pos.0][pos.1] = None;
                if board[pos.0].iter().all(Option::is_none)
                    || board.iter().map(|e| &e[pos.1]).all(Option::is_none)
                {
                    let res = board
                        .iter()
                        .map(|e| e.iter().filter_map(|x| *x))
                        .flatten()
                        .sum::<u32>()
                        * num;

                    if first.is_none() {
                        first = Some(res)
                    }
                    last = res;

                    board.clear()
                }
            }
        }
    }

    println!("Part 1: {}", first.unwrap());
    println!("Part 2: {}", last);
}
