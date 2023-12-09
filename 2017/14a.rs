use std::io::BufRead;

fn knothash(s: &str) -> [u8; 16] {
  let lens = s.chars()
    .map(|c| c as u32 as usize)
    .chain([17, 31, 73, 47, 23])
    .collect::<Vec<_>>();

  const LEN: usize = 256;
  let mut list = (0..=((LEN - 1) as u8)).collect::<Vec<_>>();
  let mut rn = 0;
  let count = lens.len() * 64;
  for (s, l) in lens.iter().cycle().take(count).enumerate() {
    list[0..*l].reverse();
    list.rotate_left((l + s) % LEN);
    rn += l + s;
  }
  list.rotate_right(rn % LEN);

  let mut hash = [0; 16];
  let bytes = list.chunks(16)
    .map(|w| w.iter().fold(0, |r, b| r ^ b));
  for (e, h) in hash.iter_mut().zip(bytes) {
    *e = h;
  }
  hash
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let key = lines.next().expect("key");
  let n = (0..128)
    .map(|i| knothash(&format!("{}-{}", key, i)))
    .map(|h| h.iter().map(|b| b.count_ones()).sum::<u32>())
    .sum::<u32>();
  
  println!("{}", n);
}