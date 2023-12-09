use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);

  let data = buf.trim().chars()
    .map(|c| c.to_digit(10).unwrap() as usize)
    .collect::<Vec<usize>>();
  
  let mut img = vec![2; 25 * 6];  
  for l in data.chunks(25 * 6).rev() {
    for i in 0..(25 * 6) {
      img[i] = match l[i] {
        0 => 0,
        1 => 1,
        _ => img[i],
      };
    }
  }

  for r in img.chunks(25) {
    let l = r.iter()
      .map(|p| match *p {
        0 => ' ',
        1 => '#',
        _ => ' ',
      })
      .collect::<String>();
    println!("{}", l);
  }
}