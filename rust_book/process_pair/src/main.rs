use std::{
    thread,
    time::{Instant, Duration},
    env,
    process::Command,
    net::UdpSocket,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let backup_target = "127.0.0.1:20001";

    if args.len() > 1 {
        thread::sleep(Duration::from_secs(2));
        spawn_new();
        main_process(backup_target);
    } else {
        let _ = backup_process(backup_target);
        println!("TAKING OVER");
        spawn_new();
        main_process(backup_target);
    }
}

fn spawn_new() {
    let new_program = "cargo run; echo 'Press ENTER to close'; read;";

    let _ = Command::new("gnome-terminal")
        .arg("--").arg("sh").arg("-c").arg(new_program).status().expect("fail spawn");
}

fn main_process(target: &str) {
    for i in 1..10 {
        thread::sleep(Duration::from_secs(2));
        let _ = send_udp(target, i);
    }
}

fn backup_process(socket_addr: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind(socket_addr)?;
    let mut buf = [0u8; 1024];
    let mut last_recv = Instant::now();

    loop {
        socket.set_read_timeout(Some(Duration::from_secs(2)))?;
        match socket.recv_from(&mut buf) {
            Ok((amt, src_addr)) => {
                last_recv = Instant::now();
                println!("Received {} bytes from {}: {:?}", amt, src_addr, &buf[..amt]);
            }
            Err(_) if last_recv.elapsed() >= Duration::from_secs(2) => return Ok(()),
            Err(_) => continue,
        }
    }
}

fn send_udp(target: &str, num: u8) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let buf = [num];

    socket.send_to(&buf, target)?;
    println!("Sent {num} to {target}");
    Ok(())
}
