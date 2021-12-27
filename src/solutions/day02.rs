pub fn part1(text: &str) -> String {
    let mut op_codes: Vec<usize> = text.split(',').flat_map(|num| num.parse()).collect();
    op_codes[1] = 12;
    op_codes[2] = 2;
    let mut i = 0;
    loop {
        match op_codes[i] {
            1 => add(&mut op_codes, i),
            2 => mul(&mut op_codes, i),
            99 => break,
            _ => panic!(),
        }
        i += 4;
    }
    op_codes[0].to_string()
}

fn add(op_codes: &mut [usize], i: usize) {
    op_codes[op_codes[i + 3]] = op_codes[op_codes[i + 1]] + op_codes[op_codes[i + 2]];
}
fn mul(op_codes: &mut [usize], i: usize) {
    op_codes[op_codes[i + 3]] = op_codes[op_codes[i + 1]] * op_codes[op_codes[i + 2]];
}

pub fn part2(text: &str) -> String {
    let op_codes: Vec<usize> = text.split(',').flat_map(|num| num.parse()).collect();
    for a in 0..op_codes.len() {
        for b in a..op_codes.len() {
            let mut op_codes = op_codes.clone();
            op_codes[1] = a;
            op_codes[2] = b;
            let mut i = 0;
            loop {
                match op_codes[i] {
                    1 => add(&mut op_codes, i),
                    2 => mul(&mut op_codes, i),
                    99 => break,
                    _ => panic!(),
                }
                i += 4;
            }
            if op_codes[0] == 19690720 {
                return (100 * a + b).to_string();
            }
        }
    }
    String::new()
}
