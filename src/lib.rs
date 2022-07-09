use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

pub mod user_io;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub fertile_window: (NaiveDate, NaiveDate),
    pub approximate_ovulation: NaiveDate,
    pub next_period: NaiveDate,
    pub next_pregnancy_test: NaiveDate,
}

impl UserData {
    pub fn compute(user_data: &user_io::UserInput<u16>) -> UserData {
        // compute ovulation date
        const MAX_OVULATION_DAYS: u16 = 26;
        let current_pregnancy_cycle = 40 - user_data.avg_cycle_days;
        let ovulation_days = MAX_OVULATION_DAYS - current_pregnancy_cycle - 1;
        let approximate_ovulation_date =
            user_data.last_period_date + Duration::days(ovulation_days as i64);

        return UserData {
            next_period: user_data.last_period_date
                + Duration::days(user_data.avg_cycle_days as i64 - 1),
            next_pregnancy_test: user_data.last_period_date
                + Duration::days(user_data.avg_cycle_days as i64),
            approximate_ovulation: approximate_ovulation_date,
            fertile_window: (
                approximate_ovulation_date - Duration::days(3),
                approximate_ovulation_date + Duration::days(1),
            ),
        };
    }
}
