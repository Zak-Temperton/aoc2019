pub fn part1(text: &str) -> String {
    let op_codes: Vec<isize> = text.split(',').flat_map(|n| n.parse()).collect();
    run_instructions(op_codes, 1).last().unwrap().to_string()
}

pub fn part2(text: &str) -> String {
    let op_codes: Vec<isize> = text.split(',').flat_map(|n| n.parse()).collect();
    run_instructions(op_codes, 5).last().unwrap().to_string()
}

fn run_instructions(mut op_codes: Vec<isize>, input: isize) -> Vec<isize> {
    let mut output = Vec::new();

    let mut index = 0;

    while index < op_codes.len() {
        let op_code = op_codes[index];
        match op_code % 100 {
            //add
            1 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param2 = match as_value(op_code / 1000, &op_codes, index, 2) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param3 = op_codes[index + 3] as usize;

                op_codes[param3] = param1 + param2;
                index += 4;
            }
            //mul
            2 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param2 = match as_value(op_code / 1000, &op_codes, index, 2) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param3 = op_codes[index + 3] as usize;

                op_codes[param3] = param1 * param2;
                index += 4;
            }
            //input
            3 => {
                let param = op_codes[index + 1] as usize;
                op_codes[param] = input;
                index += 2;
            }
            //Store
            4 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                output.push(param1);
                index += 2;
            }
            //jump_if_true
            5 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };

                let param2 = match as_value(op_code / 1000, &op_codes, index, 2) {
                    Value::Position(index) => op_codes[index] as usize,
                    Value::Immediate(num) => num as usize,
                };
                if param1 != 0 {
                    index = param2;
                } else {
                    index += 3;
                }
            }
            //jump_if_false
            6 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };

                let param2 = match as_value(op_code / 1000, &op_codes, index, 2) {
                    Value::Position(index) => op_codes[index] as usize,
                    Value::Immediate(num) => num as usize,
                };
                if param1 == 0 {
                    index = param2;
                } else {
                    index += 3;
                }
            }
            //Less_than
            7 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param2 = match as_value(op_code / 1000, &op_codes, index, 2) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param3 = op_codes[index + 3] as usize;

                op_codes[param3] = if param1 < param2 { 1 } else { 0 };
                index += 4;
            } //equal
            8 => {
                let param1 = match as_value(op_code / 100, &op_codes, index, 1) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param2 = match as_value(op_code / 1000, &op_codes, index, 2) {
                    Value::Position(index) => op_codes[index],
                    Value::Immediate(num) => num,
                };
                let param3 = op_codes[index + 3] as usize;

                op_codes[param3] = if param1 == param2 { 1 } else { 0 };
                index += 4;
            }
            99 => break,
            c => panic!("{} {}", index, c),
        }
    }
    output
}

fn as_value(op_code: isize, op_codes: &[isize], index: usize, param: usize) -> Value {
    match (op_code) % 10 {
        0 => Value::Position(op_codes[index + param] as usize),
        1 => Value::Immediate(op_codes[index + param]),
        _ => panic!(),
    }
}

enum Value {
    Immediate(isize),
    Position(usize),
}
