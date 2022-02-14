/*
** SPIJKERMAN NICOLAS PROJECT, 2021
**   boursor_algo_ns
** File description:
**   main.rs
*/

#![allow(while_true)]
#![allow(unused)]

use std::{thread, time::Duration};
use std::io::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use chrono;

fn read_io() -> bool {
    let start: String = String::from("y");
    let stop: String = String::from("n");

    io::stdout().flush().unwrap();
    let mut input = String::new();

    // Get input
    io::stdin().read_line(&mut input);
    if input.trim().eq(&start) {
        return false;
    }
    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut loop_it: i32 = 0;
    let loop_max: i32 = 10;

    // HTML request historic
    let data = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/GetTicksEOD?symbol=1rPRNO&length=1&period=0&guid=")?.text()?;
    println!("|| HTML request - Data   OK !");

    // HTML request now
    let mut buffer = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/UpdateCharts?symbol=1rPRNO&period=-1")?.text()?;
    println!("|| HTML request - Buffer OK !");

    // Write historical data
    fs::write("data.json", data).expect("Unable to write file");
    println!("|| data.json    - loaded");

    // Write recent data
    fs::write("out.json", buffer).expect("Unable to write file");
    println!("|| out.json     - loaded");

    print!("|| Start algo ? ([y]es/[n]o) : ");
    if read_io() {
        println!("|| STOP.");
    } else {
        // Start main loop
        while loop_it < loop_max {
            // HTML request recenr data
            print!("   \tRequest: ");
            buffer = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/UpdateCharts?symbol=1rPRNO&period=-1")?.text()?;
            print!("OK !");

            // Write into out.json
            fs::write("out.json", buffer).expect("Unable to write file");
            println!(" -> out.json loaded | {:?}", chrono::offset::Local::now());
            loop_it += 1;
            // Sleep for [x] secs
            thread::sleep(Duration::from_secs(5));
        }
    }
    println!("|| EXIT.");
    Ok(())
}
