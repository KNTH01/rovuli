mod rustovuli;

fn main() -> Result<(), String> {
    let user_data = rustovuli::fetch_user_data().expect("Failed to read your input");
    let result = rustovuli::UserData::compute(&user_data);

    rustovuli::print_output(&result);

    Ok(())
}
