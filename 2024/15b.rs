use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
  Empty,
  Wall,
  Box(i32),
}

fn step(grid: &HashMap<(i32, i32), Cell>,
        updates: &mut HashMap<(i32, i32), Cell>,
        (x, y): (i32, i32),
        (dx, dy): (i32, i32),
        c: Cell) -> bool {
  let np = (x + dx, y + dy);
  let nc = *updates.get(&np).unwrap_or(&grid[&np]);
  updates.insert(np, c);
  match nc {
    Cell::Empty => true,
    Cell::Wall => false,
    Cell::Box(d) => {
      let twin = (np.0 + d, np.1);
      updates.insert(twin, Cell::Empty);
      step(grid, updates, twin, (dx, dy), Cell::Box(-d)) &&
      step(grid, updates, np, (dx, dy), Cell::Box(d))
    }
  }
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
        let y = y as i32;
        grid.extend(l.chars().enumerate().map(
          |(x, c)| ((x as i32, match c {
            '.' => [Cell::Empty; 2],
            '#' => [Cell::Wall; 2],
            'O' => [Cell::Box(1), Cell::Box(-1)],
            '@' => {
              start = Some((2 * x as i32, y as i32));
              [Cell::Empty; 2]
            },
            _ => panic!("bad cell {c}"),
          })))
          .flat_map(
            |(x, [c1, c2])|
            [((2 * x, y), c1), ((2 * x + 1, y), c2)]));
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
    .fold((grid, start), |(mut grid, mut pos), dir| {
      let mut updates = HashMap::new();
      if step(&grid, &mut updates, pos, *dir, Cell::Empty) {
        grid.extend(updates);
        pos = (pos.0 + dir.0, pos.1 + dir.1);
      }
      (grid, pos)
    });

  let n = grid.iter()
    .filter(|(_, c)| **c == Cell::Box(1))
    .map(|((x, y), _)| y * 100 + x)
    .sum::<i32>();
  
  println!("{n}");
}