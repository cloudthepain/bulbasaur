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
    attempt_to_ssh(&ip_addr);
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
            .stdout(std::process::Stdio::null())
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
    .stdout(std::process::Stdio::null())
    .status()
    .expect("Failed to locate bash");

    if status.success()
    {
        println!("Bash present");
    }
}

fn attempt_to_ssh(ip_addr: &str)
{
    let ssh_user_name = "username";
    let status = Command::new("ssh")
            .args([ssh_user_name, ip_addr])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("failed")
            .wait()
            .expect("failed during wait");

    if status.success()
    {
        println!("SSH failed to connect")
    }
    if !status.success()
    {
        println!("You aren't able to run from this battle!")
    }
}