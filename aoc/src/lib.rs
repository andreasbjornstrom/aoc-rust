use std::fs;
use std::path::PathBuf;
use std::string::ToString;
use curl::easy::{Easy, List};
use dirs;

static FILE_PREFIX: &str = "large_file_";

pub fn get_aoc_file(day: usize) -> String {
    let file_name = format!("{FILE_PREFIX}_{day}");
    let file = fs::read_to_string(&file_name);
    return if file.is_err() {
        download_aoc_file(day, &file_name);
        get_aoc_file(day)
    } else {
        file.unwrap()
    };
}

fn get_cookie() -> String {
    let mut cookie_file: PathBuf = dirs::home_dir().expect("Unable to find user home dir!");
    cookie_file.push(".aoc_cookie");
    dbg!(&cookie_file);
    let cookie = fs::read_to_string(cookie_file)
        .expect("unable to read cookie file from, {}");
    return cookie.strip_suffix("\n").unwrap().to_string();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn download_aoc_file(day: usize, file_name: &str) {
    println!("Downloading aoc file for day {:?}", &day);
    let mut handle = Easy::new();
    let mut data = Vec::new();
    handle
        .url(&*format! {"https://adventofcode.com/2022/day/{day}/input"})
        .unwrap();
    handle.http_headers(create_headers()).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    fs::write(file_name, data).unwrap()
}

fn create_headers() -> List {
    let mut headers = List::new();
    headers
        .append(&*format!("Cookie: {}", get_cookie()))
        .unwrap();
    dbg!(&headers);
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_exists() {
        get_cookie();
    }

    #[test]
    fn download_works() {
        download_aoc_file(1, "/tmp/aoc_temp_1");
    }
}
