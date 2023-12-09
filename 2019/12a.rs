use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut pos = Vec::<(isize, isize, isize)>::new();
    
  loop {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    //println!("buf: {}", buf);
    let v = buf.split(',')
      .map(|s| s.chars().filter(|c| c.is_digit(10) || *c == '-').collect::<String>())
      .map(|s| s.parse::<isize>().unwrap())
      .collect::<Vec<isize>>();
      
    pos.push((v[0], v[1], v[2]));
  }
  
  let mut vel = vec![(0isize, 0isize, 0isize); pos.len()];
  
  for i in 1..=1000 {
    for ((x, y, z), (vx, vy, vz)) in pos.iter().zip(vel.iter_mut()) {
      for (ox, oy, oz) in &pos {
        *vx += (ox - x).signum();
        *vy += (oy - y).signum();
        *vz += (oz - z).signum();
      }
    }
    
    for ((x, y, z), (vx, vy, vz)) in pos.iter_mut().zip(vel.iter()) {
      *x += vx;
      *y += vy;
      *z += vz;
    }
    
    /*
    println!("iter {}", i);
    for (p, v) in pos.iter().zip(vel.iter()) {
      println!("{:?} {:?}", p, v);
    }
    */
  }
  
  let norm = |(x, y, z): &(isize, isize, isize)| x.abs() + y.abs() + z.abs();
  let e: isize = pos.iter().map(norm).zip(vel.iter().map(norm))
    .map(|(p, k)| p * k).sum();
  println!("{}", e);
}