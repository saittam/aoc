use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
  Air,
  Rock,
  Sand,
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut map = HashMap::new();
  for l in lines {
    let numi = l.split(|c: char| !c.is_digit(10))
                .filter(|w| w.len() > 0)
                .map(|n| n.parse::<isize>().expect("num"));
    let xli = numi.clone().step_by(2);
    let yli = numi.clone().skip(1).step_by(2);
    let xhi = xli.clone().skip(1);
    let yhi = yli.clone().skip(1);
    let linei = xli.zip(yli).zip(xhi.zip(yhi));
    for ((xl, yl), (xh, yh)) in linei {
      let r = |l, h| if l < h { l..=h } else { h..=l };
      let pi = r(xl, xh)
        .flat_map(|x| r(yl, yh).map(move |y| (x, y)));
      for p in pi {
        map.insert(p, Tile::Rock);
      }
    }
  }

  let ymax = *map.keys().map(|(_, y)| y).max().expect("max");

  while map.get(&(500, 0)) != Some(&Tile::Sand) {
    let (mut x, mut y) = (500, 0);
    loop {
      if y == ymax + 1 {
        map.insert((x, y), Tile::Sand);
        break;
      }
      let n = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
      let p = n.iter().find(
        |n| *map.get(&n).unwrap_or(&Tile::Air) == Tile::Air);
      if let Some((px, py)) = p {
        x = *px;
        y = *py;
      } else {
        map.insert((x, y), Tile::Sand);
        break;
      }
    }
  }

  let r = map.values().filter(|t| **t == Tile::Sand).count();
  println!("{}", r);
}