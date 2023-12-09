use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  let nums = lines.take_while(|l| l.len() > 0)
                  .map(|l| l.parse::<usize>().unwrap())
                  .collect::<Vec<_>>();
                  
  let mut counts = vec![vec![0; nums.len() + 1]; 151];
  for n in &nums {
    let mut cn = vec![vec![0; nums.len() + 1]; 151];
    for i in 1..(150 - n + 1) {
      for j in 0..nums.len() {
        cn[i + n][j + 1] += counts[i][j];
      }
    }
    cn[*n][1] += 1;
    for i in 0..151 {
      for j in 0..(nums.len() + 1) {
        counts[i][j] += cn[i][j];
      }
    }
  }

  println!("{}", counts[150].iter().find(|c| **c > 0).unwrap());
}