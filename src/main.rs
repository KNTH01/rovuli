use rovuli;
mod output_printer;

fn main() -> Result<(), String> {
    let user_input = rovuli::fetch_user_data().expect("Failed to read your input");
    let result = rovuli::UserData::compute(&user_input);

    output_printer::print_output(&result);
    output_printer::print_user_data_json(&vec![&result]).ok();

    Ok(())
}
