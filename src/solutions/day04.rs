pub fn part1(text: &str) -> String {
    let mut split = text.split('-');
    let lower_bound = split.next().unwrap().parse::<usize>().unwrap();
    let upper_bound = split.next().unwrap().parse::<usize>().unwrap();
    let mut count = 0;
    for num in lower_bound..=upper_bound {
        let mut valid = true;
        let mut double = false;
        for index in 0..5 {
            if get_digit(num, index) < get_digit(num, index + 1) {
                valid = false;
                break;
            } else if get_digit(num, index) == get_digit(num, index + 1) {
                double = true
            }
        }
        if valid && double {
            count += 1;
        }
    }
    count.to_string()
}

fn get_digit(num: usize, index: usize) -> usize {
    (num / 10usize.pow(index as u32)) % 10
}

pub fn part2(text: &str) -> String {
    let mut split = text.split('-');
    let lower_bound = split.next().unwrap().parse::<usize>().unwrap();
    let upper_bound = split.next().unwrap().parse::<usize>().unwrap();
    let mut count = 0;
    for num in lower_bound..=upper_bound {
        let mut valid = true;
        let mut double = false;
        for index in 0..5 {
            if get_digit(num, index) < get_digit(num, index + 1) {
                valid = false;
                break;
            }
        }
        let mut last = 0;
        let mut c = 0;
        for index in 0..6 {
            if get_digit(num, index) == last {
                c += 1;
            } else {
                last = get_digit(num, index);
                if c == 2 {
                    double = true
                }
                c = 1;
            }
        }
        if c == 2 {
            double = true
        }
        if valid && double {
            count += 1;
        }
    }
    count.to_string()
}
