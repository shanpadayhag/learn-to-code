use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let words: Vec<String> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().trim().to_string())
        .collect();

    let mut common_letters = String::from("");
    for character_index in 0..=9999 {
        for index in 0..=(n - 1) {
            if index == n - 1 {
                break;
            }

            let word1 = &words[index];
            let word2 = &words[index + 1];
            let character1 = word1.chars().nth(character_index);
            let character2 = word2.chars().nth(character_index);

            if let Some(character1) = character1 {
                if let Some(character2) = character2 {
                    if character1 == character2 {
                        if index == n - 2 {
                            common_letters.push(character1);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", common_letters);
}
