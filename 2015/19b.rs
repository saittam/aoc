use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Rule<'a> {
  left: &'a str,
  right: &'a str,
}

type RuleMap<'a> =
  HashMap<(char, char), HashSet<Rule<'a>>>;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Match<'a> {
  prefix: String,
  pos: usize,
  rule: &'a Rule<'a>,
}

impl<'a> Match<'a> {
  fn new(prefix: &str,
         pos: usize,
         rule: &'a Rule<'a>) -> Match<'a> {
    let prefix = prefix.to_string();
    Match { prefix, pos, rule }
  }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Tail<'a> {
  start: Match<'a>,
  suffix: String,
  goal: &'a str,
}

impl<'a> Tail<'a> {
  fn new(start: Match<'a>,
         suffix: &str,
         goal: &'a str) -> Tail<'a> {
    let suffix = suffix.to_string();
    Tail { start, suffix, goal }
  }
}

type CountedTail<'a> = (usize, Tail<'a>);

struct Context<'a> {
  rules: &'a RuleMap<'a>,
  links: HashMap<Match<'a>, Vec<CountedTail<'a>>>,
  queue: BinaryHeap<Reverse<CountedTail<'a>>>,
  qnext: BinaryHeap<Reverse<CountedTail<'a>>>,
  done: HashSet<Tail<'a>>,
}

impl<'a> Context<'a> {
  fn new(rules: &'a RuleMap<'a>) -> Context<'a> {
    let links = HashMap::new();
    // Insert a tail into the queue to indicate what to
    // construct. Note that its goal will never complete,
    // but the $ will remain in the queue, along with the
    // total number of replacements.
    const ROOT_RULE: Rule = Rule { left: "", right: "e$" };
    let root_match = Match::new("", 0, &ROOT_RULE);
    let root = Tail::new(root_match, "", &ROOT_RULE.right);
    let queue = BinaryHeap::new();
    let mut qnext = BinaryHeap::new();
    qnext.push(Reverse((0, root)));
    let done = HashSet::new();
    Context { rules, links, queue, qnext, done, }
  }

  fn update(&mut self,
            (count, tail): CountedTail<'a>,
            prefix: &str,
            pos: usize) {
    if tail.goal.is_empty() {
      let rprefix = format!("{}{}",
                            tail.start.rule.left,
                            prefix);
      let rec = self.links[&tail.start].iter()
        .map(|(lcount, ltail)| {
          let rtail = Tail::new(ltail.start.clone(),
                                &rprefix,
                                ltail.goal);
          let rcount = 1 + count + lcount;
          (rcount, rtail)
        }).collect::<Vec<_>>();
      for (rcount, rtail) in rec {
        self.update((rcount, rtail), &rprefix, pos);
      }
      return;
    }
    
    let ct = (count, tail);
    let mut pi = prefix.chars();
    if let Some(input) = pi.next() {
      self.process(ct, input, pi.as_str(), pos);
    } else {
      self.qnext.push(Reverse(ct));
    }
  }
  
  fn process(&mut self,
             (count, tail): CountedTail<'a>,
             input: char,
             prefix: &str,
             pos: usize) {
    if !self.done.insert(tail.clone()) {
      return;
    }

    let mut goali = tail.goal.chars();
    let gc = goali.next().expect("goal empty");
    if let Some(rs) = self.rules.get(&(input, gc)) {
      for rule in rs {
        let mut rgoal = rule.right.chars();
        assert_eq!(rgoal.next(), Some(input));
        let m = Match::new(prefix, pos, rule);
        
        self.links.entry(m.clone())
          .and_modify(|v| {
            if let Result::Err(p) = v.binary_search_by_key(
              &&tail, |(_, t)| t) {
              v.insert(p, (count, tail.clone()))
            }
          })
          .or_insert_with(|| vec![(count, tail.clone())]);
        
        let rtail = Tail::new(m, prefix, rgoal.as_str());
        self.update((0, rtail), prefix, pos);
      }
    }
    
    if input == gc {
      let rm = tail.start.clone();
      let rtail = Tail::new(rm, prefix, goali.as_str());
      self.update((count, rtail), prefix, pos);
    }
  }

  fn step(&mut self,
          input: char,
          pos: usize) {
    self.done.clear();
    self.queue.clear();
    std::mem::swap(&mut self.queue, &mut self.qnext);
    while let Some(Reverse(ct)) = self.queue.pop() {
      self.process(ct, input, "", pos);
    }
  }

  fn count(&self) -> Option<usize> {
    self.qnext.iter()
      .filter(|Reverse((_, t))| t.goal == "$")
      .map(|Reverse((c, _))| *c)
      .next()
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut r = Vec::new();
  for l in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut t = l.split(" => ");
    r.push((t.next().unwrap().to_owned(),
            t.next().unwrap().to_owned()));
  }

  let s = lines.next().unwrap();

  // Make a map to look up any potential rule to
  // pursue for a given input and goal character.
  let mut rules = RuleMap::new();
  for (left, right) in &r {
    let k = (right.chars().next().unwrap(),
             left.chars().next().unwrap());
    rules.entry(k)
      .or_insert_with(HashSet::new)
      .insert(Rule { left, right });
  }
  loop {
    let mut nrules = rules.clone();
    for (i1, g1) in rules.keys() {
      for ((i2, g2), rs) in &rules {
        if g2 == i1 {
          nrules
            .entry((*i2, *g1))
            .or_insert_with(HashSet::new)
            .extend(rs.iter().cloned());
        } 
      }
    }
    if rules == nrules {
      break;
    }
    rules = nrules;
  }

  // Scan over the input, keeping track of which partial
  // rule matches are possible, extending them with input
  // character step by step, and reducing complete matches.
  let mut ctx = Context::new(&rules); 
  for (pos, c) in s.chars().enumerate() {
    ctx.step(c, pos);
    
    println!("{:>3} char {} nproc {} open {}",
             pos, c, ctx.done.len(), ctx.qnext.len());
  }

  let n = ctx.count().expect("cannot make target");
  println!("{}", n);
}