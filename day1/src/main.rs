extern crate core;

use std::any::Any;
use std::fmt::{Debug, format};
use std::{env, fs};
use std::borrow::Borrow;
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
fn main() {
    let winner = find_winner(&read_test_file());
    assert!(winner.sum.eq(&24000));

    let content = download_content_for_day("1".to_string());
    let winner = find_winner(&content);
    debug(&winner);
    let top_three = sum_of_three_top(&content);
    debug(&top_three)
}

fn sum_of_three_top(content: &String) -> i32 {
    let mut sums :Vec<i32> = parse_rows(content).into_iter()
        .map(|row| -> i32 { sum_row(&row)})
        .collect();
    sums.sort_by(|a, b|  b.partial_cmp(a).unwrap());

    return sums.get(0).unwrap() + sums.get(1).unwrap() + sums.get(2).unwrap()
}

// session=53616c7465645f5fd106dc0e23e58fda43ffdceb8f92ec17e69b4a0f4c52330f1df4a4ca19af291194174a8698212c268431274770135032c992ea8b9f2de51e
fn download_content_for_day(day1: String) -> String {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&*format! {"https://adventofcode.com/2022/day/{day1}/input"}).unwrap();
    handle.http_headers(configureHeaders()).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("{:?}", data);
    return str::from_utf8(&data).map(String::from).unwrap();
}

fn configureHeaders() -> List {
    let mut headers = List::new();
    let cookie = fs::read_to_string("cookie").expect("TODO: panic message");
    headers.append(&*format!("Cookie: {}", cookie)).expect("TODO: panic message");
    headers
}

fn find_winner(content: &String) -> Elf {
    let rows = parse_rows(content);

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

fn parse_rows(content: &String) -> Vec<Vec<i32>> {
    let rows: Vec<Vec<i32>> = content.split("\n\n")
        .map(String::from)
        .map(|prylar: String| -> Vec<i32>{
            prylar.split('\n')
                .map(String::from)
                .filter(|s| -> bool { !s.is_empty() })
                .map(|s: String| -> i32 { s.parse::<i32>().expect(" panic?! wtf?") })
                .collect()
        })
        .collect();
    rows
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

fn read_test_file() -> String {
    let file_path = "src/resources/input";
    let content = fs::read_to_string((file_path))
        .expect("Should have been able to read the file?!");
    println!("file content:\n {content}");
    return content;
}
