use std::io::BufRead;

fn px(p: &(isize, isize, isize)) -> isize {
  p.0
}
fn py(p: &(isize, isize, isize)) -> isize {
  p.1
}
fn pz(p: &(isize, isize, isize)) -> isize {
  p.2
}

fn eq<I, I0, Item, P>(v: I, v0: I0, p: P) -> bool
where
  I: IntoIterator<Item = Item>,
  I0: IntoIterator<Item = Item>,
  P: Fn(Item) -> isize + Clone
{
  v.into_iter().map(p.clone()).zip(v0.into_iter().map(p)).all(
    |(vp, v0p)| vp == v0p)
}

fn gcd(ai: isize, bi: isize) -> isize {
  let mut a = ai;
  let mut b = bi;
  while b != 0 {
    let tmp = b;
    b = a % b;
    a = tmp;
  }
  a
}

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
  
  let pos0 = pos.clone();
  let vel0 = vel.clone();
  
  let mut cx = 0;
  let mut cy = 0;
  let mut cz = 0;
  
  for i in 1.. {
  
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
    
    if cx == 0 && eq(&pos, &pos0, px) && eq(&vel, &vel0, px) {
      cx = i;
    }
    if cy == 0 && eq(&pos, &pos0, py) && eq(&vel, &vel0, py) {
      cy = i;
    }
    if cz == 0 && eq(&pos, &pos0, pz) && eq(&vel, &vel0, pz) {
      cz = i;
    }
      
    if !( cx == 0 || cy == 0 || cz == 0 ) {
      break;
    }
  }
  
  //println!("{} {} {}", cx, cy, cz);
  
  let mxy = cx * cy / gcd(cx, cy);
  let mxyz = mxy * cz / gcd(mxy, cz);
  println!("{}", mxyz);
}