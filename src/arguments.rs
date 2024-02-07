use std::{net::IpAddr, str::FromStr};

pub struct Arguments {
  pub threads: u16,
  pub ip_address: IpAddr,
}

impl Arguments {
    pub fn new(args: Vec<String>) -> Result<Arguments, &'static str> {
        // valid-port
        if args.len() == 1 {
            return Err("Not enough arguments.");
        } else if args.len() > 4 {
            return Err("Too many arguments.");
        }

        let mut threads = 500;

        // valid-port <ip_address>
        if let Ok(ip_address) = IpAddr::from_str(&args[1]) {
            return Ok(Arguments { threads, ip_address });
        }
        // valid-port <--help ||  -h>
        else if args[1].contains("-h") || args[1].contains("--help") {
            if args.len() > 2 {
                return Err("Too many arguments.");
            }

            println!(
                "Usage: -j number of threads.
                \r\n   -h or --help for more information."
            );
            return Err("help");
        }
        // valid-port -j <thread_number>
        else if args[1].contains("-j") {  
            if let Ok(threads_number) = args[2].parse::<u16>() {
                threads = threads_number;
            } else {
                return Err("Invalid number of threads.");
            }

            // valid-port -j <thread_number> <ip_address>
            if let Ok(ip_address) = IpAddr::from_str(&args[3]) {
                return Ok(Arguments { threads, ip_address });
            }
        }
        
        return Err("Invalid arugments.
      \r\nvalid-port --help");
    }
}

