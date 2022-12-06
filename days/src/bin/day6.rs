use std::collections::HashSet;

fn main() {
    let _data = "bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11";
    // part 2 - test
    let _data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26";

    let data = aoc::get_aoc_file(6);
    let sum_part1: usize = data.split('\n').map(|r| find_marker(r, 4)).sum();
    let sum_part2: usize = data.split('\n').map(|r| find_marker(r, 14)).sum();
    println!("part 1 {:?}", sum_part1);
    println!("part 2 {:?}", sum_part2);
    assert_eq!(1804, sum_part1);
    assert_eq!(2508, sum_part2)
}

fn find_marker(row: &str, length: usize) -> usize {
    if row.is_empty() {
        return 0;
    }
    let mut search_buffer: Vec<char> = Vec::new();
    let total_parsed_chars = row
        .chars()
        .take_while(|x| -> bool {
            search_buffer.push(*x);
            if search_buffer.len() > length {
                search_buffer.remove(0);
            }
            let buff_set: HashSet<&char> = HashSet::from_iter(search_buffer.iter());
            buff_set.len() != length
        })
        .collect::<String>();
    println!(
        "marker {:?}, position {:?}, row {:?}, ",
        search_buffer,
        total_parsed_chars.len() + 1,
        row
    );
    total_parsed_chars.len() + 1
}
