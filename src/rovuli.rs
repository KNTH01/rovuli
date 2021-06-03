use billboard::{Alignment, Billboard, BorderStyle};
use chrono::{Duration, NaiveDate, Utc};
use colored::*;
use dialoguer::Input;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Result as ResultSerde;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Serialize, Deserialize)]
struct SimpleDate {
    day: String,
    month: String,
}

#[derive(Serialize, Deserialize)]
struct WindowDate {
    start: String,
    end: String,
    month: String,
}

pub struct UserInput<T> {
    last_period_date: NaiveDate,
    avg_cycle_days: T,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    fertile_window: WindowDate,
    approximate_ovulation: SimpleDate,
    next_period: SimpleDate,
    next_pregnancy_test: SimpleDate,
}

impl UserData {
    pub fn compute(user_data: &UserInput<u16>) -> UserData {
        // compute next period
        let next_period_date =
            user_data.last_period_date + Duration::days(user_data.avg_cycle_days as i64 - 1);

        // compute next pregnancy test date
        let next_pregnancy_test_date =
            user_data.last_period_date + Duration::days(user_data.avg_cycle_days as i64);

        // compute ovulation date
        const MAX_OVULATION_DAYS: u16 = 26;
        let current_pregnancy_cycle = 40 - user_data.avg_cycle_days;
        let ovulation_days = MAX_OVULATION_DAYS - current_pregnancy_cycle - 1;
        let approximate_ovulation_date =
            user_data.last_period_date + Duration::days(ovulation_days as i64);

        return UserData {
            next_period: SimpleDate {
                day: next_period_date.format("%d").to_string(),
                month: next_period_date.format("%b").to_string(),
            },
            next_pregnancy_test: SimpleDate {
                day: next_pregnancy_test_date.format("%d").to_string(),
                month: next_pregnancy_test_date.format("%b").to_string(),
            },
            approximate_ovulation: SimpleDate {
                day: approximate_ovulation_date.format("%d").to_string(),
                month: approximate_ovulation_date.format("%b").to_string(),
            },
            fertile_window: WindowDate {
                start: (approximate_ovulation_date - Duration::days(3))
                    .format("%d")
                    .to_string(),
                end: (approximate_ovulation_date + Duration::days(1))
                    .format("%d")
                    .to_string(),
                month: (approximate_ovulation_date - Duration::days(3))
                    .format("%b")
                    .to_string(),
            },
        };
    }
}

pub fn fetch_user_data() -> Result<UserInput<u16>, String> {
    let today = Utc::now().format(DATE_FORMAT).to_string();
    let default_avg_cycle_days = String::from("25");

    let last_date_input = NaiveDate::parse_from_str(
        &Input::new()
            .validate_with(|input: &String| -> Result<(), &str> {
                let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
                if re.is_match(input) {
                    Ok(())
                } else {
                    Err("This is not valid date")
                }
            })
            .with_prompt("Enter the first day of your last period (YYYY-MM-DD)")
            .with_initial_text(&today)
            .default(today.as_str().into())
            .interact_text()
            .unwrap(),
        DATE_FORMAT,
    )
    .unwrap();

    let avg_cycle_days_input = Input::new()
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<u16>() {
                Ok(_v) => Ok(()),
                Err(_e) => Err("It must be a number"),
            }
        })
        .with_prompt("How long is your average cycle (in Days)?")
        .default(default_avg_cycle_days)
        .interact_text()
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let user_data = UserInput {
        last_period_date: last_date_input,
        avg_cycle_days: avg_cycle_days_input,
    };

    Ok(user_data)
}

pub fn print_output(rovuli_data: &UserData) {
    let formated_output = String::from(format!(
        "{}: {}\n\n
{}: {}-{} {}
{}: {} {}
{}: {} {}
{}: {} {}",
        "rovuli".magenta().bold(),
        "Ovulation Cycle Calculator".italic(),
        "Fertile Window".green().bold(),
        rovuli_data.fertile_window.start,
        rovuli_data.fertile_window.end,
        rovuli_data.fertile_window.month,
        "Approximate Ovulation".purple().bold(),
        rovuli_data.approximate_ovulation.day,
        rovuli_data.approximate_ovulation.month,
        "Next Period".yellow().bold(),
        rovuli_data.next_period.day,
        rovuli_data.next_period.month,
        "Pregnancy Test Day".blue().bold(),
        rovuli_data.next_pregnancy_test.day,
        rovuli_data.next_pregnancy_test.month,
    ));

    Billboard::builder()
        .padding(1)
        .margin(1)
        .text_alignment(Alignment::Left)
        .box_alignment(Alignment::Left)
        .border_style(BorderStyle::Double)
        .border_color(billboard::BorderColor::Yellow)
        .build()
        .display(&formated_output);
}

pub fn print_user_data_json(user_data_history: &[&UserData]) -> ResultSerde<()> {
    let k = serde_json::to_string(&user_data_history)?;

    println!("{}", k);

    Ok(())
}

// [
//   {
// should be date range
//     "fertile_window": { "start": "09", "end": "13", "month": "Jun" },
// should be dates
//     "approximate_ovulation": { "day": "12", "month": "Jun" },
//     "next_period": { "day": "26", "month": "Jun" },
//     "next_pregnancy_test": { "day": "27", "month": "Jun" }
//   }
// ]
