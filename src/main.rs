//cargo new, run build, & check
// mut allows variable to be mutable
use std::process::Command;
use std::io;

fn main() {

    const HOST_OS: &str = std::env::consts::OS;

    println!("{}",HOST_OS);


    get_terminal_controllers();


    let ip_addr = getuserinput();

    ping(&ip_addr);
}

fn getuserinput() -> String
{
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("invalid");

    let temp_trim_val= value.trim();
    let trim_value = temp_trim_val.to_string();
    return trim_value;
}

fn ping(ip_address_to_ping: &str)
{

    let x = "-c 4";
    let status = Command::new("ping")
            .args([ip_address_to_ping, x])
            .spawn()
            .expect("failed")
            .wait()
            .expect("failed during wait");

    if status.success()
    {
        println!("Completed Successfully Pokemon Master")
    }
}

fn get_terminal_controllers()
{
    let status = Command::new("bash")
    .arg("--version")
    .spawn()
    .expect("failed")
    .wait()
    .expect("failed during wait");

    if status.success()
    {
        println!("We're in")
    }
}