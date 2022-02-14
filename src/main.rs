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

// Candle struct
struct Candle {
    d: i32,
    o: i32,
    h: i32,
    l: i32,
    c: i32,
    v: i32,
}

// Historic struct
struct Data {
    name: String,
    symbol_id: String,
    x_period: i32,
    qv: Candle,
    qd: Candle,
    // Number of Candle type element = 327 (in data.json) :
    quote_tab: [Candle; 327],
}

// Struct for out.json data
struct Out {
    o: i32,
    h: i32,
    l: i32,
    c: i32,
    v: i32,
    var: i32,
    qt: [Candle; 2],
}

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

    // Struct
    let mut data_s: Data;
    let mut candle_s: Candle;

    // HTML request historic
    let mut data: std::string::String = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/GetTicksEOD?symbol=1rPRNO&length=1&period=0&guid=")?.text()?;
    println!("|| HTML request - Data   OK !");

    // HTML request now
    let mut buffer: std::string::String = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/UpdateCharts?symbol=1rPRNO&period=-1")?.text()?;
    println!("|| HTML request - Buffer OK !");

    // Write historical data
    fs::write("data.json", data).expect("Unable to write file");
    println!("|| data.json    - loaded");

    // Write recent data
    fs::write("out.json", buffer).expect("Unable to write file");
    println!("|| out.json     - loaded");

    // Fill data structs
    let data_buffer = fs::read_to_string("data.json")
        .expect("Something went wrong reading the file");

    // Fill candle structs
    let out_buffer = fs::read_to_string("out.json")
        .expect("Something went wrong reading the file");

    print!("|| Start algo ? ([y]es/[n]o) : ");
    // If input == n (no), read_io = true -> exit
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
