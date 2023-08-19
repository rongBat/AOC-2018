use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // split input string into lines
    let lines = input.lines().collect::<Vec<_>>();

    let mut totals= (0, 0);

    for line in lines{
        // split line into iterator of Char
        let chars_iter = line.chars();

        // use a Hashmap with the letter as the key, count as value
        let mut char_counts = HashMap::new();

        // count how many times each character appears
        for c in chars_iter {
            if let Some(entry) = char_counts.get_mut(&c) {
                // increment if found
                *entry += 1;
            } else {
                // else default to 1
                char_counts.entry(c).or_insert(1);
            }
        }

        // return (number of 2x chars) * (number of 3x chars)
        // check if any letters found twice, or thrice
        let found_twice: bool = char_counts.iter().filter(|&(_k, v)| *v == 2 ).count() > 0;
        let found_thrice: bool = char_counts.iter().filter(|&(_k, v)| *v == 3 ).count() > 0;
        // dbg!(line, char_counts, found_twice, found_thrice);

        if found_twice {totals.0 += 1};
        if found_thrice {totals.1 += 1};
    }
                                                
    Some(totals.0 * totals.1)
}

/// Returns the count of letters which do not match between words one and two
fn check_similarity(one: &str, two: &str) -> u8 {
    let mut num_diff_letters = 0;

    for (i, el) in one.chars().enumerate() {
        if el == two.chars().nth(i).unwrap_or(' ') {
            continue;
        } else {
            num_diff_letters += 1;
        }
    }

    return num_diff_letters;
}

fn get_similar_letters(one: &str, two: &str) -> String {
    let mut similar_letters: String = String::new();

    for (i, el) in one.chars().enumerate() {
        if el == two.chars().nth(i).unwrap_or(' ') {
            similar_letters.push(el)
        } else {
            continue;
        }
    }

    return similar_letters;
}

pub fn part_two(input: &str) -> Option<String> {
    // check each line against each other line
    // the check is how similar are they; are they just one character different in one spot?

    let mut found_box_ids = ("", "");

    // split input string into lines
    let lines = input.lines().collect::<Vec<_>>();
    'outer: for line1 in &lines {
        for line2 in &lines {
            if line1 == line2 {continue};
            
            if check_similarity(line1, line2) == 1 {
                // Found the Correct Box IDs!
                found_box_ids = (line1, line2);
                break 'outer;
            }
        }
    }

    Some(get_similar_letters(found_box_ids.0, found_box_ids.1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
