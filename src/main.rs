use std::env;
use std::io::{Write, Read};
use std::thread;

use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut interval = tokio::time::interval(Duration::from_millis(3998));

    let ports = serialport::available_ports().unwrap();
    let mut quty_ports = Vec::new();

    for port in ports {
        match port.port_type {
            serialport::SerialPortType::UsbPort(info) => 
                if (info.pid == 0xEA60) & (info.vid == 0x10C4) {
                    quty_ports.push(port.port_name);
                },
            _ => {}
        }
    }

    let mut ports = Vec::new();
    let mut rx_ports = Vec::new();

    for port in quty_ports {
        let serial = serialport::new(port, 9600).open().unwrap();
        rx_ports.push(serial.try_clone().unwrap());
        ports.push(serial);
    }

    let n_ports = ports.len();
    println!("Found {} QUTy board(s).", n_ports);

    // Check if -r flag is present
    if env::args().any(|v| v == "-r") {
        // Receive Thread
        thread::spawn(move || {
            loop {
                for (i, rx_port) in rx_ports.iter_mut().enumerate() {
                    let mut buffer = [0u8; 64]; // 64 byte buffer
                    if let Ok(n) = rx_port.read(&mut buffer) {
                        if n > 0 {
                            let s = String::from_utf8_lossy(&buffer[0..n]);
                            println!("RX{}: {}", i, s);
                        }
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    let mut tictoc = true;

    loop {
        interval.tick().await;

        // CMD_SYNC
        for i in 0..n_ports {
            if ports[i].write(&['\\' as u8, '\\' as u8, 'y' as u8]).is_err() {
                println!("ERROR: Failed to write to {:}.", ports[i].name().unwrap());
            }
        }    
        
        if tictoc {println!("TICK!")} else {println!("TOCK!")};
        tictoc = !tictoc;       
    }   
}
