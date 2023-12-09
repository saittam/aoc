use std::io::BufRead;

fn show(s: usize) {
  for y in 0..5 {
    let line = (0..5).into_iter().map(
      |x| if (s & (1 << (y * 5) + x)) > 0 { '#' } else { '.' }
    ).collect::<String>();
    println!("{}", line);
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut state = 0;
  for y in 0.. {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }

    let r = buf.trim().chars().enumerate().map(
      |(i, c)| match c {
        '#' => 1 << i,
        '.' => 0,
        _ => panic!("input {}", c),
      }).fold(0, |s, v| s | v);
    
    state = state | (r << (5 * y));
  }
  
  //show(state);
  //println!("");
  
  let mut mask = [[0, 0, 0]; 25];
  for y in 0isize..5 {
    for x in 0isize..5 {
      let neigh = [
        (x, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x, y + 1),
      ];
      
      let (mut l, mut c, mut h) = (0, 0, 0);
      for n in &neigh {
        let b = |i| 1usize << i;
        match n {
          (_, -1) => l |= b(7),
          (_, 5) => l |= b(17),
          (-1, _) => l |= b(11),
          (5, _) => l |= b(13),
          (2, 2) => match (x, y) {
            (2, 1) => h |= b(0) | b(1) | b(2) | b(3) | b(4),
            (1, 2) => h |= b(0) | b(5) | b(10) | b(15) | b(20),
            (3, 2) => h |= b(4) | b(9) | b(14) | b(19) | b(24),
            (2, 3) => h |= b(20) | b(21) | b(22) | b(23) | b(24),
            _ => panic!("center {:?}", (x, y)),
          },
          (nx, ny) => c |= b(ny * 5 + nx),
        }
      }
      
      mask[(y * 5 + x) as usize] = [l, c, h];
    }
  }
  mask[12] = [0, 0, 0];
  
  let mut v = vec![0, 0, state, 0, 0];
  for _ in 0..200 {
    let mut start = 0;
    while v[start + 2] == 0 {
      start += 1;
    }
    let mut end = v.len();
    while v[end - 3] == 0 {
      end -= 1;
    }
    
    let mut vnew = Vec::new();
    vnew.extend(&[0, 0]);
    for w in v[start..end].windows(3) {
      let mut news = 0;
      for (i, m) in mask.iter().enumerate() {
        let s = w.iter().zip(m.iter()).map(
          |(vv, mv)| (vv & mv).count_ones()
        ).sum();
        let bit = match (s, (w[1] & (1 << i)) > 0) {
          (1, true) => true,
          (1, false) | (2, false) => true,
          _ => false,
        };
        news |= (bit as usize) << i;
      }
      vnew.push(news);
    }
    vnew.extend(&[0, 0]);
    v = vnew;
  }
  
  println!("{}", v.iter().map(|s| s.count_ones()).sum::<u32>());
}