use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Reverse;
//use std::collections::hash_map::Entry;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
  Up = -1,
  Down = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
  Wall,
  Empty,
  Space,
  Portal(Dir, [char; 2]),
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut grid = Vec::new();
  let mut labels = HashMap::new();

  for y in 0.. {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    //println!("buf: {}", buf);
    let v = buf.chars().enumerate()
      .map(|(x, c)| match c {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        ' ' => Tile::Space,
        p @ 'A'..='Z' => {
          labels.insert((x, y), p);
          Tile::Space
        },
        _ => Tile::Space,
      }).collect::<Vec<Tile>>();

    grid.push(v);
  }
  
  let mut labels2 = HashMap::<(usize, usize), [char; 2]>::new();
  for ((x, y), l) in &labels {
    if let Some(l2) = labels.get(&(*x + 1, *y)) {
      labels2.insert((*x, *y), [*l, *l2]);
      labels2.insert((*x + 1, *y), [*l, *l2]);
    }
    if let Some(l2) = labels.get(&(*x, *y + 1)) {
      labels2.insert((*x, *y), [*l, *l2]);
      labels2.insert((*x, *y + 1), [*l, *l2]);
    }
  }

  for ((x, y), l) in labels2 {
    
    let neighbors: [(usize, usize); 4] = [
      (x.overflowing_sub(1).0, y),
      (x + 1, y),
      (x, y.overflowing_sub(1).0),
      (x, y + 1),
    ];
    
    for (nx, ny) in &neighbors {
      if *grid.get(*ny).and_then(|v| v.get(*nx)).unwrap_or(&Tile::Space) == Tile::Empty {
        let dir = if *nx > 2 && *ny > 2 && *ny < grid.len() - 3 &&
            *nx < grid.first().unwrap().len() - 4 {
          Dir::Down
        } else {
          Dir::Up
        };
        
        //println!("portal {:?} {:?} at {},{}", l, dir, *nx, *ny);
        
        grid[*ny][*nx] = Tile::Portal(dir, l);
      }
    }
  }

  let mut edges = HashMap::new();
  for (p, d, l) in grid.iter().enumerate().flat_map(
    |(y, v)| v.iter().enumerate().map(
      move |(x, t)| ((x, y), t))).filter_map(
    |(p, t)| if let Tile::Portal(d, l) = t { Some((p, d, l)) } else { None }) {
    edges.insert((*d, *l), reachable(&grid, p));
  }
  edges.insert((Dir::Down, ['A', 'A']), vec![]);
  edges.insert((Dir::Down, ['Z', 'Z']), vec![]);
  
  for e in &edges {
    println!("{:?}", e);
  }
  
  let mut q = std::collections::BinaryHeap::new();
  let mut vis = std::collections::HashSet::new();
  q.push(Reverse((0, (Dir::Up, ['A', 'A'], 0))));
  vis.insert((Dir::Up, ['A', 'A'], 0));
  
'outer:
  while let Some(Reverse((dist, (d, l, lvl)))) = q.pop() {
    //println!("{} {:?} {:?} {}", dist, d, l, lvl);
    
    for (rdist, (rd, rl)) in &edges[&(d, l)] {
      if *rl == ['Z', 'Z'] && lvl == 0 {
        println!("{}", dist + rdist);
        break 'outer;
      }
    
      let dinv = match rd {
        Dir::Up => Dir::Down,
        Dir::Down => Dir::Up,
      };
      
      if lvl == 0 && *rd == Dir::Up {
        continue;
      }
      
      let neigh = (dinv, *rl, lvl + *rd as isize);
      if vis.contains(&neigh) {
        continue;
      }
      vis.insert(neigh);
      q.push(Reverse((dist + 1 + rdist, neigh)));
    }
  }
}

fn reachable(grid: &Vec<Vec<Tile>>, start: (usize, usize)) -> Vec<(usize, (Dir, [char; 2]))> {
  let mut result = Vec::new();
  let mut q = std::collections::VecDeque::new();
  let mut vis = std::collections::HashSet::new();
  
  q.push_back((start, 0));
  vis.insert(start);
  
  while let Some(((x, y), d)) = q.pop_front() { 
    
    //println!("{},{} {} {}", x, y, lvl, d);
   
    let neighbors = [
      (x - 1, y),
      (x + 1, y),
      (x, y - 1),
      (x, y + 1),
    ];
    for (nx, ny) in &neighbors {
      if vis.contains(&(*nx, *ny)) {
        continue;
      }
      vis.insert((*nx, *ny));
    
      match grid[*ny][*nx] {
        Tile::Wall | Tile::Space => (),
        Tile::Empty => q.push_back(((*nx, *ny), d + 1)),
        Tile::Portal(pd, pl) => {
          q.push_back(((*nx, *ny), d + 1));
          result.push((d + 1, (pd, pl)));
        }
      }
    }
  }
  
  result
}
  