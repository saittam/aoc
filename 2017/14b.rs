use std::io::BufRead;
use std::collections::HashSet;
use std::collections::VecDeque;

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
  let mut used = (0..128)
    .map(|i| knothash(&format!("{}-{}", key, i)))
    .enumerate()
    .fold(HashSet::new(), |mut s, (x, h)| {
      s.extend(h.iter()
        .flat_map(|b| (0..8).rev()
                            .map(move |i| b & (1 << i) != 0))
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(y, _)| (x + 1, y + 1)));
      s
    });

  let mut n = 0;
  while let Some(p) = used.iter().next() {
    n += 1;
    let mut queue = VecDeque::new();
    queue.push_back(*p);
    while let Some((x, y)) = queue.pop_front() {
      if !used.remove(&(x, y)) {
        continue;
      }

      queue.extend([
        (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)
      ]);
    }
  }
  
  println!("{}", n);
}