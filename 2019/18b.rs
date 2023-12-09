use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Clone, Copy, Debug)]
enum Tile {
  Wall,
  Empty,
  Key(char),
  Door(char),
}

type Grid = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
  Key(char),
  Door(char),
}

fn get(grid: &Grid, (x, y): (isize, isize)) -> Option<Tile> {
  if x >= 0 && y >= 0 {
    grid.get(y as usize).and_then(|r| r.get(x as usize)).cloned()
  } else {
    None
  }
}

fn reachable(grid: &Grid, p: (isize, isize)) -> Vec<(Node, usize)> {
  let mut q = VecDeque::new();
  let mut vis = HashSet::<(isize, isize)>::new();
  let mut res = Vec::<(Node, usize)>::new();
  q.push_back((p, 0));
  vis.insert(p);
  while let Some(((x, y), dist)) = q.pop_front() {
    let neighbors = [
      (x - 1, y),
      (x + 1, y),
      (x, y - 1),
      (x, y + 1),
    ];
    for pn in &neighbors {
      if vis.contains(pn) {
        continue;
      }
      vis.insert(*pn);
      
      match get(grid, *pn).unwrap() {
        Tile::Wall => continue,
        Tile::Empty => (),
        Tile::Key(k) => {
          res.push((Node::Key(k), dist + 1));
          continue;
        }
        Tile::Door(d) => {
          res.push((Node::Door(d), dist + 1));
          continue;
        }
      }
      
      q.push_back((*pn, dist + 1));
    }
  }
  
  res
}

fn step(dist: &HashMap<Node, Vec<(Node, usize)>>, start: char, keys: &[char]) -> Vec<(usize, char)> {
  let mut q = BinaryHeap::new();
  let mut vis = HashSet::new();
  let mut result = Vec::new();
  
  for (nn, nd) in dist.get(&Node::Key(start)).unwrap_or(&vec![]) {
    q.push(Reverse((*nd, *nn)));
  }
  
  while let Some(Reverse((d, n))) = q.pop() {
    if vis.contains(&n) {
      continue;
    }
    vis.insert(n);
    
    match n {
      Node::Door(d) => {
        if !keys.contains(&d.to_ascii_lowercase()) {
          continue;
        }
      }
      Node::Key(k) => {
        if !keys.contains(&k) {
          result.push((d, k));
          continue;
        }
      }
    }
            
    for (nn, nd) in dist.get(&n).unwrap_or(&vec![]) {
      q.push(Reverse((d + nd, *nn)));
    }
  }
  
  result
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut pc = None;
  let mut grid = Grid::new();
  let mut nodes = Vec::new();
  let mut nkeys = 0;
  for y in 0.. {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }

    let r = buf.trim().chars().enumerate().map(|(x, c)| match c {
      '#' => Tile::Wall,
      '.' => Tile::Empty,
      '@' => {
        pc = Some((x as isize, y as isize));
        Tile::Empty
      }
      k @ 'a'..='z' => {
        nodes.push((Node::Key(c), (x as isize, y as isize)));
        nkeys += 1;
        Tile::Key(k)
      }
      d @ 'A'..='Z' => {
        nodes.push((Node::Door(d), (x as isize, y as isize)));
        Tile::Door(d)
      }
      _ => panic!("tile {}", c),
    }).collect::<Vec<Tile>>();
    
    grid.push(r);
  }

  
  let (cxi, cyi) = pc.unwrap();
  let (cx, cy) = (cxi as usize, cyi as usize);
  
  grid[cy - 1][cx] = Tile::Wall;
  grid[cy][cx - 1] = Tile::Wall;
  grid[cy][cx] = Tile::Wall;
  grid[cy][cx + 1] = Tile::Wall;
  grid[cy + 1][cx] = Tile::Wall;
  
  let pos = vec![
    (cxi - 1, cyi - 1),
    (cxi - 1, cyi + 1),
    (cxi + 1, cyi - 1),
    (cxi + 1, cyi + 1),
  ];
  let mut bots = Vec::new();
  for (i, p) in pos.iter().enumerate() {
    let c = "0123456789".chars().nth(i).unwrap();
    nodes.push((Node::Key(c), *p));
    bots.push(c);
  }
  
  let mut dist = HashMap::new();
  for (n, p) in nodes {
    dist.insert(n, reachable(&grid, p));
    println!("{:?} -> {:?}", n, dist[&n]);
  }
  
  let mut q = BinaryHeap::new();
  q.push(Reverse((0, Vec::new(), bots)));
  println!("{:?}", q);
  
  let mut seen = HashSet::new();
  while let Some(Reverse((d, keys, bots))) = q.pop() {
    if keys.len() == nkeys {
      println!("{} {:?}", d, keys);
      break;
    }
    
    let mut ks = keys.clone();
    ks.sort();
    ks.extend(&bots);
    if seen.contains(&ks) {
      continue;
    }
    seen.insert(ks);
    
    for (i, bc) in bots.iter().enumerate() {
      //println!("stepping {} {:?}", i, keys);
      for (nd, nk) in step(&dist, *bc, &keys) {
        println!("{:?} -> {:?}", keys, (nd, nk));
        let mut ukeys: Vec<char> = keys.to_vec();
        ukeys.push(nk);
        let mut nbots = bots.clone();
        nbots[i] = nk; 
        q.push(Reverse((d + nd, ukeys, nbots)))
      }
    }
  }
}