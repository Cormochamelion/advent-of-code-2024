advent_of_code::solution!(5);

use std::collections::{HashMap, HashSet};

fn parse_rule_line(line: &str) -> (&str, &str) {
    return line.split_once("|").unwrap();
}

fn parse_print_line(line: &str) -> Vec<&str> {
    return line.split(",").collect();
}

fn update_hashmap(hash_map: &mut HashMap<String, HashSet<String>>, key: String, behind: String) {
    if !hash_map.contains_key(&key) {
        hash_map.insert(key.clone(), HashSet::new());
    }

    hash_map.get_mut(&key).unwrap().insert(behind.clone());
}

fn parse_input(input: &str) -> (HashMap<String, HashSet<String>>, Vec<Vec<&str>>) {
    let mut rule_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut print_vec: Vec<Vec<&str>> = Vec::new();

    let mut is_rule = true;

    for line in input.lines() {
        if line == "" {
            is_rule = false;
            continue;
        }

        if is_rule {
            let (key_str, behind_str) = parse_rule_line(line);
            update_hashmap(
                &mut rule_map,
                String::from(key_str),
                String::from(behind_str),
            );
        } else {
            print_vec.push(parse_print_line(line));
        }
    }

    return (rule_map, print_vec);
}

fn get_middle_job_value(job: &Vec<&str>) -> u32 {
    let middle = (job.len() as f32 / 2 as f32).ceil() - 1 as f32;
    let middle_str = job[middle as usize];

    return middle_str.parse::<u32>().unwrap();
}

fn job_conforms(job: &Vec<&str>, rule_map: &HashMap<String, HashSet<String>>) -> bool {
    let mut front_set: HashSet<String> = HashSet::new();
    let mut rule_set: &HashSet<String>;

    for (i, page) in job.iter().enumerate().rev() {
        front_set.clear();
        front_set.extend(
            job[0..i]
                .iter()
                .map(|page| String::from(*page))
                .collect::<HashSet<String>>(),
        );

        rule_set = rule_map.get(*page).unwrap();

        match front_set.intersection(rule_set).next() {
            Some(_) => return false,
            None => continue,
        }
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, print_queue) = parse_input(input);

    let mut print_sum: u32 = 0;

    for job in print_queue {
        if job_conforms(&job, &rules) {
            print_sum += get_middle_job_value(&job);
        }
    }

    Some(print_sum)
}

fn make_conforming<'a>(
    job: &Vec<&'a str>,
    rule_map: &HashMap<String, HashSet<String>>,
) -> Vec<&'a str> {
    let job_set: HashSet<String> = job
        .iter()
        .map(|page| String::from(*page))
        .collect::<HashSet<String>>();

    let mut rule_set: &HashSet<String>;
    let mut order_vec: Vec<(&str, u32)> = Vec::new();

    for page in job.iter() {
        rule_set = rule_map.get(*page).unwrap();

        order_vec.push((*page, rule_set.intersection(&job_set).count() as u32));
    }

    order_vec.sort_by(|a, b| a.1.cmp(&b.1));

    return order_vec.iter().map(|x| x.0).collect();
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, print_queue) = parse_input(input);

    let mut print_sum: u32 = 0;

    for job in print_queue {
        if !job_conforms(&job, &rules) {
            let conforming_job = make_conforming(&job, &rules);
            print_sum += get_middle_job_value(&conforming_job);
        }
    }

    Some(print_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
