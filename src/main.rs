mod rustovuli;

fn main() -> Result<(), String> {
    let user_input = rustovuli::fetch_user_data().expect("Failed to read your input");
    let result = rustovuli::UserData::compute(&user_input);

    rustovuli::print_output(&result);

    rustovuli::print_user_data_json(&vec![&result]);

    Ok(())
}
