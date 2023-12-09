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

fn step(dist: &HashMap<Node, Vec<(Node, usize)>>, keys: &[char]) -> Vec<(usize, char)> {
  let start = keys.last().unwrap();
  let mut q = BinaryHeap::new();
  let mut vis = HashSet::new();
  let mut result = Vec::new();
  q.push(Reverse((0, Node::Key(*start))));
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
  
  let mut pos = None;
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
        pos = Some((x as isize, y as isize));
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
  
  let mut dist = HashMap::new();
  for (n, p) in nodes {
    dist.insert(n, reachable(&grid, p));
    //println!("{:?} -> {:?}", n, dist[&n]);
  }
  
  let mut q = BinaryHeap::new();
  for (n, d) in reachable(&grid, pos.unwrap()) {
    if let Node::Key(k) = n {
      q.push(Reverse((d, vec![k])));
    }
  }
  //println!("{:?}", q);
  
  let mut seen = HashSet::new();
  while let Some(Reverse((d, keys))) = q.pop() {
    if keys.len() == nkeys {
      println!("{} {:?}", d, keys);
      break;
    }
    
    let mut ks = keys.clone();
    let pl = ks.len() - 1;
    ks[0..pl].sort();
    if seen.contains(&ks) {
      continue;
    }
    seen.insert(ks);
    
    for (nd, nk) in step(&dist, &keys) {
      println!("{:?} -> {:?}", keys, (nd, nk));
      let mut nkeys = keys.clone();
      nkeys.push(nk);
      q.push(Reverse((d + nd, nkeys)))
    }
  }
}