use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let lengths = lines.next().expect("input")
    .split(',')
    .map(|w| w.parse::<usize>().expect("num"))
    .collect::<Vec<_>>();

  const LEN: usize = 256;
  let mut list = (0..LEN).collect::<Vec<_>>();
  let mut rn = 0;
  for (s, l) in lengths.iter().enumerate() {
    list[0..*l].reverse();
    list.rotate_left((l + s) % LEN);
    rn += l + s;
  }

  let k = LEN - rn % LEN;
  println!("{}", list[k] * list[(k + 1) % LEN]);
}