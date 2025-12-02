pub struct CombinationSafe {
    pub pos: i16,
}

impl Default for CombinationSafe {
    fn default() -> Self {
        CombinationSafe { pos: 50 }
    }
}

impl CombinationSafe {
    pub fn process_dials(&mut self, dials: &[&str]) -> (i16, i16) {
        dials
            .iter()
            .map(|next| {
                let (direction, amount) = next.split_at(1);
                let n: i16 = amount.parse().unwrap();
                let sign = if direction == "L" { -1 } else { 1 };
                self.dial(sign * n)
            })
            .fold((0, 0), |(land_acc, rot_acc), (land, rot)| {
                (land_acc + land, rot_acc + rot)
            })
    }

    pub fn dial(&mut self, n: i16) -> (i16, i16) {
        let old_pos = self.pos;
        let raw_pos = old_pos + n;
        let new_pos = raw_pos.rem_euclid(100);
        self.pos = new_pos;

        let wraps = if n > 0 {
            (old_pos + n) / 100
        } else {
            let reversed = (100 - old_pos) % 100;
            (reversed + (-n)) / 100
        };

        let landed = (new_pos == 0) as i16;
        let rotations = wraps - landed; // If we landed on 0, subtract 1 from wraps

        (landed, rotations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_left() {
        let mut safe = CombinationSafe::default();
        safe.dial(-10);
        assert_eq!(safe.pos, 40);
    }

    #[test]
    fn test_dial_right() {
        let mut safe = CombinationSafe::default();
        safe.dial(10);
        assert_eq!(safe.pos, 60);
    }

    #[test]
    fn test_wraps_around() {
        let mut safe = CombinationSafe::default();
        safe.dial(-60);
        assert_eq!(safe.pos, 90);
    }

    #[test]
    fn test_lands_on_zero() {
        let mut safe = CombinationSafe::default();
        let (landed_clicks, _) = safe.dial(-50);
        assert_eq!(safe.pos, 0);
        assert_eq!(landed_clicks, 1);
    }

    #[test]
    fn test_single_wrap_right() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(60); // 50 + 60 = 110, wraps once
        assert_eq!(rotations, 1);
        assert_eq!(safe.pos, 10);
    }

    #[test]
    fn test_single_wrap_left() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(-60); // 50 - 60 = -10, wraps once
        assert_eq!(rotations, 1);
        assert_eq!(safe.pos, 90);
    }

    #[test]
    fn test_multiple_wraps_right() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(1000); // 50 + 1000 = 1050, wraps 10 times
        assert_eq!(rotations, 10);
        assert_eq!(safe.pos, 50);
    }

    #[test]
    fn test_multiple_wraps_left() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(-1000); // 50 - 1000 = -950, wraps 10 times
        assert_eq!(rotations, 10);
        assert_eq!(safe.pos, 50);
    }

    #[test]
    fn test_exactly_100_right() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(100); // 50 + 50 = 100, wraps once
        assert_eq!(rotations, 1);
        assert_eq!(safe.pos, 50);
    }

    #[test]
    fn test_exactly_100_left() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(-50); // 50 - 50 = 0, no wrap
        assert_eq!(rotations, 0);
        assert_eq!(safe.pos, 0);
    }

    #[test]
    fn test_no_wrap_right() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(30); // 50 + 30 = 80, no wrap
        assert_eq!(rotations, 0);
        assert_eq!(safe.pos, 80);
    }

    #[test]
    fn test_no_wrap_left() {
        let mut safe = CombinationSafe::default();
        let (_, rotations) = safe.dial(-30); // 50 - 30 = 20, no wrap
        assert_eq!(rotations, 0);
        assert_eq!(safe.pos, 20);
    }
    #[test]
    fn test_wrap_and_end_on_zero() {
        let mut safe = CombinationSafe::default();
        let (landed, rotated) = safe.dial(50); // 50 + 50 = 100, wraps once and ends on 0
        assert_eq!(landed, 1);
        assert_eq!(rotated, 0);
    }

    #[test]
    fn test_wrap_ends_on_zero_vs_passes_through() {
        let mut safe1 = CombinationSafe::default();
        let (land1, rot1) = safe1.dial(50); // Wraps once, ends on 0
        assert_eq!(land1, 1);
        assert_eq!(rot1, 0);

        let mut safe2 = CombinationSafe::default();
        let (land2, rot2) = safe2.dial(60); // Wraps once, ends on 10 (passed through 0)
        assert_eq!(land2, 0);
        assert_eq!(rot2, 1);
    }
}
