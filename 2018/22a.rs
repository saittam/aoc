
const DEPTH: usize = 3198;
const T: (usize, usize) = (12, 757);

fn main() {
	let target = T;

	let mut m = [[0; T.0 + 1]; T.1 + 1];
	
	for x in 1..(target.0 + 1) {
		m[0][x] = ((x * 16807) + DEPTH) % 20183;
	}
	for y in 1..(target.1 + 1) {
		m[y][0] = ((y * 48271) + DEPTH) % 20183;
		for x in 1..(target.0 + 1) {
			m[y][x] = ((m[y - 1][x] * m[y][x - 1]) + DEPTH) % 20183;
		}
	}
	m[target.1][target.0] = 0;
	let chrs = [ '.', '=', '|' ];
	let mut r = 0;
	for y in 0..(target.1 + 1) {
		let mut line = String::new();
		for x in 0..(target.0 + 1) {
			let t = m[y][x] % 3;
			r += t;
			line.push(chrs[t]);
		}
		println!("{}", line);
	}
	
	println!("{}", r);
}