use std::{str::FromStr, io::stdin};

pub fn query_user_for_value_with_message<T: FromStr>(msg:String) -> Result<T, <T as FromStr>::Err> {
    println!("{}", msg);
    query_user_for_value::<T>()
}

pub fn query_user_for_value<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut input_buffer = String::new();

    stdin()
    .read_line(&mut input_buffer)
    .expect("Failed to read input.");

    // trim() shaves off linebreak-symbol, which buggers up the parsing
    input_buffer.trim().parse::<T>()
}