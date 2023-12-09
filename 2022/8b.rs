use std::io::BufRead;

fn dist<'a, I>(i: I)
where
  I: Iterator<Item=&'a mut(usize, usize)>,
  I: std::iter::DoubleEndedIterator,
{
  let mut x = 0;
  let mut vis = vec![(0, usize::MAX, &mut x)];
  for (cp, (ch, cs)) in i.enumerate() {
    while let Some((vp, vh, vs)) = vis.last_mut() {
      let vh = *vh;
      let vp = *vp;
      if vh <= *ch {
        **vs *= cp - vp;
        vis.pop();
      }
      if vh >= *ch {
        *cs *= cp - vp;
        vis.push((cp, *ch, cs));
        break;
      }
    }
  }

  let n = vis.last().unwrap().0;
  while let Some((p, _, s)) = vis.pop() {
    *s *= n - p;
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut grid = lines
    .take_while(|l| l.len() > 0)
    .map(|l| l.chars()
              .map(|c| c.to_digit(10).expect("digit") as usize)
              .map(|h| (h, 1))
              .collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let _h = grid.len();
  let w = grid.first().expect("width").len();
  assert!(grid.iter().all(|r| r.len() == w));

  grid.iter_mut().for_each(|r| dist(r.iter_mut()));
  (0..w).for_each(
    |c| dist(grid.iter_mut().map(|r| &mut r[c])));

  //println!("{:?}", grid);

  let max = grid.iter().flatten()
    .map(|(_, v)| *v)
    .max().unwrap();
  println!("{}", max);
}