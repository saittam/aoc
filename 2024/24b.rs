use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug)]
enum Gate {
  AND,
  OR,
  XOR,
}

type Id = usize;
type GateMap = HashMap<Id, (Gate, Id, Id)>;
type SigSet = HashSet<Id>;
type Assignment = Vec<Option<bool>>;

fn eval(
  signal: Id,
  gates: &GateMap,
  values: &mut Assignment,
  seen: &mut SigSet,
) -> Option<bool> {
  if let Some(val) = values[signal] {
    return Some(val);
  }

  // cycle detection
  if !seen.insert(signal) {
    return None;
  }

  let (g, a, b) = gates.get(&signal)?;
  let va = eval(*a, gates, values, seen)?;
  let vb = eval(*b, gates, values, seen)?;
  let v = match g {
    Gate::AND => va & vb,
    Gate::OR => va | vb,
    Gate::XOR => va ^ vb,
  };
  values[signal] = Some(v);
  Some(v)
}

fn check(
  inx: Id,
  iny: Id,
  out: Id,
  gates: &GateMap,
  values: &Assignment,
  carry: bool,
) -> bool {
  [(0, 0), (0, 1), (1, 0), (1, 1)].iter().all(|(vx, vy)| {
    let (vx, vy) = (*vx != 0, *vy != 0);
    let exp = vx ^ vy ^ carry;
    let mut values = values.clone();
    values[inx] = Some(vx);
    values[iny] = Some(vy);
    let mut seen = SigSet::new();
    Some(exp) == eval(out, gates, &mut values, &mut seen)
  })
}

fn swap(gates: &mut GateMap, a: Id, b: Id) {
  if a != b {
    let va = gates.remove(&a).expect("swap a");
    let vb = gates.insert(b, va).expect("swap b");
    gates.insert(a, vb);
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut idmap = HashMap::new();
  let mut id = |s: &str| {
    let i = idmap.len();
    *idmap.entry(s.to_string()).or_insert(i)
  };

  lines
    .by_ref()
    .take_while(|l| !l.is_empty())
    .for_each(drop);

  let mut gates = lines
    .map(|l| {
      let words = l.split_whitespace().collect::<Vec<_>>();
      let gate = match words[1] {
        "AND" => Gate::AND,
        "OR" => Gate::OR,
        "XOR" => Gate::XOR,
        _ => panic!("bad gate {}", words[1]),
      };
      (id(words[4]), (gate, id(words[0]), id(words[2])))
    })
    .collect::<HashMap<_, _>>();

  let getid = |c, i| idmap.get(&format!("{c}{i:02}"));
  let mut swapped = Vec::new();
  let mut all1 = vec![None; idmap.len()];
  let mut all0 = vec![None; idmap.len()];

  let count =
    (0..).find(|b| getid('x', *b).is_none()).unwrap();
  for bit in 0..count {
    let inx = *getid('x', bit).expect("inx");
    let iny = *getid('y', bit).expect("iny");
    let out = *getid('z', bit).expect("out");

    if !check(inx, iny, out, &gates, &all1, bit > 0)
      || !check(inx, iny, out, &gates, &all0, false)
    {
      // Choice of gates to swap:
      // * can't chooes gates that control lower bits
      // * must choose a gate affecting the output (fanin)
      let mut fanin = SigSet::new();
      eval(out, &gates, &mut all0.clone(), &mut fanin);
      fanin.retain(|s| gates.contains_key(s));
      let pending = gates
        .keys()
        .copied()
        .filter(|i| all0[*i].is_none())
        .filter(|i| !fanin.contains(i))
        .collect::<Vec<_>>();
      let fanin_candidates = fanin
        .iter()
        .flat_map(|a| fanin.iter().map(move |b| (*a, *b)))
        .filter(|(a, b)| a < b);
      let pending_candidates = pending
        .iter()
        .flat_map(|a| fanin.iter().map(move |b| (*a, *b)));

      // This assumes that there is always a unique swap
      // to fix the current bit. Thst's technically
      // incorrect: There could be multiple options, or
      // multiple swaps could be necessary. More elaborate
      // searching would be required in these situatuons.
      let (a, b) = fanin_candidates
        .chain(pending_candidates)
        .find(|(a, b)| {
          swap(&mut gates, *a, *b);
          let r =
            check(inx, iny, out, &gates, &all1, bit > 0)
              && check(inx, iny, out, &gates, &all0, false);
          swap(&mut gates, *a, *b);
          r
        })
        .expect(&format!(
          "no single swap to fix bit {bit}"
        ));
      swapped.extend([a, b]);
      swap(&mut gates, a, b);
    }

    all0[inx] = Some(false);
    all0[iny] = Some(false);
    eval(out, &gates, &mut all0, &mut SigSet::new());

    all1[inx] = Some(true);
    all1[iny] = Some(true);
    eval(out, &gates, &mut all1, &mut SigSet::new());
  }

  // Check carry. A swap could be required to fix it,
  // but that's not the case with my input.
  let car = *getid('z', count).expect("car");
  if eval(car, &gates, &mut all1, &mut SigSet::new())
    != Some(true)
    || eval(car, &gates, &mut all0, &mut SigSet::new())
      != Some(false)
  {
    panic!("final carry incorrect");
  }

  let mut swapped = swapped
    .iter()
    .map(|i| {
      idmap
        .iter()
        .find_map(|(k, v)| (i == v).then(|| k))
        .expect("signal name")
        .to_string()
    })
    .collect::<Vec<_>>();
  swapped.sort();
  println!("{}", swapped.join(","));
}
