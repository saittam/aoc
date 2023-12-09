use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let lengths = lines.next().expect("input").chars()
    .map(|c| c as u32 as usize)
    .chain([17, 31, 73, 47, 23])
    .collect::<Vec<_>>();

  const LEN: usize = 256;
  let mut list = (0..LEN).collect::<Vec<_>>();
  let mut rn = 0;
  let count = lengths.len() * 64;
  for (s, l) in lengths.iter().cycle().take(count).enumerate() {
    list[0..*l].reverse();
    list.rotate_left((l + s) % LEN);
    rn += l + s;
  }
  list.rotate_right(rn % LEN);
  
  let hash = list.chunks(16)
    .map(|w| w.iter().fold(0, |r, b| r ^ b))
    .flat_map(|b| [b >> 4, b & 0xf])
    .map(|d| char::from_digit(d as u32, 16).expect("digit"))
    .collect::<String>();
  println!("{}", hash);
}