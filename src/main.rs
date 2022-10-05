use std::io::Write;

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

    for port in quty_ports {
        ports.push(serialport::new(port, 9600).open().unwrap());
    }
    
    let n_ports = ports.len();
    println!("Found {} QUTy board(s).", n_ports);

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
