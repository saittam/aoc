use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let input = lines.next().expect("input")
    .chars().collect::<Vec<_>>();

  
  const PI : [(usize, usize); 6] = [
    (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)
  ];
  let r = input.windows(4)
    .position(|w| PI.iter().all(|(a, b)| w[*a] != w[*b]));
  println!("{}", r.expect("no marker") + 4);
}