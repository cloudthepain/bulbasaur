//cargo new, run build, & check
// mut allows variable to be mutable
use std::process::Command;
use std::io;

fn main() {

    let mut ip_addr = String::new();

    io::stdin()
        .read_line(&mut ip_addr)
        .expect("invalid");
    let ip_addr = ip_addr.trim();

    ping(ip_addr);
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