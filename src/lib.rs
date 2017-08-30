const STARDUST_LEVEL_LOOKUP: &'static [(u32, &'static [f32])] = &[
    (200, &[1.0, 1.5, 2.0, 2.5]),
    (400, &[3.0, 3.5, 4.0, 4.5]),
    (600, &[5.0, 5.5, 6.0, 6.5]),
    (800, &[7.0, 7.5, 8.0, 8.5]),
    (1_000, &[9.0, 9.5, 10.0, 11.5]),
    (1_300, &[11.0, 11.5, 12.0, 12.5]),
    (1_600, &[13.0, 13.5, 14.5, 15.0]),
    (1_900, &[15.0, 15.5, 16.0, 16.5]),
    (2_200, &[17.0, 17.5, 18.0, 18.5]),
    (2_500, &[19.0, 19.5, 20.0, 20.5]),
    (3_000, &[21.0, 21.5, 22.0, 22.5]),
    (3_500, &[23.0, 23.5, 24.0, 24.5]),
    (4_000, &[25.0, 25.5, 26.5, 27.0]),
    (4_500, &[27.0, 27.5, 28.0, 28.5]),
    (5_000, &[29.0, 29.5, 30.0, 30.5]),
    (6_000, &[31.0, 31.5, 32.0, 32.5]),
    (7_000, &[33.0, 33.5, 34.0, 34.5]),
    (8_000, &[35.0, 35.5, 36.0, 36.5]),
    (9_000, &[37.0, 37.5, 38.0, 38.5]),
    (10_000, &[39.0, 39.5, 40.0]),
];

/// Given the stardust cost to power up a Pokemon, return the possible values for that Pokemon's
/// level. If `cost_to_powerup` is not a valid value (i.e., there is no Pokemon level that requires
/// that amount of dust) returns `None`.
///
/// Example:
///
/// ```
/// use pokemon_iv::possible_levels;
///
/// let powerup_cost = 2_200;
/// let possible_pokemon_levels: &[f32] = &[17.0, 17.5, 18.0, 18.5];
///
/// assert_eq!(Some(possible_pokemon_levels), possible_levels(powerup_cost));
/// ```
///
/// The highest possible value returned by this function is `40.0`.
pub fn possible_levels(cost_to_powerup: u32) -> Option<&'static [f32]> {
    STARDUST_LEVEL_LOOKUP
        .iter()
        .find(|&&(amount, _)| amount == cost_to_powerup)
        .map(|&(_, levels)| levels)
}

#[cfg(test)]
mod tests {
    use possible_levels;

    #[test]
    fn possible_levels_basic() {
        let cost = 200;
        let levels: &[f32] = &[1., 1.5, 2., 2.5];

        assert_eq!(Some(levels), possible_levels(cost));

        let cost = 2_200;
        let levels: &[f32] = &[17.0, 17.5, 18.0, 18.5];
        assert_eq!(Some(levels), possible_levels(cost));

        let cost = 8_000;
        let levels: &[f32] = &[35.0, 35.5, 36.0, 36.5];
        assert_eq!(Some(levels), possible_levels(cost));

        let cost = 10_000;
        let levels: &[f32] = &[39.0, 39.5, 40.0];
        assert_eq!(Some(levels), possible_levels(cost));

        let cost = 0;
        assert_eq!(None, possible_levels(cost));

        let cost = 100;
        assert_eq!(None, possible_levels(cost));

        let cost = 2_100;
        assert_eq!(None, possible_levels(cost));

        let cost = 11_000;
        assert_eq!(None, possible_levels(cost));
    }
}
