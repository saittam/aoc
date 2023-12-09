use std::io::BufRead;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Pos {
  Floor,
  Empty,
  Occupied,
}

struct DirIter {
  pos: (usize, usize),
  dir: (isize, isize),
  end: (usize, usize),
}

impl Iterator for DirIter {
  type Item = (usize, usize);
  
  fn next(&mut self) -> Option<Self::Item> {
    if (self.pos.0 == self.end.0 && self.dir.0 != 0) ||
       (self.pos.1 == self.end.1 && self.dir.1 != 0) {
      None
    } else {
      self.pos = ((self.pos.0 as isize + self.dir.0) as usize,
                  (self.pos.1 as isize + self.dir.1) as usize);
      Some(self.pos)
    }
  }
}

#[derive(Default)]
struct Seat {
  occupied: bool,
  occupied_next: bool,
  visible: [Option<Rc<RefCell<Seat>>>; 8],
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let p = buf.trim().chars().map(
      |c| match c {
        '.' => Pos::Floor,
        'L' => Pos::Empty,
        _ => panic!("Bad sos: {}", c),
      }).collect::<Vec<Pos>>();
    m.push(p);
  }
  
  let cols = m.last().unwrap().len();
  let rows = m.len();
  
  let mut seats = Vec::new();
  let mut map = HashMap::new();
  for y in 0..rows {
    for x in 0..cols {
      if m[y][x] != Pos::Floor {
        let s: Seat = Default::default();
        let rc = Rc::new(RefCell::new(s));
        seats.push(Rc::clone(&rc));
        map.insert((x, y), rc);
      }
    }
  }
  
  for ((x, y), n) in &map {
    let dirs = [
      (( 1,  0), (cols, *y)),
      (( 1,  1), (cols, rows)),
      (( 0,  1), (*x, rows)),
      ((-1,  1), (0, rows)),
      ((-1,  0), (0, *y)),
      ((-1, -1), (0, 0)),
      (( 0, -1), (*x, 0)),
      (( 1, -1), (cols, 0)),
    ];

    for i in 0..8 {
      let (d, e) = dirs[i];
      (**n).borrow_mut().visible[i] = 
        DirIter { pos: (*x, *y), dir: d, end: e }
        .find_map(|p| map.get(&p))
        .map(Rc::clone);
    }
  }
        
  let mut changed = true;
  while changed {
    changed = false;
    
    for mut s in seats.iter().map(|r| r.borrow_mut()) {
      let vo = s.visible.iter()
        .filter_map(|s| s.as_ref())
        .filter(|s| s.borrow().occupied).count();
      s.occupied_next = match vo {
        0 => true,
        1 | 2 | 3 | 4 => s.occupied,
        5 | 6 | 7 | 8 => false,
        _ => unreachable!(),
      };
      changed |= s.occupied != s.occupied_next;
    }
    
    for mut s in seats.iter().map(|r| r.borrow_mut()) {
      s.occupied = s.occupied_next;
    }
  }
  
  let c = seats.iter().filter(
    |r| r.borrow_mut().occupied).count();
  
  println!("{}", c);
}