use std::io::BufRead;
    
fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines.next().expect("input")
    .parse::<usize>().expect("number");

  let (s, _) = (1..)
    .scan((0, n), |(start, count), k| {
      let step = 1 << k;
      *start =
        if *count % 2 == 0 { *start } else { *start + step };
      *count /= 2;
      Some((*start, *count))
    })
    .find(|(_, count)| *count == 1)
    .unwrap();
  
  println!("{}", s + 1)
}