use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = lines
    .map(|l| l.parse::<i32>().expect("num"))
    .collect::<Vec<_>>();

  let mut p = 0;
  let n = std::iter::from_fn(|| {
      let j = nums.get_mut(usize::try_from(p).ok()?)?;
      p += *j;
      *j = if *j >= 3 { *j - 1 } else { *j + 1 };
      Some(())
    })
    .count();

  println!("{}", n);
}
