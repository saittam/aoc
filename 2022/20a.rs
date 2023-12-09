use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let list = lines
    .map(|w| w.parse::<isize>().expect("num"))
    .collect::<Vec<_>>();

  let n = list.len();
  let ni = n as isize;
  let mut mix = (0..list.len()).collect::<Vec<_>>();
  for k in 0..n {
    let mut p = mix.iter().position(|v| *v == k).expect("kp");
    let v = list[k];
    for _ in 0..v.abs() {
      let pn = (p + (ni + v.signum()) as usize) % n;
      mix.swap(p, pn);
      p = pn;
    }
  }

  let mixed = mix.iter().map(|i| list[*i]).collect::<Vec<_>>();
  let zero = mixed.iter().position(|v| *v == 0).expect("zero");
  let n = [1000, 2000, 3000].iter()
    .map(|k| mixed[(k + zero) % n])
    .sum::<isize>();
  println!("{n}");
}