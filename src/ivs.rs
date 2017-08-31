#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum OverallAppraisal {
    Bad,
    Okay,
    Good,
    Great,
}

impl OverallAppraisal {
    pub fn min_stats(self) -> IndividualValue {
        match self {
            OverallAppraisal::Great => IndividualValue::new(7, 7, 7).unwrap(),
            OverallAppraisal::Good => IndividualValue::new(1, 1, 1).unwrap(),
            _ => IndividualValue::new(0, 0, 0).unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum StatRange {
    Low,
    Average,
    High,
    Perfect,
}

impl StatRange {
    pub fn max_stats(self) -> IndividualValue {
        match self {
            StatRange::Low => IndividualValue::new(7, 7, 7).unwrap(),
            StatRange::Average => IndividualValue::new(12, 12, 12).unwrap(),
            StatRange::High => IndividualValue::new(14, 14, 14).unwrap(),
            StatRange::Perfect => IndividualValue::new(15, 15, 15).unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TopStat {
    attack: bool,
    defense: bool,
    stamina: bool,
}

impl TopStat {
    pub fn new(attack: bool, defense: bool, stamina: bool) -> Option<TopStat> {
        if !(attack || defense || stamina) {
            return None;
        }

        Some(TopStat {
            attack,
            defense,
            stamina,
        })
    }

    pub fn attack_hi(&self) -> bool {
        self.attack
    }

    pub fn defense_hi(&self) -> bool {
        self.defense
    }

    pub fn stamina_hi(&self) -> bool {
        self.stamina
    }

    pub fn matches(&self, value: &IndividualValue) -> bool {
        let max_stat = value.highest_stat_value();

        self.attack == (value.attack() == max_stat) &&
            self.defense == (value.defense() == max_stat) &&
            self.stamina == (value.stamina() == max_stat)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PokeEvaluation {
    overall: OverallAppraisal,
    stat_range: StatRange,
    top_stat: TopStat,
}

impl PokeEvaluation {
    pub fn new(
        overall: OverallAppraisal,
        stat_range: StatRange,
        top_stat: TopStat,
    ) -> PokeEvaluation {
        PokeEvaluation {
            overall,
            stat_range,
            top_stat,
        }
    }

    pub fn overall(&self) -> OverallAppraisal {
        self.overall
    }

    pub fn stat_range(&self) -> StatRange {
        self.stat_range
    }

    pub fn top_stat(&self) -> TopStat {
        self.top_stat
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct IndividualValue {
    attack: u8,
    defense: u8,
    stamina: u8,
}

impl IndividualValue {
    pub fn new(attack: u8, defense: u8, stamina: u8) -> Option<IndividualValue> {
        if !IndividualValue::valid_stat(attack) || !IndividualValue::valid_stat(defense) ||
            !IndividualValue::valid_stat(stamina)
        {
            return None;
        }

        Some(IndividualValue {
            attack,
            defense,
            stamina,
        })
    }

    pub fn from_tuple((attack, defense, stamina): (u8, u8, u8)) {
        IndividualValue::new(attack, defense, stamina);
    }

    pub fn valid_stat(num: u8) -> bool {
        num <= 15
    }

    pub fn attack(&self) -> u8 {
        self.attack
    }

    pub fn defense(&self) -> u8 {
        self.defense
    }

    pub fn stamina(&self) -> u8 {
        self.stamina
    }

    pub fn highest_stat_value(&self) -> u8 {
        use std::cmp::max;

        max(self.attack(), max(self.defense(), self.stamina()))
    }

    pub fn top_stat(&self) -> TopStat {
        let highest_stat = self.highest_stat_value();

        TopStat::new(
            self.attack() == highest_stat,
            self.defense() == highest_stat,
            self.stamina() == highest_stat,
        ).unwrap()
    }

    pub fn as_tuple(&self) -> (u8, u8, u8) {
        (self.attack(), self.defense(), self.stamina())
    }

    pub fn overall_appraisal(&self) -> OverallAppraisal {
        let stat_sum = self.attack() + self.defense() + self.stamina();

        if stat_sum <= 22 {
            OverallAppraisal::Bad
        } else if stat_sum <= 29 {
            OverallAppraisal::Okay
        } else if stat_sum <= 36 {
            OverallAppraisal::Good
        } else {
            OverallAppraisal::Great
        }
    }

    pub fn matches_evaluation(&self, evaluation: PokeEvaluation) -> bool {
        self.overall_appraisal() == evaluation.overall() &&
            self.stat_range() == evaluation.stat_range() &&
            evaluation.top_stat().matches(self)
    }

    pub fn stat_range(&self) -> StatRange {
        let max_stat = self.highest_stat_value();

        if max_stat <= 7 {
            StatRange::Low
        } else if max_stat <= 12 {
            StatRange::Average
        } else if max_stat <= 14 {
            StatRange::High
        } else {
            StatRange::Perfect
        }
    }

    pub fn is_perfect(&self) -> bool {
        return self.attack() == 15 && self.defense() == 15 && self.stamina() == 15;
    }
}

#[cfg(test)]
mod tests {
    use ivs::{IndividualValue, TopStat};

    #[test]
    fn top_stat_new() {
        assert!(TopStat::new(false, false, false).is_none());
        assert!(TopStat::new(true, true, true).is_some());

        let top_stat = TopStat::new(true, false, true).unwrap();
        assert!(top_stat.attack_hi());
        assert!(!top_stat.defense_hi());
        assert!(top_stat.stamina_hi());
    }

    #[test]
    fn top_stat_match() {
        let top_stat = TopStat::new(false, true, true).unwrap();
        let iv = IndividualValue::new(0, 11, 11).unwrap();

        assert!(top_stat.matches(&iv));

        let iv = IndividualValue::new(11, 11, 0).unwrap();
        assert!(!top_stat.matches(&iv));
    }

    #[test]
    fn iv_new() {
        assert!(IndividualValue::new(0, 0, 0).is_some());
        assert!(IndividualValue::new(15, 15, 15).is_some());

        assert!(IndividualValue::new(16, 15, 0).is_none());

        let iv = IndividualValue::new(1, 2, 3).unwrap();
        assert_eq!((1, 2, 3), iv.as_tuple());
    }

    #[test]
    fn iv_is_perfect() {
        let perfect = IndividualValue::new(15, 15, 15).unwrap();
        assert!(perfect.is_perfect());

        let not_perfect = IndividualValue::new(15, 15, 14).unwrap();
        assert!(!not_perfect.is_perfect())
    }
}
