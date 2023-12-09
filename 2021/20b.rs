use std::io::BufRead;
 
fn apply(img: &Vec<Vec<bool>>, lut: &Vec<bool>, def: bool)
  -> Vec<Vec<bool>> {
  let EMPTY: Vec<bool> = vec![];
  let mut res = Vec::new();
  for y in -1..=(img.len() as isize) {
    let mut r = Vec::new();
    for x in -1..=(img[0].len() as isize) {
      let neigh = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),     (x, y),     (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
      ];
      
      let k = neigh.iter()
        .map(|(nx, ny)|
             img.get(*ny as usize).unwrap_or(&EMPTY)
                .get(*nx as usize).unwrap_or(&def))
        .fold(0, |a, d| (a << 1) | *d as usize);
      r.push(lut[k]);
    }
    res.push(r);
  }
    
  res
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let lut = lines.by_ref().take_while(|l| l.len() > 0)
    .collect::<Vec<_>>().iter()
    .flat_map(|l| l.chars().map(|c| c == '#'))
    .collect::<Vec<_>>();
  let img = lines.take_while(|l| l.len() > 0)
    .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
    .collect::<Vec<_>>();
    
  let n = (0..50)
    .fold(img, |i, k| apply(&i, &lut, (k % 2 == 1) && lut[0]))
    .iter()
    .map(|r| r.iter().filter(|p| **p).count())
    .sum::<usize>();
  
  println!("{}", n);
}