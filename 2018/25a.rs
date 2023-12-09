//rust 1.17.0 
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};
use std::hash::Hash;
//use std::cmp;
//use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
struct UnionFindRoot<Key: std::clone::Clone> {
   size: usize,
	rep: Key,
}

enum UnionFindNode<Key: std::clone::Clone> {
	Ref(Rc<RefCell<UnionFindNode<Key>>>),
	Root(UnionFindRoot<Key>),
}

fn find<Key: std::clone::Clone>(node: &mut Rc<RefCell<UnionFindNode<Key>>>)
   -> Rc<RefCell<UnionFindNode<Key>>>
{
	let root = match *node.borrow_mut() {
	   UnionFindNode::Ref(ref mut r) => find(r),
	   _ => node.clone(),
	};
	*node = root.clone();
	return root;
}

fn root<Key: std::clone::Clone>(node: &Rc<RefCell<UnionFindNode<Key>>>)
   -> Option<UnionFindRoot<Key>>
{
	match *node.borrow() {
	   UnionFindNode::Ref(ref r) => None,
	   UnionFindNode::Root(ref r) => Some(r.clone()),
   }
}

struct UnionFind<Key> 
where Key: Eq, Key: Hash, Key: std::clone::Clone
{
	map: HashMap<Key, Rc<RefCell<UnionFindNode<Key>>>>,
}


impl<Key> UnionFind<Key>
where Key: Eq, Key: Hash, Key: std::clone::Clone
{
	fn contains(&self, key: Key) -> bool {
		self.map.contains_key(&key)
	}
	
	fn insert(&mut self, key: Key)
	   -> &mut Rc<RefCell<UnionFindNode<Key>>> {
	   let rep = key.clone();
		return self.map.entry(key)
		   .or_insert(Rc::new(RefCell::new(
		   	UnionFindNode::Root(
		   		UnionFindRoot{ size: 1, rep: rep }))));
	}
	
	fn union(&mut self, key1: Key, key2: Key) {
		let n1 = find(self.insert(key1));
		let n2 = find(self.insert(key2));
		let r1 = root(&n1).unwrap();
		let r2 = root(&n2).unwrap();
		
		if r1.rep == r2.rep {
			return;
		}
		
		let (nroot, nchild, rep) = if r1.size < r2.size {
		   (n2, n1, r2.rep)
		} else {
			(n1, n2, r1.rep)
		};
		*nroot.borrow_mut() = UnionFindNode::Root(
			UnionFindRoot{ size: r1.size + r2.size, rep: rep });
		*nchild.borrow_mut() = UnionFindNode::Ref(nroot.clone());
	}
	
	fn count(&mut self) -> usize {
		self.map.values_mut()
		   .map(|mut n| root(&find(&mut n)).unwrap().rep)
		   .collect::<HashSet<Key>>()
		   .len()
	}
}

type Point = (isize, isize, isize, isize);

fn neg(p: &Point) -> Point {
	(-p.0, -p.1, -p.2, -p.3)
}

fn add(a: &Point, b: &Point) -> Point {
	(a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

fn scl(a: &Point, b: &Point) -> isize {
	a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3
}

fn dist(a: &Point, b: &Point) -> isize {
	(a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut seq = Vec::<Point>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let c: Vec<isize> = buf
		   .split(|c:char| !c.is_digit(10) && c != '-')
  		 .filter(|c:&&str| !c.is_empty())
  		 .map(|s| s.parse::<isize>().unwrap())
  		 .collect();
  	 seq.push((c[0], c[1], c[2], c[3]));
   }
   
   let mut offs = Vec::<Point>::new();
   for a in -3..4 {
   	for b in -3..4 {
   		for c in -3..4 {
   			for d in -3..4 {
   				let p = (a, b, c, d);
   				if dist(&p, &(0, 0, 0, 0)) <= 3 {
   					offs.push(p);
   				}
   			}
   		}
   	}
   }
   
   let mut cons = UnionFind::<Point>{ map: HashMap::new() }; 
   
   for p in seq {
   	cons.insert(p);
   	
   	for o in offs.iter() {
   		let n = add(&p, &o);
   		if cons.contains(n) {
   			cons.union(n, p);
   		}
   	}
   }
   
   println!("{}", cons.count());
}
    
    
