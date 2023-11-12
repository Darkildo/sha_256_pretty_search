use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use futures::{executor::block_on, lock::Mutex};
use rand::random;

use sha256::digest;
use substring::Substring;
fn main() {
    println!("Hi! type any String in console!");

    let mut input_text = String::new();

    let result = std::io::stdin().read_line(&mut input_text);

    if !result.err().is_none() {
        println!("Read error");
        return;
    }

    let input_hash = digest(&input_text);

    println!("Your hash: {0}", input_hash);

    println!("Select hash difficult: ");

    let mut input_difficult = String::new();

    let _err = std::io::stdin().read_line(&mut input_difficult);

    let difficult_result = input_difficult.trim().parse::<i16>();

    if difficult_result.is_err() {
        println!("Read error, need a number like: 12345");
        return;
    }

    let difficult = difficult_result.ok().unwrap();
    let mut thread_counter = num_cpus::get();
    let shared_difficult = Arc::new(Mutex::new(difficult));
    let shared_source_hash = Arc::new(Mutex::new(&input_hash));
    let string_channel = mpsc::channel::<String>();
    let counter_channel = mpsc::channel::<i32>();

    let mut main_counter = 0;
    loop {
        if thread_counter == 0 {
            break;
        }
        thread_counter -= 1;
        let thread_chared_hash_sender = string_channel.0.clone();

        let thread_chared_counter_sender = counter_channel.0.clone();
        let thread_shared_difficult = block_on(shared_difficult.lock()).clone();
        let thread_shared_source_hash = block_on(shared_source_hash.lock()).clone();

        thread::spawn(move || {
            println!("Start generation in thread: {}", thread_counter);

            thread::sleep(Duration::from_millis(1));
            let _result = find_hash(
                thread_shared_difficult,
                thread_shared_source_hash.clone(),
                thread_chared_counter_sender,
                thread_chared_hash_sender,
            );
        });
    }
    let generated_salt_string;
    loop {
        let counter_res = counter_channel.1.try_recv();
        if counter_res.is_ok() {
            main_counter += counter_res.unwrap();
        }

        let string_res = string_channel.1.try_recv();

        if string_res.is_ok() {
            generated_salt_string = string_res.unwrap();
            println!("Your text:   {}", input_text);
            println!("Your hash: {}", input_hash);
            println!("Finded Hash: {}", digest(&generated_salt_string));
            println!(
                "matcherPart: {}",
                input_hash.substring(0, difficult as usize)
            );
            println!("Salt: {}", generated_salt_string);
            println!(
                "Generated hash count: {}",
                if main_counter > 0 {
                    ">".to_string() + &main_counter.to_string()
                } else {
                    "<1000".to_string()
                }
            );

            break;
        }

        thread::sleep(Duration::from_millis(1));
    }
}

fn find_hash(
    difficult: i16,
    find_hash: String,
    counter_sender: Sender<i32>,
    string_sender: Sender<String>,
) {
    let mut counter = 0;

    loop {
        counter += 1;

        let generated_salt: String = random::<i128>().to_string();

        let generated_hash = digest(&generated_salt);

        let compart_string = generated_hash.substring(0, difficult as usize);
        if compart_string == find_hash.substring(0, difficult as usize) {
            let _ = string_sender.send(generated_salt);

            break;
        }

        if counter >= 1000 {
            let error = counter_sender.send(counter);
            if error.is_err() {
                break;
            }

            counter = 0;
        }
    }

    return;
}
