use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
  id: u32,
  edges: [u32; 4],
  orient: usize,
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
  
  let mut tiledata = HashMap::new();
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
    for (k, e) in [edges, flip].iter().enumerate() {
      for rot in 0..4 {
        let edges = [
          e[(0 + rot) % 4],
          e[(1 + rot) % 4],
          reverse_bits(e[(2 + rot) % 4], n),
          reverse_bits(e[(3 + rot) % 4], n),
        ];
        let orient = 4 * k + (4 - rot) % 4;
        tiles.push(Tile { id, edges, orient });
      }
    }
    
    tiledata.insert(
      id,
      tile[1..(tile.len() - 1)].into_iter().fold(
        Vec::new(),
        |mut a, v| { a.extend(&v[1..(v.len() - 1)]); a }));
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
        break 'o;
        *sel.last_mut().unwrap() += 1;
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
  
  assert!(sel.len() == tiles.len() / 8);

  let tdim = (tiledata.values().next().unwrap().len() as f64).sqrt() as isize;
  let mdim = (tdim as usize) * dim;
  let mut map = vec![false; mdim * mdim];
  for (i, t) in sel.iter().map(|i| &tiles[*i]).enumerate() {
    let data = &tiledata[&t.id];
    let step = [
      (0, 1, 0),
      (tdim * (tdim - 1), -tdim, tdim * tdim + 1),
      (tdim * tdim - 1, -1, 0),
      (tdim - 1, tdim, -tdim * tdim - 1),
      
      (tdim * (tdim - 1), 1, -2 * tdim),
      (0, tdim, -tdim * tdim + 1),
      (tdim - 1, -1, 2 * tdim),
      (tdim * tdim - 1, -tdim, tdim * tdim - 1),
    ];
    let (mut p, dx, dy) = step[t.orient];
    let oy = (i / dim) * (tdim as usize);
    let ox = (i % dim) * (tdim as usize);
    for y in 0..(tdim as usize) {
      for x in 0..(tdim as usize) {
        map[(oy + y) * mdim + ox + x] = data[p as usize];
        p += dx;
      }
      p += dy;
    }
  }
  
  let seamonster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
  let sp = seamonster.split('\n').enumerate()
    .flat_map(|(y, l)| l.chars().enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| (x as isize,
                                            y as isize)))
    .collect::<Vec<_>>();
  let sw = *sp.iter().map(|(x, _)| x).max().unwrap() + 1;
  let sh = *sp.iter().map(|(_, y)| y).max().unwrap() + 1;
  
  let trans = [
    ((0, 0), (1, 0), (0, 1)),
    ((sh - 1, 0), (0, -1), (1, 0)),
    ((sw - 1, sh - 1), (-1, 0), (0, -1)),
    ((0, sw - 1), (0, 1), (-1, 0)),
    ((0, sh - 1), (1, 0), (0, -1)),
    ((0, 0), (0, 1), (1, 0)),
    ((sw - 1, 0), (-1, 0), (0, 1)),
    ((sh - 1, sw - 1), (0, -1), (-1, 0)),
  ];

  let wavec = map.iter().filter(|x| **x).count();
  for (i, ((ox, oy), (cxx, cxy), (cyx, cyy)))
      in trans.iter().enumerate() {
    let spt = sp.iter()
      .map(|(x, y)| (ox + cxx * x + cxy * y,
                     oy + cyx * x + cyy * y))
      .collect::<Vec<_>>();
    
    let (tw, th) = if i % 2 == 0 { (sw, sh) } else { (sh, sw) };
    
    let mref = &map;
    let sptref = &spt;
    let sc = (0..(mdim as isize - th)).flat_map(
      move |y| (0..(mdim as isize - tw)).map(
        move |x| (x, y)))
      .filter(|(x, y)| sptref.iter().all(
          |(sx, sy)| mref[
            ((y + sy) * mdim as isize + x + sx) as usize]))
      .collect::<Vec<_>>();
      
    if sc.len() > 0 {
      println!("{}", wavec - sc.len() * sp.len());
    }
  }
}