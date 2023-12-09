use std::collections::HashSet;

fn main() {
	let mut seen = HashSet::<usize>::new();
	let mut r4: usize = 0;
	let mut r5: usize = 0;
	let mut last: usize = 0;
	
	loop {
		r5 = r4 | 0x10000;
		r4 = 10704114;
		loop {
			//println!("{} {}", r4, r5);
			r4 = (((r4 + (r5 & 0xff)) & 0xffffff) * 65899) & 0xffffff;
			//println!("{} {}", r4, r5);
			if r5 < 256 {
				break;
			}
			r5 /= 256;
		}
		if seen.contains(&r4) {
			println!("cycle {} last {}", r4, last);
			break;
		}
		seen.insert(r4);
		last = r4;
	}

}