mod request;

use std::{
    collections::{HashMap, LinkedList, VecDeque}, env, error::Error, fmt::format, fs::File, io::{BufRead, BufReader}
};
use chrono::{offset, DateTime, Utc};
use hash_lib::Hashtable;
use queue_lib::QueueTrait;
use request::Request;

use crate::request::RequestParser;

const MS_FACTOR : i64 = 1000;
const TIME_WINDOW_IN_MS: i64 = 5 * MS_FACTOR ;
const MAX_REQUESTS: usize = 10;

fn get_file(path: &str) -> Option<File> {
    File::open(path).ok()
}

fn print_log_attack(request: &Request) {
  let ip = &request.ip;
  println!("ALERTA: IP {} podría estar lanzando un ataque ({} accesos en {}s )", ip, request.timestamp_queue.size(), TIME_WINDOW_IN_MS / MS_FACTOR);
}

fn get_key(ip_str: &str)->String {
  format!("{}",ip_str)
}

fn process_line(line: &str, logs_hash: &mut Hashtable::<Request, 50000>) -> Result<(),  Box<dyn Error>> {
    let line_args: Vec<&str> = line.split(' ').collect();

    let ip_str = line_args.get(0).unwrap_or_else(|| &"");
    let date_str = line_args.get(1).unwrap_or_else(|| &"");
    let time_str = line_args.get(2).unwrap_or_else(|| &"");

    let timestamp = RequestParser::parse_datetime(date_str, time_str)?;
    let key_ip = get_key(ip_str);

    match logs_hash.search_mut(key_ip.as_str()) {
       Some(asociated_request) => {
          let new_timestamp = timestamp;
          let count = &mut asociated_request.count;

          *count += 1;

          let timestamp_queue = &mut asociated_request.timestamp_queue;

          timestamp_queue.enqueue(timestamp);

         if timestamp_queue.size() > 1 && let Some(old_timestamp) = timestamp_queue.peek() {

            let offset_old_timestamp = new_timestamp.and_utc().timestamp_millis() - old_timestamp.and_utc().timestamp_millis();

            if offset_old_timestamp > TIME_WINDOW_IN_MS {
                 if timestamp_queue.size() > MAX_REQUESTS {
                  print_log_attack(asociated_request);
                }
                asociated_request.timestamp_queue.clear();
            }
          }
        },
        None => {
          let request_obj = Request::build(line_args.as_slice())?;
          logs_hash.insert(get_key(ip_str), request_obj);
        }
    }

    Ok(())
}

fn run(logs_hash: &mut Hashtable::<Request, 50000>) -> Result<(),  Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];
    let file = get_file(&file_path).expect(format!("Error opening file: {}", file_path).as_str());

    let br = BufReader::new(file);
    let mut buffered_lines = br.lines();

    while let Some(Ok(line)) = buffered_lines.next() {
      process_line(line.as_str(), logs_hash)?;
    }

    Ok(())
}

fn main() {
    let mut logs_hash = Hashtable::<Request, 50000>::new();

    if let Err(e) = run(&mut logs_hash) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
