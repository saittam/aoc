use std::io::BufRead;

struct Mem {
  m: Vec<isize>,
}

impl Mem {
  fn new(m: Vec<isize>) -> Mem {
    Mem {
      m: m,
    }
  }
}

impl std::ops::Index<isize> for Mem {
  type Output = isize;
  
  fn index(&self, index: isize) -> &isize {
    self.m.get(index as usize).unwrap_or(&0)
  }
}

impl std::ops::IndexMut<isize> for Mem {
  fn index_mut(&mut self, index: isize) -> &mut isize {
    self.m.get_mut(index as usize).unwrap()
  }
}


fn run(m: &mut Mem, input: &Vec<isize>) -> Vec<isize> {
  let mut ip = 0;
  let mut output = Vec::<isize>::new();
  let mut ini = input.iter();
  loop {
    //println!("{:?} {}", m[ip], ip);
    let a = m[ip + 1];
    let b = m[ip + 2];
    let d = m[ip + 3];
    ip += match m[ip] {
      01101 => { m[d] = a + b; 4 }
      01001 => { m[d] = m[a] + b; 4 }
      00101 => { m[d] = a + m[b]; 4 }
      00001 => { m[d] = m[a] + m[b]; 4 }
      01102 => { m[d] = a * b; 4 }
      01002 => { m[d] = m[a] * b; 4 }
      00102 => { m[d] = a * m[b]; 4 }
      00002 => { m[d] = m[a] * m[b]; 4 }
      3 => { m[a] = *ini.next().unwrap(); 2 }
      4 => { output.push(m[a]); 2 }
      104 => { output.push(a); 2 }
      99 => break,
      _ => panic!("invalid opcode {}", m[ip]),
    }
  }
  output
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let mut p = buf.split(',')
    .map(|s| s.trim().parse::<isize>().unwrap())
    .collect::<Vec<isize>>();

  let input = vec![ 1 ];
  let mut m = Mem::new(p.clone());
  let output = run(&mut m, &input);
  
  println!("{:?}", output);
}