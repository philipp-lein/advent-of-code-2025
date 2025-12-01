use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut actions: Vec<String> = Vec::new();

    for line in reader.lines() {
        actions.push(line?);
    }

    println!("Finished reading file.");

    let password = get_password(&actions);
    println!("Password: {}", password);

    Ok(())
}

// Extract only the first character from each action line
// Count how often the index is 0 after applying all actions
fn get_password(actions: &[String]) -> i32 {
    let mut result = 0;
    let mut actual_index: i32 = 50; // start position, will be kept in 0..=98

    for action in actions {
        let action = action.trim();
        if action.is_empty() {
            continue;
        }

        // Split into direction and number
        let (dir, num_str) = action.split_at(1);
        let first_char = dir.chars().next().unwrap();
        let steps: i32 = num_str.parse().expect("Failed to parse number");

        // Normalize steps into 0..=98 (optional, aber sauber)
        let steps_mod = steps.rem_euclid(100);

        match first_char {
            'L' => {
                actual_index = (actual_index - steps_mod).rem_euclid(100);
            }
            'R' => {
                actual_index = (actual_index + steps_mod).rem_euclid(100);
            }
            _ => {
                // Unknown instruction, skip or handle error
                continue;
            }
        }

        println!("Index after {}: {}", action, actual_index);

        if actual_index == 0 {
            result += 1;
        }
    }

    result
}
