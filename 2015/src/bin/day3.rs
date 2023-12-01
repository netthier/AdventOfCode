fn main() {
    let input = include_str!("../../inputs/day3").trim();

    fn calc_pos(pos: (isize, isize), e: char) -> (isize, isize) {
        match e {
            '^' => (pos.0, pos.1 + 1),
            '>' => (pos.0 + 1, pos.1),
            'v' => (pos.0, pos.1 - 1),
            '<' => (pos.0 - 1, pos.1),
            _ => unreachable!(),
        }
    }

    let mut visited_set = std::collections::HashSet::new();
    input.chars().fold((0, 0), |acc, e| {
        let ret = calc_pos(acc, e);
        visited_set.insert(ret);
        ret
    });
    println!("Part 1: {}", visited_set.len());

    visited_set.clear();

    input
        .chars()
        .enumerate()
        .fold(((0, 0), (0, 0)), |(s_acc, r_acc), (idx, e)| {
            if idx % 2 == 0 {
                let pos = calc_pos(s_acc, e);
                visited_set.insert(pos);
                (pos, r_acc)
            } else {
                let pos = calc_pos(r_acc, e);
                visited_set.insert(pos);
                (s_acc, pos)
            }
        });
    println!("Part 2: {}", visited_set.len());
}
