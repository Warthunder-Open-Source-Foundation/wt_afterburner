#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Thrust {
    // Current level of thrust
    pub current: u8,
    // Known levels of thrust
	pub ab_levels: [Option<u8>; 10],
}

impl Thrust {
	pub fn new(throttle: u8) -> Self {
		let mut levels = [None; 10];
		if let Some(res) = 	levels.get_mut(thrust_to_ab_idx(throttle)) {
			*res = Some(throttle)
		}
		Self {
			current: throttle,
			ab_levels: levels,
		}
	}
	pub fn add_ab_level(&mut self, throttle: u8) {
		if throttle > 100 {
			if let Some(res) = self.ab_levels.get_mut(thrust_to_ab_idx(throttle)) {
				*res = Some(throttle)
			}
		}
	}
	// This function trades not needing a &mut ref for returning an option
	fn get_ab(&self, throttle: u8) -> Option<u8> {
		if throttle < 101 {
			return None;
		}
		let known_stages = self.ab_levels.into_iter().filter_map(|x|x).enumerate();

		for (idx, known_stage) in known_stages {
			if throttle == known_stage {
				return Some(idx as u8);
			}
		}
		return None;
	}
	pub fn get_and_set_ab(&mut self, throttle: u8) -> Option<u8> {
		self.add_ab_level(throttle);
		self.get_ab(throttle)
	}
	pub fn get_current (&self) -> u8 {
		self.current
	}
}

fn thrust_to_ab_idx(throttle: u8) -> usize {
    let abs = throttle.saturating_sub(100 + 1);
    abs as usize
}

#[cfg(test)]
mod tests {
	use crate::Thrust;

	#[test]
	fn test_ab_level_direct() {
		let mut thrust = Thrust::new(101);
		thrust.add_ab_level(110);
		thrust.add_ab_level(105);
		assert_eq!(1, thrust.get_and_set_ab(105).unwrap())
	}

	#[test]
	fn test_all_levels() {
		let mut thrust = Thrust::new(101);
		for i in 102..=110 {
			thrust.add_ab_level(i);
		}
		for i in 0..10 {
			assert_eq!(Some(i), thrust.get_ab(i + 100 + 1))
		}
	}

	#[test]
	fn test_all() {
		let mut thrust = Thrust::new(0);
		for i in 0..=110 {
			thrust.get_and_set_ab(i);
		}
	}
}
