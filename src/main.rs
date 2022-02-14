/*
** SPIJKERMAN NICOLAS PROJECT, 2021
** boursor_algo_ns
** File description:
** main.rs
*/

#![allow(while_true)]
#![allow(unused)]

use std::{thread, time::Duration};
use std::error::Error;
use std::fs;
use chrono;

fn main() -> Result<(), Box<dyn Error>> {
    // HTML request historic
    let data = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/GetTicksEOD?symbol=1rPRNO&length=1&period=0&guid=")?.text()?;
    println!("HTML request - Data OK !");

    // HTML request now
    let mut buffer = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/UpdateCharts?symbol=1rPRNO&period=-1")?.text()?;
    println!("HTML request - Buffer OK !");

    // Write historical data
    fs::write("data.json", data).expect("Unable to write file");
    println!("data.json loaded");

    // Write recent data
    fs::write("out.json", buffer).expect("Unable to write file");
    println!("out.json loaded\nSTART loop:");

    // Start main loop
    while true {
        // HTML request recenr data
        print!("\tRequest: ");
        buffer = reqwest::blocking::get("https://bourse.boursorama.com/bourse/action/graph/ws/UpdateCharts?symbol=1rPRNO&period=-1")?.text()?;
        print!("OK !");
        // Write into out.json
        fs::write("out.json", buffer).expect("Unable to write file");
        println!(" -> out.json loaded | {:?}", chrono::offset::Local::now());
        // Sleep for [x] secs
        thread::sleep(Duration::from_secs(60));
    }
    Ok(())
}
