fn main() {
    let data = "";
    //let day = 0;
    // let data = aoc::get_aoc_file(day);
    let sum_part1: usize = data.split("\n")
        .map(parse_things_part1)
        .sum();
    let sum_part2 : usize = data.split("\n")
        .map(parse_things_part2)
        .sum();
    println!("part 1 {:?}", sum_part1);
    println!("part 2 {:?}", sum_part2)
}

fn parse_things_part2(row: &str) -> usize {
    if row.is_empty() {
        return 0
    }
    return 0
}
fn parse_things_part1(row: &str) -> usize{
    if row.is_empty() {
        return 0
    }
    return 0
}
