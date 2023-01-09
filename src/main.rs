use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    // let file = fs::File::open("data/input.txt")?;
    // let reader = io::BufReader::new(file);
    // let mut sums = Vec::new();
    // let mut sum = 0;

    // for line in reader.lines() {
    //     let line = line?;
    //     if line.is_empty() {
    //         sums.push(sum);
    //         sum = 0;
    //         continue;
    //     }
    //     let number = line.parse::<i32>().unwrap();
    //     sum += number;
    // }
    // sums.push(sum);
    // println!("Min: {}", sums.iter().min().unwrap());
    // println!("Max: {}", sums.iter().max().unwrap());
    // let top_3 = top_n((sums, 3));
    // // sum the top 3
    // let sum_top_3 = top_3.iter().sum::<i32>();
    // println!("Sum of top 3: {}", sum_top_3);
    let score = rocks_paper_scissors();
    println!("Score: {}", score);
    Ok(())
}

fn _top_n((arr , n): (Vec<i32>, i32)) -> Vec<i32> {
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

fn rocks_paper_scissors() -> i32 {
    let file: fs::File = fs::File::open("data/rock_paper_scissors.txt").unwrap();
    let reader: io::BufReader<fs::File> = io::BufReader::new(file);
    let mut score: i32 = 0;
    reader.lines().for_each(|line: Result<String, io::Error>| {
        let line: String = line.unwrap();
        let moves: Vec<&str> = line.split_whitespace().collect();
        let opponent_move: String = moves[0].to_string();
        let self_move: String = moves[1].to_string();
        let calculated_score: i32 = calculate_score((opponent_move, self_move));
        score += calculated_score;
    });
    return score;
}

fn calculate_score((opponent_move, self_move): (String, String)) -> i32 {
    if self_move == "X" { // Need to lose
        match opponent_move.as_str() {
            "A" => return 3, // rock beats scissors
            "B" => return 1, // paper beats rock
            "C" => return 2, // scissors beats paper
            _ => panic!("Invalid opponent move"),
        }
    }
    if self_move == "Y" { // Need to draw
        match opponent_move.as_str() {
            "A" => return 4,
            "B" => return 5,
            "C" => return 6,
            _ => panic!("Invalid opponent move"),
        }
    }
    if self_move == "Z" { // Need to win
        match opponent_move.as_str() {
            "A" => return 2 + 6,
            "B" => return 3 + 6,
            "C" => return 1 + 6,
            _ => panic!("Invalid opponent move"),
        }
    }
    panic!("Invalid self move");

}

