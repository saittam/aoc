use std::io::Read;
use std::collections::BTreeMap;

fn main() {
  let stdin = std::io::stdin();
  let mut input = String::new();
  stdin.lock().read_to_string(&mut input).expect("input");

  let nums = input.chars().map(
    |c| c.to_digit(10).expect("digit"))
    .collect::<Vec<_>>();

  let mut alloc = nums.iter()
    .enumerate()
    .scan(0, |n, (i, l)| {
      let r = (i % 2 == 0).then_some((*n, (i / 2, *l)));
      *n += l;
      Some(r)
    })
    .flatten()
    .collect::<BTreeMap<_, _>>();

  let reverse_allocs = alloc.iter()
    .map(|(k, v)| (*k, *v))
    .rev()
    .collect::<Vec<_>>();
  let mut start = [0; 10];
  for (o, (i, l)) in reverse_allocs {
    let start = &mut start[l as usize];
    let no = alloc.range(*start..)
      .scan(*start, |p, (ro, (_, rl))| {
        let r = (*ro - *p >= l).then_some(*p);
        *p = ro + rl;
        Some(r)
      })
      .flatten()
      .next();
    if let Some(no) = no {
      if no < o {
        alloc.insert(no, (i, l));
        alloc.remove(&o);
        *start = no;
      }
    }
  }

  let n = alloc.iter().map(
    |(o, (i, l))| i * (*o..(o + l)).sum::<u32>() as usize)
    .sum::<usize>();
    
  println!("{n}");
}