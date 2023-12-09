use std::io::BufRead;

fn presents(cache: &mut Vec<usize>,
            n: usize) -> usize {
  if let Some(p) = cache.get(n - 1) {
    return p * 10;
  }

  assert_eq!(cache.len(), n - 1);
  for factor in 2.. {
    let mut div = n / factor;
    if div < factor {
      let result = 1 + n;
      cache.push(result);
      return result * 10;
    }
    if n % factor != 0 {
      continue;
    }
    let mut sum = 1 + factor;
    let mut prod = factor;
    while (div % factor) == 0 {
      div = div / factor;
      prod *= factor;
      sum += prod;
    }
    let result = sum * cache[div - 1];
    cache.push(result);
    return result * 10;
  }
  unreachable!()
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let n = lines.next().expect("input")
    .parse::<usize>().expect("number");

  let mut cache = vec![1];
  let k = (1..)
    .map(|n| (n, presents(&mut cache, n)))
    .find(|(_, p)| *p >= n)
    .unwrap().0;
  println!("{}", k);
}