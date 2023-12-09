use std::io::BufRead;

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

  let mut di = dir.iter().cycle();
  for s in SHAPES.iter().cycle().take(2022) {
    let mut s = *s;
    let mut env = 0;
    let mut si = stack.iter().enumerate().rev();
    let h = loop {
      let (n, l) = si.next().expect("floor");
      let st = s.rotate_left(*di.next().unwrap());
      if st & (WALL | env) == 0 {
        s = st;
      }
      env = (env << 8) | *l as u32;
      if s & env != 0 {
        break n + 1;
      }
    };
    let sh = 4 - (s.leading_zeros() / 8) as usize;
    stack.resize(std::cmp::max(stack.len(), h + sh + 3), 0);
    for i in 0..4 {
      stack[h + i] |= s as u8;
      s = s >> 8;
    }
  }

  println!("{}", stack.len() - 4)
}