#![allow(warnings)]
extern crate config;

#[macro_use]
extern crate serde_derive;

mod app_config;

use app_config::AppConfig;
use std::env;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
// Multi Producer Single Consumer (mpsc)
use std::sync::mpsc::{Sender, channel};
use std::thread;
use config::ConfigError;
use core::borrow::Borrow;
use std::error::Error;


const MAX_PORTS: u16 = 65535;


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(err) => {
                println!("Unable to connect to port {} because of error {:?}", port, err);
            }

        }

        if (MAX_PORTS - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    // Benchmarking
    let start_time = Instant::now();

    // First set up the AppConfig
    let cfg_result = AppConfig::new();
    println!("{:?}", cfg_result);

    // Pattern match the result
    match cfg_result {
        // Rust concurrency is to share memory by communicating (instead of communicating by sharing memory).
        Ok(cfg) => {
            let (tx, rx) = channel();
            for i in 0..cfg.sniffer.num_threads {
                let tx = tx.clone();

                thread::spawn(move || {
                    scan(tx, i, cfg.sniffer.ipaddr, cfg.sniffer.num_threads);
                });
            }

            let mut out = vec![];
            drop(tx);
            for p in rx {
                out.push(p);
            }

            println!();
            out.sort();
            for v in out {
                println!("{} is open", v);
            }
        }
        Err(error) =>
            eprintln!("Something is fucked up because of {}", error)
    }
    let duration = start_time.elapsed();

    // Calculate how long it took to run the program
    println!("Time elapsed is: {:?}", duration);

}