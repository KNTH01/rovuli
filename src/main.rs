use rovuli::{user_io};

fn main() -> Result<(), String> {
    let user_input = user_io::fetch_user_data().expect("Failed to read your input");
    let result = rovuli::UserData::compute(&user_input);

    user_io::print_output(&result);
    user_io::print_user_data_json(&vec![&result]).ok();

    Ok(())
}
