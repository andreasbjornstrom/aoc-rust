fn main() {
    let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let data = aoc::get_aoc_file(4);
    let sum : usize = data.split("\n")
        .map(parse_things_part1)
        .sum();
    let sum_part2 : usize = data.split("\n")
        .map(parse_things_part2)
        .sum();
    println!("part1 {:?}", sum);
    println!("part2 {:?}", sum_part2)
}

fn parse_things_part2(row: &str) -> usize {
    if row.is_empty() {
        return 0
    }
    let parts = row.split(",").collect::<Vec<&str>>();
    let p1_range = parts[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let p2_range = parts[1].split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    for i in p1_range[0]..p1_range[1] +1{
        for j in p2_range[0]..p2_range[1] +1 {
            if i == j {
                return 1
            }
        }
    }
    return 0
}
fn parse_things_part1(row: &str) -> usize{
    if row.is_empty() {
        return 0
    }
    let parts = row.split(",").collect::<Vec<&str>>();
    let p1_range = parts[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let p2_range = parts[1].split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut state = false;
    if p1_range[0] >= p2_range[0] && p1_range[1] <= p2_range[1] {
        state = true;
        return 1

    }
    if p2_range[0] >= p1_range[0] && p2_range[1] <= p1_range[1] {
        state = true;
        return 1
    }
    return 0
}

fn is_within(){

}