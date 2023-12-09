use std::io::BufRead;
use std::collections::HashMap;

struct Dir {
  children: HashMap<String, Box<Entry>>,
}

impl Dir {
  fn new() -> Dir {
    Dir { children: HashMap::new() }
  }
}

struct File {
  size: usize,
}

impl File {
  fn new(size: usize) -> File {
    File { size }
  }
}

enum Entry {
  Dir(Dir),
  File(File),
}

fn parse<I: Iterator<Item=String>>(dir: &mut Dir, lines: &mut std::iter::Peekable<I>) {
  while let Some(l) = lines.peek() {
    let mut w = l.split(' ');
    assert_eq!("$", w.next().expect("input"));
    let cmd = w.next().expect("cmd");
    match cmd {
      "cd" => match w.next().expect("dest") {
          ".." => {
            lines.next();
            return;
          }
          "/" => {
            // Leave cd / on the iterator so the caller sees it.
            return;
          }
          d => {
            let he = dir.children.entry(d.to_owned());
            lines.next();
            match he.or_insert(Box::new(Entry::Dir(Dir::new()))).as_mut() {
              Entry::Dir(ref mut sd) => parse(sd, lines),
              _ => (),
            }
          }
        }
      "ls" => {
        lines.next();
        while let Some(l) = lines.next_if(|l| !l.starts_with("$")) {
          let mut w = l.split(' ');
          let ts = w.next().expect("size/dir");
          let name = w.next().expect("name");
          let entry = Box::new(match ts {
            "dir" => Entry::Dir(Dir::new()),
            size => Entry::File(File::new(size.parse::<usize>().expect("size"))),
          });
          dir.children.insert(name.to_owned(), entry);
        }
      }
      _ => panic!("unknown cmd {}", cmd),
    }
  }
}

fn size(dir: &Dir, small: &mut usize) -> usize {
  let sz = dir.children.values().map(|c|
    match c.as_ref() {
      Entry::Dir(d) => size(&d, small),
      Entry::File(f) => f.size,
    })
  .sum();
  if sz <= 100000 {
    *small += sz;
  }
  sz
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap()).take_while(|l| l.len() > 0).peekable();

  let mut root = Dir::new();
  while lines.peek().is_some() {
    parse(&mut root, &mut lines);
    lines.next();
  }

  let mut small = 0;
  size(&root, &mut small);
  println!("{}", small);
}