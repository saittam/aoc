use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
  id: u32,
  edges: [u32; 4],
  rot: usize,
  flip: bool,
}

fn tob<'a, I: Iterator<Item=&'a bool>>(x: I) -> u32 {
  x.fold(0, |a, d| (a << 1) | *d as u32)
}

fn reverse_bits(v: u32, s: usize) -> u32 {
  let mut r = 0;
  for i in 0..(s / 2) {
    r |= (v & (1 << i)) << (s - 2 * i - 1);
    r |= (v & (1 << (s - i - 1))) >> (s - 2 * i - 1);
  }
  r
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines()
    .map(|r| r.unwrap());
  
  let mut tiles = Vec::new();
  loop {
    let mut iter = lines.by_ref().take_while(|l| l.len() > 0);
    let id = match iter.next() {
      Some(l) => l.chars().filter(|c| c.is_digit(10))
        .fold(0, |a, d| a * 10 + d.to_digit(10).unwrap()),
      None => break,
    };
    let tile = iter
      .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
      .collect::<Vec<Vec<bool>>>();
    let n = tile.len();
    
    let edges = [
      tob(tile.first().unwrap().iter()),
      tob(tile.iter().map(|r| r.last().unwrap())),
      tob(tile.last().unwrap().iter().rev()),
      tob(tile.iter().map(|r| r.first().unwrap()).rev()),
    ];
    let flip = [
      reverse_bits(edges[2], n),
      reverse_bits(edges[1], n),
      reverse_bits(edges[0], n),
      reverse_bits(edges[3], n),
    ];
    for (e, f) in &[(edges, false), (flip, true)] {
      for i in 0..4 {
        let er = [
          e[(0 + i) % 4],
          e[(1 + i) % 4],
          reverse_bits(e[(2 + i) % 4], n),
          reverse_bits(e[(3 + i) % 4], n),
        ];
        tiles.push(Tile { id, edges: er, rot: i, flip: *f });
      }
    }
  }
  
  let mut counts = HashMap::new();
  for tile in &tiles {
    for e in &tile.edges {
      *counts.entry(*e).or_insert(0) += 1;
    }
  }
  tiles.sort_by_key(|t| t.edges.iter().map(|e| counts[e]).sum::<usize>());
  
  let dim = ((tiles.len() / 8) as f64).sqrt() as usize;
  let mut sel = Vec::new();
  sel.push(0);
'o:
  loop {
    let si = sel.len() - 1;
    let st = &tiles[*sel.last().unwrap()];
    let e = &st.edges;
    if ((si % dim) == 0 ||
        tiles[sel[si - 1]].edges[1] == e[3]) &&
       (si < dim ||
        tiles[sel[si - dim]].edges[2] == e[0]) &&
       sel[0..si].iter()
         .find(|t| tiles[**t].id == st.id).is_none() {
      if sel.len() == tiles.len() / 8 {
        let p = [0, dim - 1, sel.len() - dim, sel.len() - 1]
          .iter().map(|i| tiles[sel[*i]].id as u64)
          .product::<u64>();        
        println!("{}", p);
        *sel.last_mut().unwrap() += 1;
        break 'o;
      } else {
        sel.push(0);
      }
    } else {
      *sel.last_mut().unwrap() += 1;
    }
    
    while *sel.last().unwrap() >= tiles.len() {
      sel.pop();
      if sel.len() == 0 {
        break 'o;
      }
      *sel.last_mut().unwrap() += 1;
    }
  }
}