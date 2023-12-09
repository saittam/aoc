use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Tile {
  Wall,
  Empty,
  Space,
  Portal(String),
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut grid = Vec::new();
    
  loop {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    //println!("buf: {}", buf);
    let v = buf.chars()
      .map(|c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        ' ' => Tile::Space,
        p @ 'A'..='Z' => Tile::Portal(p.to_string()),
        _ => Tile::Space,
      }).collect::<Vec<Tile>>();

    grid.push(v);
  }
  
  let mut portals = HashMap::new();
    
  for (y, v) in grid.iter().enumerate() {
    for (x, tr) in v.windows(3).enumerate() {
      let p = match tr {
        [Tile::Empty, Tile::Portal(p1), Tile::Portal(p2)] =>
          Some(((x, y), format!("{}{}", p1, p2))),
        [Tile::Portal(p1), Tile::Portal(p2), Tile::Empty] =>
          Some(((x + 2, y), format!("{}{}", p1, p2))),
        _ => None,
      };
      if let Some((pp, l)) = p {
        portals.entry(l).or_insert_with(|| Vec::new()).push(pp);
      }
    }
  }
  
  for x in 0..grid.first().unwrap().len() {
    for (y, tr) in grid.iter().map(|v| v[x].clone()).collect::<Vec<Tile>>().windows(3).enumerate() {
      let p = match tr {
        [Tile::Empty, Tile::Portal(p1), Tile::Portal(p2)] =>
          Some(((x, y), format!("{}{}", p1, p2))),
        [Tile::Portal(p1), Tile::Portal(p2), Tile::Empty] =>
          Some(((x, y + 2), format!("{}{}", p1, p2))),
        _ => None,
      };
      if let Some((pp, l)) = p {
        portals.entry(l).or_insert_with(|| Vec::new()).push(pp);
      }
    }
  }

  for (l, v) in &portals {
    for (x, y) in v {
      grid[*y][*x] = Tile::Portal(l.clone());
    }
  }
  
  //println!("{:?}", portals);
  
  let start = portals["AA"].first().unwrap();
  let end = portals["ZZ"].first().unwrap();
  
  let mut q = std::collections::VecDeque::new();
  let mut vis = std::collections::HashSet::new();
  
  q.push_back((*start, 0));
  
  while let Some(((x, y), d)) = q.pop_front() { 
    if vis.contains(&(x, y)) {
      continue;
    }
    vis.insert((x, y));
    
    //println!("{},{} {}", x, y, d);
   
    if (x, y) == *end {
      println!("{}", d);
      break;
    }

    let neighbors = [
      (x - 1, y),
      (x + 1, y),
      (x, y - 1),
      (x, y + 1),
    ];
    for (nx, ny) in &neighbors {
      match grid[*ny][*nx] {
        Tile::Wall | Tile::Space => (),
        Tile::Empty => q.push_back(((*nx, *ny), d + 1)),
        Tile::Portal(ref l) => {
          if let Some(pd) = portals.get(l) {
            q.push_back(((*nx, *ny), d + 1));
            for p in pd {
              q.push_back((*p, d + 2));
            }
          }
        }
      }
    }
  }
}
  