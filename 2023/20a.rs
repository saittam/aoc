use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;

enum Module {
  FlipFlop(bool),
  Conjunction(u64),
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut idmap = HashMap::new();
  let mut next_id = 0;
  let mut id = |s: &str| *idmap.entry(s.to_owned())
    .or_insert_with(|| {
      next_id += 1;
      assert!(next_id < 64);
      next_id
  });

  let mut broadcast = Vec::new();
  let mut modules = lines
    .filter_map(|l| {
      let mut wi = l.split_whitespace();
      let name = wi.next().expect("name");
      assert_eq!(wi.next(), Some("->"));
      let dest = wi
        .map(|w| id(w.trim_end_matches(',')))
        .collect::<Vec<_>>();
      let mut ni = name.chars();
      let kind = match ni.next() {
        Some('%') => Module::FlipFlop(false),
        Some('&') => Module::Conjunction(!0),
        _ if name == "broadcaster" => {
          broadcast = dest;
          return None;
        }
        t => panic!("bad module type {:?}", t),
      };
      Some((id(ni.as_str()), (kind, dest)))
    })
    .collect::<HashMap<_, _>>();

  let conn = modules.iter()
    .flat_map(|(n, (_, d))| d.iter().map(move |d| (*n, *d)))
    .chain(broadcast.iter().map(|d| (0, *d)))
    .collect::<Vec<_>>();
  for (n, d) in conn {
    let module = modules.get_mut(&d);
    if let Some((Module::Conjunction(i), _)) = module {
      *i &= !(1 << n);
    }
  }

  let mut count = [0, 0];
  for _ in 0..1000 {
    let mut queue = broadcast.iter()
      .map(|n| (0, *n, false))
      .collect::<VecDeque<_>>();
    count[0] += 1;
    while let Some((s, d, p)) = queue.pop_front() {
      count[p as usize] += 1;
      
      let (kind, dest) = if let Some(m) = modules.get_mut(&d) {
        m
      } else {
        continue;
      };
      
      let dp = match kind {
        Module::FlipFlop(e) => if p {
          continue;
        } else {
          *e = !*e;
          *e
        }
        Module::Conjunction(i) => {
          *i = (*i & !(1 << s)) | ((p as u64) << s);
          *i != !0
        }
      };
      queue.extend(dest.iter().map(|n| (d, *n, dp)));
    }
  }

  println!("{}", count.iter().product::<usize>());
}
