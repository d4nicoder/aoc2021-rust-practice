use std::borrow::Borrow;
use std::collections::HashMap;

struct BitMap {
    one: usize,
    zero: usize,
}

pub fn run1() -> usize {
    let lines = super::utils::read_file("src/day3/input.txt", false);

    let gamma_string: String = find_most_common(&lines);
    let epsilon_string: String = find_less_common(&gamma_string);
    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_string, 2).unwrap();
    return gamma * epsilon;
}

fn count_bits(list: &Vec<String>) -> Vec<BitMap> {
    let mut bit_map: Vec<BitMap> = Vec::new();

    for _ in 0..list[0].len() {
        bit_map.push(BitMap { one: 0, zero: 0 });
    }
    for line in list {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                bit_map[i].one += 1;
            } else {
                bit_map[i].zero += 1;
            }
        }
    }
    return bit_map;
}

fn find_most_common(lines: &Vec<String>) -> String {
    let counts = count_bits(lines);
    let mut common_string = String::new();
    for (_, map) in counts.iter().enumerate() {
        if map.one >= map.zero {
            common_string.push('1');
        } else {
            common_string.push('0');
        }
    }

    return common_string;
}

fn find_less_common(most_common: &String) -> String {
    let mut less_common: String = String::new();

    for i in most_common.chars() {
        if i == '1' {
            less_common += "0";
        } else {
            less_common += "1";
        }
    }

    return less_common;
}

pub fn run2() -> usize {
    let lines = super::utils::read_file("src/day3/input.txt", false);

    let mut filtered_oxygen: Vec<String> = lines.clone();
    let mut filtered_co2: Vec<String> = lines.clone();

    for i in 0..lines.len() {
        let common = find_most_common(&filtered_oxygen);
        let less_common = find_less_common(&find_most_common(&filtered_co2));
        if filtered_oxygen.len() > 1 {
            filtered_oxygen = filtered_oxygen
                .iter()
                .filter(|&x| x.chars().nth(i) == common.chars().nth(i))
                .map(|x| x.to_string())
                .collect();
        }
        if filtered_co2.len() > 1 {
            filtered_co2 = filtered_co2
                .iter()
                .filter(|&x| x.chars().nth(i) == less_common.chars().nth(i))
                .map(|x| x.to_string())
                .collect();
        }
    }

    let oxygen = usize::from_str_radix(&filtered_oxygen[0], 2).unwrap();
    let co2 = usize::from_str_radix(&filtered_co2[0], 2).unwrap();

    return oxygen * co2;
}
