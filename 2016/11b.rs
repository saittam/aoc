use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
  Chip(usize),
  Gen(usize),
}

impl Object {
  fn id(&self) -> usize {
    match self {
      Object::Chip(id) => *id,
      Object::Gen(id) => *id,
    }
  }

  fn mask(&self) -> (u32, u32) {
    match self {
      Object::Chip(id) => (1 << id, 0),
      Object::Gen(id) => (0, 1 << id),
    }
  }
}

impl std::ops::Index<Object> for Vec<(u32, u32)> {
  type Output = u32;
  fn index(&self, obj: Object) -> &u32 {
    match obj {
      Object::Chip(id) => &self[id].0,
      Object::Gen(id) => &self[id].1,
    }
  }
}

impl std::ops::IndexMut<Object> for Vec<(u32, u32)> {
  fn index_mut(&mut self, obj: Object) -> &mut u32 {
    match obj {
      Object::Chip(id) => &mut self[id].0,
      Object::Gen(id) => &mut self[id].1,
    }
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
  cur: u32,
  loc: Vec<(u32, u32)>,
}

impl State {
  fn floor_mask(&self, floor: u32) -> (u32, u32) {
    self.loc.iter().enumerate()
      .fold((0, 0), |(m, g), (i, (lm, lg))| {
          (m | (((*lm == floor) as u32) << i),
           g | (((*lg == floor) as u32) << i))
        })
  }
  
  fn step(&self, dest: u32, o1: Object, o2: Object)
    -> Option<State> {
    if self.loc[o1] != self.cur || self.loc[o2] != self.cur {
      return None;
    }

    if dest < 1 || dest > 4 {
      return None;
    }

    let (sm, sg) = self.floor_mask(self.cur);
    let (dm, dg) = self.floor_mask(dest);
    let (o1m, o1g) = o1.mask();
    let (o2m, o2g) = o2.mask();
    let snew = (sm & !o1m & !o2m, sg & !o1g & !o2g);
    let dnew = (dm | o1m | o2m, dg | o1g | o2g);
    let safe = |(m, g)| g == 0 || m & g == m;
    if !safe(snew) || !safe(dnew) {
      return None;
    }

    let mut loc = self.loc.clone();
    loc[o1] = dest;
    loc[o2] = dest;
    loc.sort();
    Some(State { cur: dest, loc })
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut elmap = HashMap::new();
  let mut nextel = 0;
  let mut idx = |w: &str| *elmap.entry(w.to_string())
    .or_insert_with(|| {
      nextel += 1;
      nextel - 1
    });

  let objects = lines
    .enumerate()
    .flat_map(|(floor, l)|
      l.split_whitespace()
        .map(|w| w.trim_matches(
          |c: char| !c.is_alphanumeric()))
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|w| {
          let kind = w[0].trim_end_matches("-compatible");
          Some(match w[1] {
            "microchip" => (Object::Chip(idx(kind)), floor),
            "generator" => (Object::Gen(idx(kind)), floor),
            _ => None?,
          })
        })
        .collect::<Vec<_>>()
    );

  let mut loc = Vec::new();
  for (obj, floor) in objects {
      loc.resize(usize::max(obj.id() + 1, loc.len()), (0, 0));
      loc[obj] = floor as u32 + 1;
  }

  loc.extend(&[(1, 1), (1, 1)]);
  loc.sort();

  let min = loc.iter()
    .map(|(ml, gl)| (4 - ml) * 2 + (4 - gl) * 2)
    .sum::<u32>() - 3 * 3;

  let obj = (0..loc.len())
    .flat_map(|i| [Object::Chip(i), Object::Gen(i)])
    .collect::<Vec<_>>();
  let mut obji = obj.iter();
  let choice_of_2 = obj.iter().flat_map(|o1| {
      let res = obji.clone().map(|o2| (*o1, *o2));
      obji.next();
      res
    })
    .collect::<Vec<_>>();

  let state = State { cur: 1, loc };
  let mut queue = BinaryHeap::new();
  queue.push((Reverse(min), 0, state));
  let mut seen = HashSet::new();
  while let Some((Reverse(min), n, state)) = queue.pop() {
    if !seen.insert(state.clone()) {
      continue;
    }

    if state.loc[0] == (4, 4) {
      println!("{}", n);
      break;
    }

    let cur = state.cur;
    for d in &[cur - 1, cur + 1] {
      queue.extend(choice_of_2.iter().filter_map(|(o1, o2)| {
        let inc = (*d > cur) != (*o1 != *o2);
        Some((Reverse(min + if inc { 2 } else { 0 }),
              n + 1,
              state.step(*d, *o1, *o2)?))
      }));
    }
  }
}