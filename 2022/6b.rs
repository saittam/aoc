use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let input = lines.next().expect("input")
    .chars().collect::<Vec<_>>();

  const N: usize = 14;
  let pairs =
    (0..N).flat_map(|a| ((a + 1)..N).map(move |b| (a, b)))
    .collect::<Vec<_>>();
  let r = input.windows(N)
    .position(|w| pairs.iter().all(|(a, b)| w[*a] != w[*b]));
  println!("{}", r.expect("no marker") + N);
}