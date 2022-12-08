use std::collections::HashMap;
use std::iter::Map;
use std::ops::Sub;

fn main() {
    let data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    //let day = 0;
    let data = aoc::get_aoc_file(7);
    let sum: HashMap<String, usize> = parse_things(&data);

    let sum_part1: usize = part_1(&sum);
    let sum_part2: usize = part_2(&sum);
    println!("part 1 {:?}", sum_part1);
    println!("part 2 {:?}", sum_part2)
}

fn parse_things(row: &str) -> HashMap<String, usize> {
    let mut current_dir: Vec<&str> = Vec::new();
    let mut dir_content: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for x in row.lines() {
        // if command
        if x.starts_with("$ ls") {
            // nothing really..
        } else if x.starts_with("$ cd") {
            let (_, dir) = x[2..].split_once(' ').unwrap();
            if let Some(working_dir) = current_dir.last() {
                println!("cd into {working_dir}/{dir}");
            } else {
                println!("cd into {dir}");
            }
            if dir == ".." {
                current_dir.pop();
            } else if dir == "/" {
                current_dir.clear();
                current_dir.push("/")
            } else {
                current_dir.push(dir)
            }
        } else {
            let working_dir = current_dir.join("/");
            let file = if x.starts_with("dir ") {
                let (_, dir_name) = x[3..].split_once(" ").unwrap();
                let full_path = format!("{working_dir}/{dir_name}");
                (0_usize, full_path)
            } else {
                let (size, file_name) = x.split_once(' ').unwrap();
                (size.parse::<usize>().unwrap(), file_name.to_string())
            };
            println!("procesing file {:?}", file);
            match dir_content.get_mut(&working_dir) {
                None => {
                    dir_content.insert(working_dir, vec![file]);
                }
                Some(content) => content.push(file),
            }
        }
    }

    let mut folder_size: HashMap<String, usize> = HashMap::new();
    for folder in &dir_content {
        let mut total = 0;
        for file in folder.1 {
            if file.0 == 0 {
                total += get_folder_size(&dir_content, &file.1)
            }
        }
        let size = folder.1.iter().map(|x| x.0).sum::<usize>();
        folder_size.insert(folder.0.clone(), size + total);
    }

    return folder_size;
}
fn part_2(folder_size: &HashMap<String, usize>) -> usize {
    let available_disk = 70_000_000 as usize;
    let unused_needed = 30_000_000 as usize;
    let root_usage = folder_size.get(&"/".to_string()).unwrap();
    let min_size_to_delete = unused_needed.sub(available_disk.sub(root_usage));
    let mut deleteable_folder = folder_size
        .iter()
        .filter(|(_, v)| v.ge(&&min_size_to_delete))
        .map(|(_, v)| v)
        .collect::<Vec<&usize>>();
    deleteable_folder.sort();
    return **deleteable_folder.first().unwrap();
}

fn part_1(folder_size: &HashMap<String, usize>) -> usize {
    let mut total: usize = 0;
    for folder in folder_size {
        if folder.1 <= &100_000 {
            total += folder.1
        }
    }

    return total;
}

fn get_folder_size(p0: &HashMap<String, Vec<(usize, String)>>, p1: &String) -> usize {
    let mut total = 0;
    for (size, file) in p0.get(&*p1).unwrap() {
        if size == &0 {
            total += get_folder_size(p0, file)
        }
        total += size;
    }
    total
}
