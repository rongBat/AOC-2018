pub fn part_one(input: &str) -> Option<i32> {
    //need to add inputs to total

    //break long string into array of lines
    let lines = input.lines().collect::<Vec<_>>();

    //collecting each line into a vector of i32
    let nums: Vec<i32> = lines
        .iter()
        .map(|&l| match l.parse() {
            Ok(i) => i,
            Err(_) => 0,
        })
        .collect();

    //add each element of array to total as i32
    let total = nums.iter().copied().reduce(|a, b| a + b);

    total
}

pub fn part_two(input: &str) -> Option<i32> {
    //break long string into array of lines
    let lines = input.lines().collect::<Vec<_>>();

    //collecting each line into a vector of i32
    let nums: Vec<i32> = lines
        .iter()
        .map(|&l| match l.parse() {
            Ok(i) => i,
            Err(_) => 0,
        })
        .collect();

    let mut old_freqs = vec![];
    let mut total = 0;
    let mut double_found = false;
    
    // handle wrap around
    while !double_found {
        for num in &nums {
            // calculate new total
            total += num;

            //check if new total repeats
            if old_freqs.contains(&total) {
                double_found = true;
                break;
            }

            //adding the total to old_freqs
            old_freqs.push(total);
        }
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(0));
    }
}
