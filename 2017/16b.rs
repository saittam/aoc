use std::io::BufRead;
use std::collections::HashMap;

enum Move {
  Spin(usize),
  Exchange(usize, usize),
  Partner(char, char),
}

fn dance(moves: &[Move], state: &mut [char]) {
  let mut p = 0;
  let len = state.len();
  for m in moves {
    match m {
      Move::Spin(n) => p = (p + (len - n)) % len,
      Move::Exchange(i1, i2) =>
        state.swap((p + i1) % len, (p + i2) % len),
      Move::Partner(c1, c2) => {
        let i1 = state.iter()
          .position(|c| *c == *c1).expect("c1");
        let i2 = state.iter()
          .position(|c| *c == *c2).expect("c2");
        state.swap(i1, i2);
      }
    }
  }
  state.rotate_left(p);
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let moves = lines.next().expect("input")
    .split(',')
    .map(|w| {
      let mut ci = w.chars();
      let o = ci.next()?;
      let pi = ci.as_str().split('/');
      let mut pni = pi.clone()
        .map(|p| p.parse::<usize>().expect("num"));
      let mut pci = pi.clone()
        .map(|p| p.chars().next().expect("char"));
      Some(match o {
        's' => Move::Spin(pni.next()?),
        'x' => Move::Exchange(pni.next()?, pni.next()?),
        'p' => Move::Partner(pci.next()?, pci.next()?),
        _ => panic!("bad op {}", o),
      })
    })
    .map(|m| m.expect("move parse"))
    .collect::<Vec<_>>();

  const LEN: usize = 16;
  const N: usize = 1_000_000_000;
  let mut state = ('a'..).take(LEN).collect::<Vec<_>>();
  let mut seen = HashMap::new();
  let mut stop = None;
  for i in 0.. {
    if Some(i) == stop {
      break;
    }
    if let Some(p) = seen.insert(state.clone(), i) {
      stop = Some(i + (N - i) % (i - p));
    }
    dance(&moves, &mut state);
  };

  println!("{}", state.iter().collect::<String>());
}