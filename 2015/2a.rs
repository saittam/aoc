use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let area = lines.take_while(|l| l.len() > 0)
       .map(|l| l.split('x')
                 .map(|s| s.parse::<u64>().unwrap())
                 .collect::<Vec<_>>())
       .map(|d| (0..3).map(|i| d[i] * d[(i + 1) % 3])
                      .collect::<Vec<_>>())
       .map(|a| 2 * a.iter().sum::<u64>() + a.iter().min().unwrap())
       .sum::<u64>();

  println!("{}", area);
}