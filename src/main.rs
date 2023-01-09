use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = fs::File::open("data/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut sums = Vec::new();
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
            continue;
        }
        let number = line.parse::<i32>().unwrap();
        sum += number;
    }
    sums.push(sum);
    println!("Min: {}", sums.iter().min().unwrap());
    println!("Max: {}", sums.iter().max().unwrap());
    let top_3 = top_n((sums, 3));
    // sum the top 3
    let sum_top_3 = top_3.iter().sum::<i32>();
    println!("Sum of top 3: {}", sum_top_3);
    Ok(())
}

fn top_n((arr , n): (Vec<i32>, i32)) -> Vec<i32> {
    let mut top_n = Vec::new();
    // sort the array in descending order
    let mut sorted = arr.clone();
    // take the top n elements
    sorted.sort_by(|a, b| b.cmp(a));
    for i in 0..n {
        top_n.push(sorted[i as usize]);
    }
    // return the top n elements
    top_n
}