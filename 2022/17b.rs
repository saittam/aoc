use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let dir = lines.next().expect("dir").chars()
    .map(|c| match c {
      '<' => 1,
      '>' => 31,
      _ => panic!("char {}", c),
    })
    .collect::<Vec<_>>();

  let mut stack = vec![ 0b01111111, 0, 0, 0 ];

  const SHAPES: [u32; 5] = [
    0b_00011110,
    0b_00001000_00011100_00001000,
    0b_00000100_00000100_00011100,
    0b_00010000_00010000_00010000_00010000,
    0b_00011000_00011000,
  ];
  const WALL: u32 = 0x80808080;

  let mut di = dir.iter().enumerate().cycle();
  let mut heights = vec![0];
  let mut pos = HashMap::new();
  for (ks, s) in SHAPES.iter().cycle().enumerate() {
    let mut s = *s;
    let mut env = 0;
    let mut si = stack.iter().enumerate().rev();
    let (h, kd) = loop {
      let (n, l) = si.next().expect("floor");
      let (kd, r) = di.next().unwrap();
      let st = s.rotate_left(*r);
      if st & (WALL | env) == 0 {
        s = st;
      }
      env = (env << 8) | *l as u32;
      if s & env != 0 {
        break (n + 1, kd);
      }
    };
    let sh = 4 - (s.leading_zeros() / 8) as usize;
    stack.resize(std::cmp::max(stack.len(), h + sh + 3), 0);
    for i in 0..4 {
      stack[h + i] |= s as u8;
      s = s >> 8;
    }
    heights.push(stack.len() - 4);

    if ks % SHAPES.len() == 0 {
      let state = stack.iter().rev().skip(3).take(9)
        .fold(0, |a, l| (a << 7) | (*l as u64));
      let dpv = pos.entry((kd, state)).or_insert(vec![]);
      if dpv.len() >= 4 {
        let r = |i| dpv[i * (dpv.len() / 4)];
        let h = |i| heights[r(i)];
        if stack[h(1)..h(2)] == stack[h(2)..h(3)] {
          const N: usize = 1000000000000;
          let tail = N - r(1);
          let period = r(2) - r(1);
          let (div, rem) = (tail / period, tail % period);
          let th = div * (h(2) - h(1)) + heights[rem + r(1)];
          println!("{th}");
          break;
        }
      }
      dpv.push(ks);
    }
  }
}