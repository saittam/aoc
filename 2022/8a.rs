use std::io::BufRead;

fn visible<'a, I>(mut i: I)
where
  I: Iterator<Item=&'a mut(usize, bool)>,
  I: std::iter::DoubleEndedIterator,
{
  let mut hf = 0;
  let mut hb = 0;
  loop {
    let (hc, n) = if hf > hb {
      (&mut hb, i.next_back())
    } else {
      (&mut hf, i.next())
    };
    if let Some((t, v)) = n {
      let th = *t + 1;
      *v |= th > *hc;
      *hc = std::cmp::max(*hc, th);
    } else {
      break;
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut grid = lines
    .take_while(|l| l.len() > 0)
    .map(|l| l.chars()
              .map(|c| c.to_digit(10).expect("digit") as usize)
              .map(|h| (h, false))
              .collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let _h = grid.len();
  let w = grid.first().expect("width").len();
  assert!(grid.iter().all(|r| r.len() == w));

  grid.iter_mut().for_each(|r| visible(r.iter_mut()));
  (0..w).for_each(
    |c| visible(grid.iter_mut().map(|r| &mut r[c])));

  //println!("{:?}", grid);

  let total = grid.iter().flatten()
    .filter(|(_, v)| *v)
    .count();
  println!("{}", total);
}