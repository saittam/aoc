use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);

  let ml = buf.trim().chars()
    .map(|c| c.to_digit(10).unwrap() as usize)
    .collect::<Vec<usize>>()
    .chunks(25 * 6)
    .map(|c| c.iter().fold(vec![0; 3],
      |mut v, d| { if let Some(n) = v.get_mut(*d) { *n += 1; }; v }))
    .min().unwrap();

  println!("{}", ml[1] * ml[2]);
}