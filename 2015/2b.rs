use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let area = lines.take_while(|l| l.len() > 0)
       .map(|l| { let mut a = l
                    .split('x')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                  a.sort();
                  a })
       .map(|d| 2 * (d[0] + d[1]) + d.iter().product::<u64>())
       .sum::<u64>();

  println!("{}", area);
}