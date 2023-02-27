use crate::util::Point;
use crate::items::Inventoy;

pub struct Player {
	pos: Point,
	inventory: Inventory
}

impl Player {
		pub fn new() -> Self {
			Self {
				pos: Point::new(0, 0),
				inventory: Inventory::new()
			}
		}
}
