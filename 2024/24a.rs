use std::io::BufRead;
use std::collections::HashMap;

enum Gate {
  AND,
  OR,
  XOR,
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut idmap = HashMap::new();
  let mut id = |s: &str| {
    let i = idmap.len();
    *idmap.entry(s.to_string()).or_insert(i)
  };
  
  let inputs = lines.by_ref()
    .take_while(|l| !l.is_empty())
    .map(|l| {
      let mut wi = l.split(':');
      (id(wi.next().expect("name")),
       wi.next().expect("value")
       .trim().parse::<u32>().expect("bool") != 0)
    })
    .collect::<HashMap<_, _>>();

  let gates = lines
    .map(|l| {
      let words = l.split_whitespace().collect::<Vec<_>>();
      let gate = match words[1] {
        "AND" => Gate::AND,
        "OR" => Gate::OR,
        "XOR" => Gate::XOR,
        _ => panic!("bad gate {}", words[1]),
      };
      (gate, id(words[0]), id(words[2]), id(words[4]))
    })
    .collect::<Vec<_>>();

  let mut values = inputs.clone();
  loop {
    let mut more = false;
    for (g, a, b, o) in &gates {
      if values.contains_key(&o) {
        continue;
      }
      if let (Some(va), Some(vb)) =
        (values.get(&a), values.get(&b)) {
        let vo = match g {
          Gate::AND => va & vb,
          Gate::OR => va | vb,
          Gate::XOR => va ^ vb,
        };
        values.insert(*o, vo);
        more = true;
      }
    }
    if !more {
      break;
    }
  }
  
  let n = (0..)
    .map(|i| format!("z{i:02}"))
    .map_while(|l| idmap.get(&l))
    .map(|i| values[i])
    .enumerate()
    .fold(0, |n, (i, v)| n | ((v as usize) << i));
              
  println!("{n}");
}