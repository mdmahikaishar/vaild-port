mod arguments;

use std::{net::{IpAddr, SocketAddr, TcpStream}, sync::mpsc::{self, Sender}};
use arguments::Arguments;

enum ScanPort {
    Message(u16),
    Completed
}

const MAX_PORT: u16 = 65535;

fn scan_port(sender: Sender<ScanPort>, start_port: u16, ip_address: IpAddr, threads: u16) {
    let mut current_port = start_port;

    while (MAX_PORT - current_port) >= threads {
        let address = SocketAddr::from((ip_address, current_port));
        if let Ok(_) = TcpStream::connect(address) {
            sender.send(ScanPort::Message(current_port)).unwrap();
        }

        current_port += threads;
    }

    sender.send(ScanPort::Completed).unwrap();
}

fn main() {
    println!("Starting port scanning...");
    
    let args = Arguments::new(std::env::args().collect()).unwrap();  
    let (sender, receiver) = mpsc::channel::<ScanPort>();
    let mut thread_handles = Vec::new();
    let mut completed_threads: u16 = 0;

    // create threads
    for i in 1..=args.threads {
        let sender = sender.clone();
        let handle = std::thread::spawn(move || {
            scan_port(sender, i, args.ip_address, args.threads);
        });
        thread_handles.push(handle);
    }

    // receive port
    for receive in receiver {
        match receive {
            ScanPort::Message(port) => {
                println!("{:<6} is open", port);
            }
            ScanPort::Completed => {
                completed_threads += 1;
            }
        }

        if completed_threads >= args.threads {
            break;
        }
    }

    // join threads
    for hanlde in thread_handles {
        hanlde.join().unwrap();
    } 

    println!("Port scanning completed.");
}
