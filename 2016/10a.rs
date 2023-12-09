use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Target {
  Bot(usize),
  Output(usize),
}

#[derive(Debug)]
enum Instruction {
  Value(usize, Target),
  Bot(usize, Target, Target),
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut instructions = lines
    .map(|l| {
      let wi = l.split_whitespace().collect::<Vec<_>>();
      let idx = wi[1].parse::<usize>().expect("num");
      let mut targets = wi[2..].windows(2)
        .filter_map(|w| {
          let idx = w[1].parse::<usize>().ok()?;
          Some(match w[0] {
            "bot" => Target::Bot(idx),
            "output" => Target::Output(idx),
            _ => return None,
          })
        });
      match wi[0] {
        "value" =>
          Instruction::Value(idx,
                             targets.next().expect("target")),
        "bot" =>
          Instruction::Bot(idx,
                           targets.next().expect("low"),
                           targets.next().expect("high")),
        _ => panic!("instruction")
      }
    })
    .collect::<Vec<_>>();

  let mut values = HashMap::new();
  let insert = |m: &mut HashMap<Target, Vec<usize>>, t, v|
    m.entry(t).or_insert_with(Vec::new).push(v);
  let mut progress = true;
  while progress {
    progress = false;
    instructions = instructions.into_iter()
      .filter(|i| {
        let done = match i {
          Instruction::Value(v, t) => {
            insert(&mut values, *t, *v);
            true
          }
          Instruction::Bot(b, tl, th) => {
            let e = values.get(&Target::Bot(*b));
            if let Some([a, b]) = e.map(|v| v.as_slice()) {
              let (l, h) =
                if a < b { (*a, *b) } else { (*b, *a) };
              insert(&mut values, *tl, l);
              insert(&mut values, *th, h);
              true
            } else {
              false
            }
          }
        };
        progress |= done;
        !done
      })
      .collect::<Vec<_>>();
  }

  if let (Target::Bot(b), _) = values.iter()
    .find(|(_, v)| **v == [61, 17] || **v == [17, 61])
    .expect("no solution") {
    println!("{}", b);
  }
}