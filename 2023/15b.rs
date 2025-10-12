use std::io::BufRead;
use std::collections::HashMap;

enum Op {
  Remove,
  Insert(u64),
}

fn hash(s: &str) -> u8 {
  s.chars().fold(0, |h, c| (h + c as u32) * 17 & 0xff) as u8
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let input = lines.next().expect("input");
  let seq = input.split(',')
    .map(|w| {
      let (pos, op) = w.match_indices(&['-', '='])
        .next().expect("op");
      let label = w.get(0..pos).expect("label");
      let op = match op {
        "-" => Op::Remove,
        "=" => {
          let f = w.get((pos + 1)..).expect("focal len")
            .parse::<u64>().expect("num");
          Op::Insert(f)
        }
        _ => panic!("bad op {}", op)
      };
      (label, op)
    })
    .collect::<Vec<_>>();

  let state = seq.iter().fold(
    HashMap::new(),
    |mut state, (label, op)| {
      let v = state.entry(hash(label))
        .or_insert_with(Vec::new);
      let pos = v.iter().position(|(ll, _)| *ll == label);
      match op {
        Op::Remove => if let Some(p) = pos {
          v.remove(p);
        }
        Op::Insert(f) => match pos {
          Some(p) => v[p] = (label, f),
          None => v.push((label, f)),
        }                               
      }
      state
    });

  let n = state.iter().map(|(i, v)| {
      let fs = v.iter().enumerate()
        .map(|(j, (_, f))| (j + 1) as u64 * *f)
        .sum::<u64>();
      (*i as u64 + 1) * fs
    })
    .sum::<u64>();
                     
  println!("{}", n);
}