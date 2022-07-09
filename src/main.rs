use rovuli::{user_io, UserData};

fn main() -> Result<(), String> {
    let user_input = user_io::prompt_input().expect("Failed to read your input");
    let user_data = UserData::compute(&user_input);

    user_io::print(&user_data);
    user_io::print_json(&vec![&user_data]).ok();

    Ok(())
}
