use regex::Regex;

static EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

static EXAMPLE_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn solve_ex1(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    pattern
        .captures_iter(input)
        .map(|captures| {
            let a = captures[1].parse::<u32>()?;
            let b = captures[2].parse::<u32>()?;
            Ok::<u32, std::num::ParseIntError>(a * b)
        })
        .filter_map(|val: Result<u32, _>| val.ok())
        .reduce(|acc, val| acc + val)
}

fn solve_ex2(input: &str) -> u32 {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();

    let mut res = 0;
    let mut enabled = true;

    for operation in pattern.captures_iter(input) {
        match operation.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    let a = operation[1].parse::<u32>().unwrap();
                    let b = operation[2].parse::<u32>().unwrap();
                    res += a * b;
                }
            }
        }
    }

    res
}

fn main() {
    println!("{:#?}", solve_ex1(EXAMPLE).unwrap());
    println!("{:#?}", EXAMPLE_2);
    println!("{:#?}", solve_ex2(EXAMPLE_2));

    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{:#?}", solve_ex1(&input).unwrap());
    println!("{:#?}", solve_ex2(&input));
}
