use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead, Write},
    sync::{Arc, Mutex, mpsc},
};

use clap::Parser;
use log::info;
use threadpool::ThreadPool;
use uuid::Uuid;

use crate::{
    args::Args,
    bucket::{bucket_diff::PlayerBucketDiff, bucket_list::BucketList},
    heap::HeapItem,
    player::Player,
    util::players_from_ratings,
};

mod args;
mod bucket;
mod heap;
mod player;
mod util;

fn main() -> Result<(), io::Error> {
    env_logger::init();

    let args = Args::parse();
    let dataset_path = args.dataset_path;
    let score_output_path = args.score_output_path;
    let num_threads = args.threads;
    let score_output_file = Arc::new(Mutex::new(File::create(score_output_path).unwrap()));

    let diff = PlayerBucketDiff::new(15_000);

    let file = File::open(dataset_path)?;
    let reader = io::BufReader::new(file);

    let mut searcher_lobby = [Player::new(Uuid::new_v4(), 0); 5];

    let heap_max_size = 5;

    let heap = Arc::new(Mutex::new(BinaryHeap::new()));
    let bucket_list = Arc::new(Mutex::new(BucketList::new(diff, 15_000)));
    let pool = ThreadPool::new(num_threads as usize);

    let (tx, rx) = mpsc::channel();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() != 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Line {} does not have exactly 5 numbers: {}",
                    line_num + 1,
                    line
                ),
            ));
        }

        let mut nums = [0u16; 5];
        for (i, part) in parts.iter().enumerate() {
            nums[i] = part.parse::<u16>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid number at line {}: '{}'", line_num + 1, part),
                )
            })?;
        }

        if line_num == 0 {
            searcher_lobby = players_from_ratings(&nums);

            println!("Current team: {:#?}", searcher_lobby);
        } else {
            let nums = nums.clone();
            let heap = Arc::clone(&heap);
            let bucket_list = Arc::clone(&bucket_list);
            let tx = tx.clone(); // signal completion
            let searcher_lobby = searcher_lobby.clone(); // assuming this is Clone
            let output_file = Arc::clone(&score_output_file);

            pool.execute(move || {
                let mut output_file = output_file.lock().expect("Could not acquire output file");
                let waiting_lobby = players_from_ratings(&nums);

                let mut bucket_list = bucket_list.lock().expect("Could not acquire bucket list");
                let sanction_score = bucket_list.bucket_diff(searcher_lobby, waiting_lobby);

                let _ = output_file.write_all(format!("{}\n", sanction_score).as_bytes());

                print!("\rRow {}", line_num);
                io::stdout().flush().unwrap();

                // Insert into heap if better than existing

                let heap_item = HeapItem::new(sanction_score, waiting_lobby);
                let mut heap = heap.lock().expect("Could not acquire heap");

                if heap.len() < heap_max_size {
                    heap.push(heap_item);
                } else if let Some(top) = heap.peek() {
                    if heap_item.score() < top.score() {
                        heap.pop();
                        heap.push(heap_item);
                    }
                }

                tx.send(()).unwrap();
            });
        }
    }

    drop(tx);
    rx.iter().count();

    {
        let heap = heap.lock().unwrap();
        let lobbies = heap.clone().into_sorted_vec();

        println!(); // for row prints before this
        info!("Matches: {:#?}", lobbies);
    }

    Ok(())
}
