pub struct Point {
	pub x: u32,
	pub y: u32,
}

impl Point {
	fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y
		}
	}
}
