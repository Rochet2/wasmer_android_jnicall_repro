#[no_mangle]
pub fn run() -> i64 {
	let radius = 5;
	let pi = std::f64::consts::PI;
	let r = radius as f64;
	let mut res = 0;
	for i in 0..3600 {
		let angle = i as f64 * 0.01;
		let x1 = r * (angle * pi / 180.0).cos();
		let y1 = r * (angle * pi / 180.0).sin();
		// res = res + x1 as i32;

		// moving this below the next IF crashes
		// Changing the return type to f64 makes it work
		// In that case you must return for example res as f64

		if i == 100 {
			return res; // return res as f64;
		}
		if 1 == x1 as i32  {
			res += 1
		}
		if 1 == y1 as i32 {
			res += 1
		}
	}
	res
}

