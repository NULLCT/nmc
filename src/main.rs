use std::io::{stdin, stdout, Write};

#[derive(PartialEq)]
enum Operators {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Mod,
}

fn parse_input(input: String) -> i64 {
    let mut buf = String::new();
    let mut nums: Vec<i64> = Vec::new();
    let mut opts: Vec<Operators> = Vec::new();

    fn buf_flush(buf: &mut String, nums: &mut Vec<i64>, opts: &mut Vec<Operators>, ope: Operators) {
        nums.push(buf.parse().unwrap());
        opts.push(ope);
        buf.clear();
    }

    for i in input.chars() {
        match i {
            '+' => {
                buf_flush(&mut buf, &mut nums, &mut opts, Operators::Addition);
            }
            '-' => {
                buf_flush(&mut buf, &mut nums, &mut opts, Operators::Subtraction);
            }
            '*' => {
                buf_flush(&mut buf, &mut nums, &mut opts, Operators::Multiplication);
            }
            '/' => {
                buf_flush(&mut buf, &mut nums, &mut opts, Operators::Division);
            }
            '%' => {
                buf_flush(&mut buf, &mut nums, &mut opts, Operators::Mod);
            }
            ' ' => {}
            _ => {
                buf.push(i);
            }
        }
    }
    nums.push(buf.parse().unwrap());

    let mut res: i64 = nums[0];

    for (i, val) in opts.iter().enumerate() {
        match val {
            Operators::Addition => {
                res += nums[i + 1];
            }
            Operators::Subtraction => {
                res -= nums[i + 1];
            }
            Operators::Multiplication => {
                res *= nums[i + 1];
            }
            Operators::Division => {
                res /= nums[i + 1];
            }
            Operators::Mod => {
                res %= nums[i + 1];
            }
        }
    }

    return res;
}

fn main() {
    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Failed to read line");
        input.pop();
        println!("{}", parse_input(input));
    }
}
