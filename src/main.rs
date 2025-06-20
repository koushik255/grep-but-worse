use clap::{command, Arg};
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::error::Error;

type MatchedLine = (String, usize, String);

fn lt() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread");
        thread::sleep(Duration::from_secs(1));
        println!("Thread finished");
    });
    handle.join().unwrap();
    println!("thread all threads done");
}

fn input() -> String {
    let matches = command!()
        .arg(Arg::new("input").help("Your help input").required(true))
        .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("input is required")
        .trim();

    println!("here was your input {:?}", input);
    input.to_string()
}

fn list_files() -> io::Result<Vec<String>> {
    fs::read_dir(".")?
        // its a map for the dir then another map for the files in the dir and them dispkaying them
        .map(|res| res.map(|e| e.path().display().to_string()))
        .collect()
}

fn read_files(word_search: String) -> Result<Vec<MatchedLine>, Box<dyn Error>> {
    let files = list_files()?;
    let files2 = Arc::new(files);
    let current_index = Arc::new(Mutex::new(0));
    let result = Arc::new(Mutex::new(Vec::new())); // Mutex for results
    let mut handles = vec![];

    for thread_id in 0..2 {
        let files_clone = Arc::clone(&files2);
        let index_clone = Arc::clone(&current_index);
        let result_clone = Arc::clone(&result);
        let word_search_clone = word_search.clone();

        let handle = thread::spawn(move || {
            loop {
                let file_path = {
                    let mut index = index_clone.lock().unwrap();
                    if *index >= files_clone.len() {
                        break;
                    }
                    let current = *index;
                    *index += 1;
                    // just returns the next file to read for a thread
                    files_clone[current].clone()
                    };

                println!("Thread {} processing {}", thread_id, file_path);

                let contents = fs::read_to_string(&file_path)
                    .expect("Failed to read message to string");
                let file_name = file_path.clone();
                
                // same as the old method
                let matched_lines: Vec<MatchedLine> = contents
                    .lines()
                    .enumerate()
                    .filter(|(_, line)| line.contains(&word_search_clone))
                    .map(|(line_number, line)| {
                        (file_name.clone(), line_number + 1, line.to_string())
                    })
                    .collect();
                //a
                result_clone.lock().unwrap().extend(matched_lines);
            }
        });
        //a
        handles.push(handle);
    }
//a
    for handle in handles {
        handle.join().unwrap();
    }
    // into inner so we have ownership
    // //a
    Ok(Arc::try_unwrap(result)
        .expect("Mutex still has multiple strong references")
        .into_inner()
        .expect("Mutex cannot be unwrapped"))
}

fn random_bs(input: String) -> Result<Vec<String>, String> {
    let files = list_files().unwrap();

    if input.is_empty() {
        return Err("Why blank bro".to_string());
    }

    let new_files: Vec<String> = files
        .into_iter()
        .filter(|x| x.contains(&input))
        .map(|x| {
            println!("hello from random bs{}", x);
            x.to_string()
        })
        .collect();

    println!("{:?}", new_files);

    Ok(new_files)
}

// check if file name is simmlar / same as the file
//
fn check_sim() {
    let file_name = input();
    let files = list_files().unwrap();

    for file in files {
        println!("default file {:?}", file);
        let start = "";
        let parts: Option<(&str, &str)> = file.split_once('/');
        // this one splits the at the / so
        // ./main.rs
        // would just turn into main.rs

        let file_to_search_for = start.to_string() + &file_name;
        println!("file to search {:?}", file_to_search_for);

        match parts {
            Some((_name, _extension)) => {
                if file_name == _extension {
                    println!("found your file! {}", _extension);
                }

                let compare_name_to = _extension;
                let result: String = compare_name_to
                    .split('.')
                    // this one splits it at .
                    // so now its main.rs -> main
                    .next()
                    .unwrap_or("")
                    .to_string();

                if result == file_to_search_for {
                    println!("----------------------------");
                    println!("your file is {}\n {}", result, _extension);
                    println!("----------------------------");
                    return;
                } else {
                    println!("Failed to find file {}", file_to_search_for);
                }
            }
            None => {
                println!("filename does not contain a period!.");
            }
        }
    }
}

fn check_if_empty(answers: Option<Vec<MatchedLine>>) {
    match answers {
         Some(results_vec) => {
             if results_vec.is_empty() {
                 println!("No matching lines found");
             } else {
                 for (file_name, line_number,line) in results_vec {
                     println!("Line: {} {} file: {}",line_number,line,file_name);
                 }
             }
         }
         None => {
             println!("Nothing burger");
         }
    }
}

fn main() {
    check_sim();

    println!("Hello, world!");
    lt();
    let word_find = input();
    let blahbalh = random_bs(word_find.clone()).expect("Error random bs func");
    println!("{:?}", blahbalh);
    let answers = read_files(word_find).expect("failed to read file");

    check_if_empty(Some(answers));
}
