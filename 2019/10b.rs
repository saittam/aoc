use std::io::BufRead;

fn get(m: &Vec<Vec<bool>>, x: isize, y: isize) -> bool {
  *m.get(y as usize).and_then(|v| v.get(x as usize)).unwrap_or(&false)
}

fn gcd(a: isize, b: isize) -> isize {
  let mut p = isize::min(a, b);
  let mut q = isize::max(a, b);
  
  while p != 0 {
    let tmp = q % p;
    q = p;
    p = tmp;
  }
  
  q
}

fn check(sx: isize, sy: isize, x: isize, y: isize, m: &Vec<Vec<bool>>, b: &mut Vec<Vec<bool>> ) -> usize {
  if !get(m, x, y) {
    return 0;
  }
  
  if get(b, x, y) {
    return 0;
  }
  
  let dx = x - sx;
  let dy = y - sy;
  let (ix, iy) = match (dx, dy) {
    (0, v) => (0, v.signum()),
    (v, 0) => (v.signum(), 0),
    _ => {
      let g = gcd(dx.abs(), dy.abs());
      (dx / g, dy / g)
    }
  };
  
  for s in 1.. {
    let mx = x + s * ix;
    let my = y + s * iy;
    if let Some(v) = b.get_mut(my as usize) {
      if let Some(p) = v.get_mut(mx as usize) {
        if get(m, mx, my) {
          *p = true;
        }
        continue;
      }
    }
    break;
  }
  
  1
}

fn reachable(m: &Vec<Vec<bool>>, sx: isize, sy: isize) -> (usize, Vec<Vec<bool>>) {
  let h = m.len() as isize;
  let w = m[0].len() as isize;
  
  let mut count = 0;
  let mut b = vec![vec![false; w as usize]; h as usize];
      
  //println!(">>> {},{} <<<", sx, sy);
  for o in 1..isize::max(w, h) {
    let lx = sx - o;
    let ux = sx + o;
    let ly = sy - o;
    let uy = sy + o;
        
    for x in lx..ux {
      count += check(sx, sy, x, ly, &m, &mut b);
      count += check(sx, sy, x + 1, uy, &m, &mut b);
    }
    for y in ly..uy {
      count += check(sx, sy, lx, y + 1, &m, &mut b);
      count += check(sx, sy, ux, y, &m, &mut b);
    }
  }
      
  (count, b)
}

fn anglecmp((x1, y1): &(isize, isize), (x2, y2): &(isize, isize)) -> std::cmp::Ordering {
  let q1 = (*x1 < 0) as usize * 2 + ((*x1 < 0) ^ (*y1 < 0)) as usize;
  let q2 = (*x2 < 0) as usize * 2 + ((*x2 < 0) ^ (*y2 < 0)) as usize;

  q1.cmp(&q2).then((y2 * x1).cmp(&(y1 * x2)))
}

fn show(m: &Vec<Vec<bool>>) {
  for r in m {
    println!("{}", r.iter().map(|v| if *v { 'O' } else { '_' }).collect::<String>());
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::<Vec<bool>>::new();
  loop {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }

    let data = buf.trim().chars()
      .map(|c| c == '#')
      .collect::<Vec<bool>>();
      
    m.push(data);
  }
  
  let h = m.len() as isize;
  let w = m[0].len() as isize;

  let (_count, bx, by) = (0..h).into_iter()
    .flat_map(|y| (0..w).into_iter().map(move |x| (x, y)))
    .filter(|(x, y)| get(&m, *x, *y))
    .map(|(x, y)| (reachable(&m, x, y).0, x, y))
    .max().unwrap();
    
  let mut b;
  let mut nuked = 0;
  loop {
    let (count, bt) = reachable(&m, bx, by);
    b = bt;
    if nuked + count >= 200 || count == 0 {
      break;
    }
    m = b;
    nuked += count;
  }
  
  let mut nl = (0..h).into_iter()
    .flat_map(|y| (0..w).into_iter().map(move |x| (x, y)))
    .filter(|(x, y)| get(&m, *x, *y) && !get(&b, *x, *y))
    .map(|(x,y)| (x - bx, by - y))
    .collect::<Vec<(isize, isize)>>();

  nl.sort_by(anglecmp);
  
  let (tx, ty) = nl[200 - nuked];
  println!("{}", (tx + bx) * 100 + by - ty);
}