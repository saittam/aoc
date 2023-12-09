use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let lo = lines.by_ref()
    .take_while(|l| l.len() > 0)
    .collect::<Vec<_>>();
  let mut li = lo.iter().rev().skip(1)
    .map(|l| l.chars().skip(1).step_by(4))
    .collect::<Vec<_>>();
  let mut stacks = std::iter::repeat_with(
      || li.iter_mut()
           .filter_map(Iterator::next)
           .filter(|c| *c != ' ')
           .collect::<Vec<_>>())
    .take_while(|s| s.len() > 0)
    .collect::<Vec<_>>();

  for l in lines.take_while(|l| l.len() > 0) {
    let mut ni = l.split(' ').skip(1).step_by(2)
      .map(|n| n.parse::<usize>().expect("bad input"));
    let n = ni.next().expect("count");
    let from = ni.next().expect("from");
    let to = ni.next().expect("to");
    let fs = &mut stacks[from - 1];
    let t = fs.split_off(fs.len() - n);
    stacks[to - 1].extend(t);
  }

  let r = stacks.iter()
    .filter_map(|s| s.last())
    .collect::<String>();
  println!("{}", r);
}