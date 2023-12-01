#![feature(destructuring_assignment)]

fn main() {
    let input = include_str!("../../inputs/day4").trim();

    let mut answer = 1;

    while calc_md5(&format!("{}{}", input, answer)).leading_zeros() < 20 {
        answer += 1;
    }
    println!("Part 1: {}", answer);

    while calc_md5(&format!("{}{}", input, answer)).leading_zeros() < 24 {
        answer += 1;
    }
    println!("Part 2: {}", answer);
}

fn calc_md5(input: &str) -> u128 {
    use std::num::Wrapping;
    let mut input = input.as_bytes().to_owned();
    let len = input.len() as u64 * 8;

    let shifts = [7, 12, 17, 22, 5, 9, 14, 20, 4, 11, 16, 23, 6, 10, 15, 21];
    let sines = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    let mut a0 = 0x67452301_u32;
    let mut b0 = 0xefcdab89_u32;
    let mut c0 = 0x98badcfe_u32;
    let mut d0 = 0x10325476_u32;

    input.push(0x80);
    while input.len() % 64 != 56 {
        input.push(0x00);
    }

    for byte in len.to_le_bytes() {
        input.push(byte);
    }

    for chunk in input.chunks_exact(64) {
        let chunk = chunk.to_owned();
        let m = chunk
            .chunks(4)
            .map(|e| u32::from_le_bytes([e[0], e[1], e[2], e[3]]))
            .collect::<Vec<u32>>();
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..64 {
            let (mut f, g): (u32, _) = match i {
                0..=15 => (((b & c) | (!b & d)), i),
                16..=31 => (((d & b) | (!d & c)), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                48..=63 => (c ^ (b | !d), ((7 * i) % 16)),
                _ => unreachable!(),
            };

            Wrapping(f) = Wrapping(f) + Wrapping(a) + Wrapping(sines[i]) + Wrapping(m[g]);
            a = d;
            d = c;
            c = b;
            Wrapping(b) = Wrapping(b) + Wrapping(f.rotate_left(shifts[(i / 16) * 4 + i % 4]));
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    ((a0.swap_bytes() as u128) << 96)
        + ((b0.swap_bytes() as u128) << 64)
        + ((c0.swap_bytes() as u128) << 32)
        + (d0.swap_bytes() as u128)
}
