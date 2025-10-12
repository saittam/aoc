use std::io::BufRead;
use std::collections::HashMap;

fn count<'a, PadIter>(
  from: char,
  to: char,
  mut pads: PadIter,
  level: usize,
  cache: &mut HashMap<(char, char, usize), usize>)
  -> usize
where
  PadIter: Iterator<Item=&'a HashMap<char, (i32, i32)>>,
  PadIter: Clone
{
  let pad = if let Some(pad) = pads.next() {
    pad
  } else {
    return 1;
  };
  
  if let Some(n) = cache.get(&(from, to, level)) {
    return *n;
  }
  
  let gap = pad[&' '];
  let (ax, ay) = pad[&from];
  let (bx, by) = pad[&to];
  let ix =
    std::iter::repeat(if ax < bx { '>' } else { '<' })
    .take((bx - ax).abs() as usize);
  let iy =
    std::iter::repeat(if ay < by { 'v' } else { '^' })
    .take((by - ay).abs() as usize);

  let n = [
    ((ax, by), iy.clone().chain(ix.clone())),
    ((bx, ay), ix.clone().chain(iy.clone())),
  ].into_iter()
    .filter(|(turn, _)| *turn != gap)
    .map(|(_, i)| count_seq(i.chain(std::iter::once('A')),
                            pads.clone(), level + 1, cache))
    .min()
    .expect("min");

  cache.insert((from, to, level), n);
  n
}

fn count_seq<'a, SeqIter, PadIter>(
  i: SeqIter,
  pads: PadIter,
  level: usize,
  cache: &mut HashMap<(char, char, usize), usize>)
  -> usize
where
  SeqIter: Iterator<Item=char>,
  PadIter: Iterator<Item=&'a HashMap<char, (i32, i32)>>,
  PadIter: Clone
{
  i.scan('A', |p, c| {
    let e = (*p, c);
    *p = c;
    Some(e)
  })
  .map(|(from, to)|
       count(from, to, pads.clone(), level, cache))
  .sum::<usize>()
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let codes = lines.collect::<Vec<_>>();

  let npad = HashMap::from([
    ('0', (1, 3)),
    ('1', (0, 2)),
    ('2', (1, 2)),
    ('3', (2, 2)),
    ('4', (0, 1)),
    ('5', (1, 1)),
    ('6', (2, 1)),
    ('7', (0, 0)),
    ('8', (1, 0)),
    ('9', (2, 0)),
    ('A', (2, 3)),
    (' ', (0, 3)),
  ]);
  let dpad = HashMap::from([
    ('^', (1, 0)),
    ('A', (2, 0)),
    ('<', (0, 1)),
    ('v', (1, 1)),
    ('>', (2, 1)),
    (' ', (0, 0)),
  ]);

  let pads = std::iter::once(&npad).chain(
    std::iter::repeat(&dpad).take(2));
  let mut cache = HashMap::new();
  let n = codes.iter()
    .map(|code| {
      let len = count_seq(code.chars(), pads.clone(), 0,
                          &mut cache);
      let numeric = code.chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |n, d| 10 * n + d);
      len * numeric as usize
    })
    .sum::<usize>();
              
  println!("{n}");
}