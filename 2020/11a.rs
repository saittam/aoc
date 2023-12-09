use std::io::BufRead;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Pos {
  Floor,
  Empty,
  Occupied,
}

fn poschar(p: &Pos) -> char {
  match p {
    Pos::Floor => '.',
    Pos::Empty => 'L',
    Pos::Occupied => '#',
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  m.push(Vec::new());
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let p = std::iter::once(Pos::Floor).chain(
      buf.trim().chars().map(
      |c| match c {
        '.' => Pos::Floor,
        'L' => Pos::Empty,
        _ => panic!("Bad sos: {}", c),
      })).chain(std::iter::once(Pos::Floor))
      .collect::<Vec<Pos>>();
    m.push(p);
  }
  
  let cols = m.last().unwrap().len();
  m.push(vec![Pos::Floor; cols]);
  let rows = m.len();
  m[0] = vec![Pos::Floor; cols];
  
  let mut t = m.clone();
  let mut changed = true;
  let mut steps = 0;
  while changed {
    changed = false;
    
    for y in 1..(rows - 1) {
      for x in 1..(cols - 1) {
        if m[y][x] == Pos::Floor {
          continue;
        }
        
        let neigh = [
          (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
          (x - 1, y    ),             (x + 1, y    ),
          (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
        ];
        let c = neigh.iter().map(
          |(nx, ny)| (m[*ny][*nx] == Pos::Occupied) as u32)
          .sum();
        t[y][x] = match c {
          0 => Pos::Occupied,
          1 | 2 | 3 => m[y][x],
          4 | 5 | 6 | 7 | 8 => Pos::Empty,
          _ => unreachable!(),
        };
        changed |= m[y][x] != t[y][x];
      }
    }
    std::mem::swap(&mut m, &mut t);
    steps += 1;
    
    /*
    println!("{}", m.iter().map(
      |r| r.iter().map(poschar).collect::<String>())
      .collect::<Vec<String>>().join("\n"));
    */
  }
  
  let c: u32 = m.iter().map(|r| r.iter().map(
    |s| (*s == Pos::Occupied) as u32).sum::<u32>()).sum();
  
  println!("{}", c);
}