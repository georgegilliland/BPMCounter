#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use std::io::{self, BufRead, BufReader, stdin, Read};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::rc::Rc;
use rodio::{Source};

enum State {
    Started,
    InProgress,
    Stopped
}

fn main() {
    let mut state = State::Started;
    let mut entry = String::new();
    let mut bpm = String::new();

    loop {
        match state {
            State::Started => {
                println!("Enter BPM: ");
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue
                }

                if entry.is_empty() == true {
                    continue;
                }

                if entry.is_empty() == false {
                    bpm = entry.clone();
                    println!("BPM is: {}", bpm);
                    state = State::InProgress;
                }
            }
            State::InProgress => {
                let str = "Starting... Press enter to stop".to_string();
                println!("{}", str);
                let bpm_m: u64 = bpm.parse().unwrap();
                println!("{}", bpm_m);
                let bpm_mm = 60000 / bpm_m;
                println!("{}", bpm_mm);
                let (tx, rx) = mpsc::channel();
                let mut x = 0;
                let file = File::open("src/censor-beep-01.wav").unwrap();
                thread::spawn(move || loop {
                    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

                    match rx.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(TryRecvError::Empty) => {
                            stream_handle.play_raw(source.convert_samples());
                            if x == 0 || x % 4 == 0 {
                                //sink.set_volume(0.5);
                                //sink.append(source);
                            } else {
                                //sink.set_volume(0.3);
                                //sink.append(source);
                            }
                            thread::sleep(Duration::from_millis(bpm_mm));
                        }
                    }
                    x += 1;
                });

                let mut line = String::new();
                let stdin = io::stdin();
                let _ = stdin.lock().read_line(&mut line);
                let _ = tx.send(());
                state = State::Stopped;
            }

            State::Stopped => {
                println!("Stopped");
                return;
            }
        }
    }
}