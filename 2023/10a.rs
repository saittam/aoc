use std::io::BufRead;
use std::collections::HashMap;

fn neigh((x, y): (usize, usize)) -> [(usize, usize); 4] {
  [ (x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y) ]
}

fn flip(e: u8) -> u8 {
  ((e & 0b1100) >> 2) | ((e & 0b0011) << 2)
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut start = None;
  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().filter_map(
      |(x, c)| Some(((x + 1, y + 1), match c {
        '-' => 0b1010,
        '|' => 0b0101,
        'L' => 0b0011,
        'F' => 0b0110,
        '7' => 0b1100,
        'J' => 0b1001,
        '.' => return None,
        'S' => {
          start = Some((x + 1, y + 1));
          return None;
        }
        _ => panic!("Ceci n'est pas une pipe: {}", c)
      }))).collect::<Vec<_>>())
    .collect::<HashMap<_, _>>();

  let (mut c, mut pos) = neigh(start.expect("start"))
    .into_iter()
    .enumerate()
    .map(|(i, p)| (1 << i, p))
    .find(|(d, p)| matches!(map.get(&p),
                            Some(&mc) if (mc & flip(*d)) != 0))
    .expect("start connection");
  
  let n = [pos].into_iter().chain(
    std::iter::from_fn(|| map.get(&pos).map(|&pc| {
      c = flip(c) ^ pc;
      assert!(c.count_ones() == 1);
      pos = neigh(pos)[c.trailing_zeros() as usize];
      pos
    })))
    .count() / 2;
  
  println!("{}", n);
}