use std::fmt;
use crate::monsters::MonsterFlavor;
use crate::items::MagicFlavor;

pub enum WallFacing {
	Vertical,
	Horizontal
}

pub enum Symbol {
	Hero,
	Floor,
	Door,
	Passage,
	Wall(WallFacing),
	Staircase,
	Monster(MonsterFlavor),
	Gold,
	Weapon,
	Poition,
	Armor,
	Food,
	Amulet,
	Staff,
	Ring,
	Trap,
	Scroll,
	Magic(MagicFlavor),
}

impl fmt::Display for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Symbol::*;
		write!(f, "{}", match self {
			Hero => "8",
			Floor => ":",
			Door => "/",
			Passage => "#",
			Wall(facing) => match facing {
				WallFacing::Horizontal => "-",
				WallFacing::Vertical => "|"
			},
			Staircase => "=",
			Monster(flavor) => match flavor {
				_ => todo!()	
			},
			Gold => "$",
			Weapon => "^",
			Poition => "?",
			Armor => "*",
			Food => "%",
			Amulet => "&",
			Staff => "!",
			Ring => "o",
			Trap => "#",
			Scroll => "~",
			Magic(flavor) => match flavor {
				MagicFlavor::Safe => "$",
				MagicFlavor::Perilous => "+"
			}
		})
	}
}
