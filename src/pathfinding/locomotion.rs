
/// Needs solid ground underneath
pub const GROUND_WALK : u32 = 1 << 0;

/// Traverse up and down level "portals", like stairs and ramps
pub const CLIMB_STAIRS : u32 = 1 << 1;

/// Traverse up and down climbable structures, like ladders and vines
pub const CLIMB_LADDERS : u32 = 1 << 2;

/// Indicates how the entity can move
pub struct Locomotion {
    methods: u32,
}

impl Locomotion {
    pub fn new(methods: &[u32]) -> Self {
        Locomotion {
            methods: methods.iter().fold(0, |acc, x| acc | x),
        }
    }

    pub fn has_method(&self, method: u32) -> bool {
        self.methods & method == method
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_store() {
        let loc = Locomotion::new(&[GROUND_WALK, CLIMB_STAIRS]);

        assert!(loc.has_method(GROUND_WALK));
        assert!(loc.has_method(CLIMB_STAIRS));
        assert!(!loc.has_method(CLIMB_LADDERS));
    }
}
