//cargo new, run build, & check
// mut allows variable to be mutable
use std::process::Command;
use std::io;

fn main() {

    createstartmenu();
}

fn createstartmenu()
{
    println!("What phase are you in?");
    println!("1: Recon");
    println!("2: Not Recon");
    println!("3: Third Option");
    println!("Option: ");

    let value = getuserinput();
    let num:i32 = value.trim().parse().expect("Failed to parse int");
    if num == 1
    {
        ping();
    }
    else if num == 2
    {
        attempt_to_ssh();
    }
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

fn ping()
{

    println!("Enter IP to Recon: ");
    let ip_address_to_ping = &getuserinput();
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

        println!("Ping Completed Successfully");
        std::process::exit(status.code().unwrap());
    }
    else
    {
        println!("Could not successfully ping!");
        std::process::exit(status.code().unwrap());
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

fn attempt_to_ssh()
{

    print!("Enter ip to SSH into: ");
    let y = getuserinput();


    let ip_addr = y.trim();
    
    
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