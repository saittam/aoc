use std::io::BufRead;
use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Debug)]
enum Rule {
  Literal(char),
  Ref(u32),
  Seq(Vec<Rule>),
  Choice(Box<Rule>, Box<Rule>),
}

fn parse<'a, I>(i: &mut I, mut v: Vec<Rule>) -> Rule 
where I: Iterator<Item=&'a str> {
  let b = |r| Box::new(r);
  if let Some(s) = i.next() {
    let e = match s.chars().next().unwrap() {
      '|' => return Rule::Choice(b(Rule::Seq(v)),
                                 b(parse(i, Vec::new()))),
      '"' => Rule::Literal(s.chars().nth(1).unwrap()),
      c if c.is_digit(10) => Rule::Ref(s.parse::<u32>().unwrap()),
      _ => panic!("bad token {}", s),
    };
    v.push(e);
    parse(i, v)
  } else {
    Rule::Seq(v)
  }
}

#[derive(Debug)]
struct RuleChain<'a> {
  head: &'a [Rule],
  tail: Option<&'a RuleChain<'a>>,
}

fn chain<'a>(head: &'a [Rule],
             tail: Option<&'a RuleChain>) -> RuleChain<'a> {
  RuleChain{ head, tail }
}

fn chainr<'a>(head: &'a Rule,
              tail: Option<&'a RuleChain>) -> RuleChain<'a> {
  RuleChain{ head: std::slice::from_ref(head), tail }
}
  
fn matchr(m: &HashMap<u32, Rule>,
          r: &RuleChain,
          s: &str) -> bool {
  //println!("{:?}", (s, r));
  assert!(r.head.len() > 0);
  
  let ctail = chain(&r.head[1..], r.tail);
  let ntail = match r.head.len() > 1 {
    false => r.tail,
    _ => Some(&ctail),
  };
  match r.head[0] {
    Rule::Literal(c) => {
      if let Some(fc) = s.chars().next() {
        if fc == c {
          match ntail {
            Some(t) => matchr(m, t, &s[fc.len_utf8()..]),
            None => s.len() == 1,
          }
        } else {
          false
        }
      } else {
        false
      }
    },
    Rule::Ref(i) => matchr(m, &chainr(&m[&i], ntail), s),
    Rule::Seq(ref v) => matchr(m, &chain(v, ntail), s),
    Rule::Choice(ref left, ref right) => {
      matchr(m, &chainr(left.borrow(), ntail), s) ||
      matchr(m, &chainr(right.borrow(), ntail), s)
    },
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines()
    .map(|r| r.unwrap());
  
  let mut rules = HashMap::new();
  for line in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut ir = line.split(':');
    let i = ir.next().unwrap().parse::<u32>().unwrap();
    let r = parse(&mut ir.next().unwrap().trim()
                    .split(|c: char| c.is_ascii_whitespace()),
                  Vec::new());
    rules.insert(i, r);
  }
  
  let s = lines.by_ref().take_while(|l| l.len() > 0)
    .map(|l| matchr(&rules, &chainr(&rules[&0], None), &l))
    .filter(|x| *x)
    .count();

  println!("{}", s);
}