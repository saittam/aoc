use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
  Empty,
  Wall,
  Box,
}

fn step(grid: &mut HashMap<(i32, i32), Cell>,
        (x, y): (i32, i32),
        (dx, dy): (i32, i32),
        c: Cell) -> (i32, i32) {
  let np = (x + dx, y + dy);
  let rp = match grid[&np] {
    Cell::Empty => np,
    Cell::Wall => (x, y),
    Cell::Box => {
      let (nx, ny) = step(grid, np, (dx, dy), Cell::Box);
      (nx - dx, ny - dy)
    }
  };
  grid.insert(rp, c);
  rp
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let (grid, start) = lines.by_ref()
    .take_while(|l| !l.is_empty())
    .enumerate()
    .fold(
      (HashMap::new(), None),
      |(mut grid, mut start), (y, l)| {
        grid.extend(l.chars().enumerate().map(
          |(x, c)| ((x as i32, y as i32), match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            'O' => Cell::Box,
            '@' => {
              start = Some((x as i32, y as i32));
              Cell::Empty
            },
            _ => panic!("bad cell {c}"),
          })));
        (grid, start)
      });
  let start = start.expect("start");

  let path = lines
    .fold(Vec::new(), |mut path, l| {
      path.extend(l.chars().map(
        |c| match c {
          '>' => (1, 0),
          'v' => (0, 1),
          '<' => (-1, 0),
          '^' => (0, -1),
          _ => panic!("bad dir {c}"),
        }));
      path
    });

  let (grid, _) = path.iter()
    .fold((grid, start), |(mut grid, pos), dir| {
      let pos = step(&mut grid, pos, *dir, Cell::Empty);
      (grid, pos)
    });

  let n = grid.iter()
    .filter(|(_, c)| **c == Cell::Box)
    .map(|((x, y), _)| y * 100 + x)
    .sum::<i32>();
  
  println!("{n}");
}