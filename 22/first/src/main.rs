use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    let mut calories: Vec<i32> = vec![];
    let mut total = 0;

    input.split("\n").for_each(|line| {
        if line.is_empty() {
            calories.push(total);
            total = 0;

            return;
        }

        let n: i32 = line.parse().unwrap();
        total += n;
    });

    calories.sort_by(|a, b| b.cmp(a));

    let top_three = calories.iter().take(3).sum::<i32>();
    println!("{}", top_three);
}
