use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let list = lines
    .map(|w| w.parse::<isize>().expect("num"))
    .collect::<Vec<_>>();
  
  const KEY: isize = 811589153;

  let n = list.len();
  let ni1 = n as isize - 1;
  let min = list.iter().min().expect("min");
  let shift = ((min.abs() * KEY + ni1 - 1) / ni1) * ni1;
  let mut mix = (0..list.len()).collect::<Vec<_>>();
  for _ in 0..10 {
    for k in 0..n {
      let p = mix.iter().position(|v| *v == k).expect("p");
      let d = list[k] * KEY;
      let np = if d > 0 {
        1 + (p + d as usize - 1) % (n - 1)
      } else {
        (p as isize + d + shift) as usize % (n - 1)
      };
      if p <= np {
        mix.copy_within((p + 1)..=np, p);
      } else {
        mix.copy_within(np..p, np + 1);
      }
      mix[np] = k;
    }
  }

  let mixed = mix.iter()
    .map(|i| list[*i] * KEY)
    .collect::<Vec<_>>();
  let zero = mixed.iter().position(|v| *v == 0).expect("zero");
  let n = [1000, 2000, 3000].iter()
    .map(|k| mixed[(k + zero) % n])
    .sum::<isize>();
  println!("{n}");
}