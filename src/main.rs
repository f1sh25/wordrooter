use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::io::{self};
use std::io::{Error, ErrorKind};

#[cfg(test)]
mod tests;

const FINNISH_WORDLIST_CONTENT: &str = include_str!("../data/kaikkisanat.txt");

fn load_wordlist_from_str(wordlist_content: &str) -> HashSet<String> {
    wordlist_content
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| s.chars().all(|c| c.is_alphabetic()))
        .collect()
}

fn sorted_letters(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.iter().collect()
}

fn count_letters(word: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in word.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn find_chain(
    current: &str,
    used_letters: HashMap<char, usize>,
    target_letters: &HashMap<char, usize>,
    anagram_index: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    let key = sorted_letters(current);
    if let Some(cached) = memo.get(&key) {
        return cached.clone();
    }

    if &used_letters == target_letters {
        return vec![current.to_string()];
    }

    let mut longest_chain = Vec::new();

    for (&letter, &target_count) in target_letters {
        let used_count = used_letters.get(&letter).copied().unwrap_or(0);
        if used_count < target_count {
            let mut next_letters: HashMap<char, usize> = used_letters.clone();
            *next_letters.entry(letter).or_insert(0) += 1;

            let mut expanded: Vec<char> = next_letters
                .iter()
                .flat_map(|(&c, &n)| std::iter::repeat(c).take(n))
                .collect();
            expanded.sort_unstable();
            let expanded_key: String = expanded.iter().collect();

            if let Some(words) = anagram_index.get(&expanded_key) {
                for word in words {
                    if word == current {
                        continue;
                    }

                    let chain = find_chain(
                        word,
                        next_letters.clone(),
                        target_letters,
                        anagram_index,
                        memo,
                    );
                    if chain.len() > longest_chain.len() {
                        longest_chain = chain;
                    }
                }
            }
        }
    }

    if !longest_chain.is_empty() {
        let mut full_chain = vec![current.to_string()];
        full_chain.extend(longest_chain);
        memo.insert(key, full_chain.clone());
        full_chain
    } else {
        memo.insert(key, vec![current.to_string()]);
        vec![current.to_string()]
    }
}

fn wordrooter(start_word: String, available_chars: String) -> Result<Vec<String>, std::io::Error> {
    let start: String = start_word.trim().to_lowercase();
    let available: String = available_chars.trim().to_lowercase();

    let wordlist = load_wordlist_from_str(FINNISH_WORDLIST_CONTENT);

    if !wordlist.contains(&start) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Start word is not in the wordlist.",
        ));
    }

    let available_counts = count_letters(&available);
    let used_counts = count_letters(&start);

    let mut anagram_index: HashMap<String, Vec<String>> = HashMap::new();
    for word in &wordlist {
        let key = sorted_letters(word);
        anagram_index.entry(key).or_default().push(word.clone());
    }

    let mut memo = HashMap::new();
    let chain = find_chain(
        &start,
        used_counts,
        &available_counts,
        &anagram_index,
        &mut memo,
    );

    Ok(chain)
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Finds the longest word chain from a starting word using available letters. Program is intended to crack HS.fi sanajuuri game and only works on finnish words", long_about = None)]
struct Args {
    #[arg(short, long)]
    start_word: String,

    #[arg(short, long)]
    available_letters: String,
}

fn main() -> io::Result<()> {
    let ascii_art = r#"
    _    _               _______            _            
    | |  | |             | | ___ \          | |           
    | |  | | ___  _ __ __| | |_/ /___   ___ | |_ ___ _ __ 
    | |/\| |/ _ \| '__/ _` |    // _ \ / _ \| __/ _ \ '__|
    \  /\  / (_) | | | (_| | |\ \ (_) | (_) | ||  __/ |   
    \/  \/ \___/|_|  \__,_\_| \_\___/ \___/ \__\___|_|   
                                                        
    "#;

    println!("{}", ascii_art);

    let args = Args::parse();

    match wordrooter(args.start_word, args.available_letters) {
        Ok(result) => {
            println!("\nLongest valid chain:");
            for (i, word) in result.iter().enumerate() {
                println!("{}. {}", i + 1, word);
            }
            println!("Total steps: {}", result.len());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
