//cargo new, run build, & check
// mut allows variable to be mutable
use regex::Regex;
use std::io;


fn main() {

    let mut ip_address = String::new();

    io::stdin().read_line(&mut ip_address).expect("Failed to read line");
    valid_ip(&ip_address);
}

fn valid_ip(ip_value: &String)
{

    let input_text = ip_value;

    if(input_text.len() >= 17)
    {
        panic!("Too Long!")
    }

    let name_regex = Regex::new(r#"(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}"#);

    if name_regex.is_err()
    {
        panic!("Error in regex pattern!");
    };
 
    let match_result: bool = name_regex.unwrap().is_match(input_text);

    println!("{match_result}");

}
