use rovuli;

fn main() -> Result<(), String> {
    let user_input = rovuli::fetch_user_data().expect("Failed to read your input");
    let result = rovuli::UserData::compute(&user_input);

    rovuli::print_output(&result);

    rovuli::print_user_data_json(&vec![&result]).ok();

    Ok(())
}
