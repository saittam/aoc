use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  let nums = lines.take_while(|l| l.len() > 0)
                  .map(|l| l.parse::<usize>().unwrap())
                  .collect::<Vec<_>>();
                  
  let mut counts = vec![0; 151];
  for n in nums {
    let mut cn = vec![0; 151];
    for i in 1..(150 - n + 1) {
      cn[i + n] += counts[i];
    }
    cn[n] += 1;
    for i in 0..151 {
      counts[i] += cn[i];
    }
  }

  println!("{}", counts[150]);
}