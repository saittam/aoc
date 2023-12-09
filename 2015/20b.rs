use std::io::BufRead;

fn presents_infinite(cache: &mut Vec<usize>,
                     n: usize) -> usize {
  if let Some(p) = cache.get(n - 1) {
    return p * 11;
  }

  assert_eq!(cache.len(), n - 1);
  for factor in 2.. {
    let mut div = n / factor;
    if div < factor {
      let result = 1 + n;
      cache.push(result);
      return result * 11;
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
    return result * 11;
  }
  unreachable!()
}

fn presents_correct(n: usize) -> usize {
  let mut result = 11;
  for factor in 2..=n  {
    if n % factor == 0 && n <= factor * 50 {
      result += 11 * factor;
    }
  }
  result
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let n = lines.next().expect("input")
    .parse::<usize>().expect("number");
  
  // Idea: find houses that receive enough presents
  // disregarding the 50 houses limit, then recompute
  // correctly (and slowly) to filter.
  let mut cache = vec![1];
  let k = (1..)
    .filter(|k| presents_infinite(&mut cache, *k) >= n)
    .find(|k| presents_correct(*k) >= n)
    .unwrap();
  println!("{}", k);
}