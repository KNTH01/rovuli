use billboard::{Alignment, Billboard, BorderStyle};
use colored::*;
use serde_json::Result as ResultSerde;
use chrono::NaiveDate;
use rovuli::UserData;

pub fn print_output(rovuli_data: &UserData) {
    fn get_format_day(date: NaiveDate) -> String {
        date.format("%d").to_string()
    }
    fn get_format_month(date: NaiveDate) -> String {
        date.format("%b").to_string()
    }

    let formated_output = String::from(format!(
        "{}: {}\n\n
{}: {}-{} {}
{}: {} {}
{}: {} {}
{}: {} {}",
        "rovuli".magenta().bold(),
        "Ovulation Cycle Calculator".italic(),
        "Fertile Window".green().bold(),
        get_format_day(rovuli_data.fertile_window.0),
        get_format_day(rovuli_data.fertile_window.1),
        get_format_month(rovuli_data.fertile_window.0),
        "Approximate Ovulation".purple().bold(),
        get_format_day(rovuli_data.approximate_ovulation),
        get_format_month(rovuli_data.approximate_ovulation),
        "Next Period".yellow().bold(),
        get_format_day(rovuli_data.next_period),
        get_format_month(rovuli_data.next_period),
        "Pregnancy Test Day".blue().bold(),
        get_format_day(rovuli_data.next_pregnancy_test),
        get_format_month(rovuli_data.next_pregnancy_test),
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
