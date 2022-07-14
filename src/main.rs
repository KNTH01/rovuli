use rovuli::{user_io, UserData};

fn main() -> Result<(), String> {
    let user_input = user_io::prompt_input().expect("Failed to read user's input");
    let user_data = UserData::compute(&user_input);

    user_io::print(&user_data);
    user_io::print_json(&[&user_data]).ok();

    Ok(())
}
