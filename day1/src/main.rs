extern crate core;

use std::any::Any;
use std::fmt::{Debug, format};
use std::{env, fs};
use std::fs::{copy, File};
use std::str;
use std::i32;
use std::io::{BufRead, Read, stdout, Write};
use std::sync::Arc;
use std::time::Duration;
use curl::easy::{Easy, List};

#[derive(Debug)]
struct Elf {
    index: i32,
    sum: i32,
}

// https://adventofcode.com/2022/day/1/input
// #[tokio::main(flavor = "current_thread")]
fn main() {
    let mut content = read_file();
    if env::args().len() > 1 {
        content = download_content_for_day("1".to_string())
    }

    let winner = find_winner(content);
    debug(&winner)
}

// session=53616c7465645f5fd106dc0e23e58fda43ffdceb8f92ec17e69b4a0f4c52330f1df4a4ca19af291194174a8698212c268431274770135032c992ea8b9f2de51e
fn download_content_for_day(day1: String) -> String {
    let mut handle = Easy::new();
    let mut buf = Arc::new(Vec::new());
    handle.url(&*format! {"https://adventofcode.com/2022/day/{day1}/input"}).unwrap();
    handle.http_headers(configureHeaders()).unwrap();
    handle.write_function(| data| {
        buf.extend_from_slice(data);
        //println!("{}", str::from_utf8(data).unwrap());
        // &response.push_str(str::from_utf8(data).unwrap());
        Ok(data.len())
    }).unwrap();

    handle.perform().unwrap();

//    debug(&buf);
    //   println!("{}", handle.response_code().unwrap());
    return str::from_utf8(&buf).map(String::from).unwrap()
}

fn configureHeaders() -> List {
    let mut headers = List::new();
    let cookie = fs::read_to_string("cookie").expect("TODO: panic message");
    headers.append(&*format!("Cookie: {}", cookie)).expect("TODO: panic message");
    headers
}

fn find_winner(content: String) -> Elf {
    let rows: Vec<Vec<i32>> = content.split("\n\n")
        .map(String::from)
        .map(|prylar: String| -> Vec<i32>{
            prylar.split('\n')
                .map(String::from)
                .map(|s: String| -> i32 { s.parse::<i32>().unwrap() })
                .collect()
        })
        .collect();
    debug(&rows);

    let mut winner: Elf = Elf { index: -1, sum: 0 };

    for i in 0..rows.len() {
        let row = rows.get(i).unwrap();
        let sum = sum_row(row);
        if sum > winner.sum {
            winner = Elf { index: i as i32, sum }
        }
    }
    return winner;
}

fn sum_row(row: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in row {
        sum += num
    }
    return sum;
}

fn debug<T: Any + Debug>(rows: &T) {
    println!("{:?}", rows);
}

fn read_file() -> String {
    let file_path = "src/resources/input";
    let content = fs::read_to_string((file_path))
        .expect("Should have been able to read the file?!");
    println!("file content:\n {content}");
    return content;
}
