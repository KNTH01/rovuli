use billboard::{Alignment, Billboard, BorderStyle};
use chrono::{Duration, TimeZone, Utc};
use colored::*;

struct SimpleDate {
    day: String,
    month: String,
}
struct WindowDate {
    start: String,
    end: String,
    month: String,
}

fn main() {
    let last_date = Utc.ymd(2021, 05, 28);
    let avg_cycle_days = 26;

    // compute next period
    let next_period_date = last_date + Duration::days(avg_cycle_days - 1);
    let next_period = SimpleDate {
        day: next_period_date.format("%d").to_string(),
        month: next_period_date.format("%b").to_string(),
    };

    // compute next pregnancy test date
    let next_pregnancy_test_date = last_date + Duration::days(avg_cycle_days);
    let next_pregnancy_test = SimpleDate {
        day: next_pregnancy_test_date.format("%d").to_string(),
        month: next_pregnancy_test_date.format("%b").to_string(),
    };

    // compute ovulation date
    const MAX_OVULATION_DAYS: i64 = 26;
    let current_pregnancy_cycle = 40 - avg_cycle_days;
    let ovulation_days = MAX_OVULATION_DAYS - current_pregnancy_cycle - 1;
    let approximate_ovulation_date = last_date + Duration::days(ovulation_days);
    let approximate_ovulation = SimpleDate {
        day: approximate_ovulation_date.format("%d").to_string(),
        month: approximate_ovulation_date.format("%b").to_string(),
    };

    let fertile_window = WindowDate {
        start: (approximate_ovulation_date - Duration::days(3))
            .format("%d")
            .to_string(),
        end: (approximate_ovulation_date + Duration::days(1))
            .format("%d")
            .to_string(),
        month: (approximate_ovulation_date - Duration::days(3))
            .format("%b")
            .to_string(),
    };

    let result = String::from(format!(
        "{}: {}-{} {}
{}: {} {}
{}: {} {}
{}: {} {}",
        "Fertile Window".green().bold(),
        fertile_window.start,
        fertile_window.end,
        fertile_window.month,
        "Approximate Ovulation".purple().bold(),
        approximate_ovulation.day,
        approximate_ovulation.month,
        "Next Period".yellow().bold(),
        next_period.day,
        next_period.month,
        "Pregnancy Test Day:".blue().bold(),
        next_pregnancy_test.day,
        next_pregnancy_test.month,
    ));

    Billboard::builder()
        .padding(1)
        .margin(1)
        .text_alignment(Alignment::Left)
        .box_alignment(Alignment::Left)
        .build()
        .display(&String::from(format!(
            "{}: {}",
            "Rustovuli".magenta().bold(),
            "Ovulation Cycle Calculator".italic()
        )));

    Billboard::builder()
        .padding(1)
        .margin(1)
        .text_alignment(Alignment::Left)
        .box_alignment(Alignment::Left)
        .border_style(BorderStyle::Double)
        .border_color(billboard::BorderColor::Yellow)
        .build()
        .display(&result);
}
