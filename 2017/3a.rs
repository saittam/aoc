use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let k = lines.next().expect("line")
    .parse::<i32>().expect("num");

  // 1 + 8 + 16 + 24 + ..
  // 1 + sum_i 8 * i
  // 1 + 4 * n * (n + 1)
  // 4 * (n² + n + 1/4)
  // 4 * (n + 1/2)²
  //
  // sqrt(k)/2 = n + 1/2
  // (sqrt(k) - 1) / 2 = n

  let n = (((k - 1) as f64).sqrt() as i32 - 1) / 2 + 1;
  let r = k - 1 - (1 + 4 * n * (n - 1));
  let p = r % (2 * n ) + 1;
  let d = if p <= n { n - p } else { p - n };

  println!("{}", n + d);
}