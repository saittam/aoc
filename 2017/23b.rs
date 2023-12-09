use std::io::BufRead;

fn is_prime(n: usize) -> bool {
  for i in 2.. {
    if i * i > n {
      println!("{} is prime", n);
      return true;
    }
    if n % i == 0 {
      println!("{} is composite", n);
      return false;
    }
  }
  unreachable!()
}

fn main() {
  let count=
    (106700..=123700).step_by(17)
    .filter(|n| !is_prime(*n))
    .count();
  println!("{}", count);
}

/*

program is an inefficient (test all products) primality test
impractical to optimize mechanically so it runs in acceptable
time, at least I don't feel like implementing an optimizer 
that understands number theory ;-)

b = 67   // set b 67
c = 67   // set c b
if a != 0 {
  b = 106700
  c = 123700
}

while b != c {
  f = 1
  d = 2
  for d in 2..b {
    for e in 2..b {
      if d * e == b {
        f = 0
      }
    }
  }
  if f == 0 {
    h++
  }
  b += 17
}

*/