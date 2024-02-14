use core::fmt;
use std::{
    fs::{self, File, OpenOptions},
    hash::{Hash, Hasher},
    io::{self, Read, Write},
    os::unix::fs::FileExt,
    path::Path,
};

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::cli::{self, Commands};
extern crate sha2; // 0.9.1

use sha2::{Digest, Sha256}; // 0.9.1

const FRAMES_PATH: &str = "/home/sfrick/.rtime/frames";
const STATE_PATH: &str = "/home/sfrick/.rtime/state";
const TAGS_PATH: &str = "/home/sfrick/.rtime/tags";

#[derive(Debug, Serialize, Deserialize)]
struct ID(String);

type Tag = String;

#[derive(Debug, Serialize, Deserialize)]
struct Frame {
    id: ID,
    start: NaiveDateTime,
    end: NaiveDateTime,
    tags: Vec<Tag>,
}

impl Frame {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime, tags: Vec<Tag>) -> Frame {
        let hash = Sha256::new()
            .chain(start.to_string())
            .chain(end.to_string())
            .chain(tags.join(","))
            .finalize();
        Frame {
            id: ID(format!("{:x}", hash)),
            start,
            end,
            tags,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    tags: Vec<Tag>,
    start: NaiveDateTime,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tags_str = self.tags.join(",");
        write!(f, "start: {}, tags: {}", self.start, tags_str)
    }
}

#[derive(Debug, Default)]
pub struct RTime {
    tags: Vec<Tag>,
    state: Option<State>,
}

impl RTime {
    pub fn new(tags: Vec<Tag>, state: Option<State>) -> Self {
        Self { tags, state }
    }
}

pub fn start(cmd: Commands) {
    if let Commands::Start { at, tags } = cmd {
        stop(at);
        // TODO: stop and write frame from state

        let start = at.unwrap_or_else(|| Utc::now().naive_local());
        let new_state = State { start, tags };
        let new_state_str = serde_json::to_string(&new_state).unwrap();
        let mut file = File::create(String::from(STATE_PATH)).unwrap();
        file.write_all(new_state_str.as_bytes()).unwrap();
        println!(
            "Starting frame [{}] at {}\n",
            new_state.tags.join(" "),
            start
        );
    }
}

pub fn stop(at: Option<NaiveDateTime>) {
    let rtime = read_rtime();
    if let Some(state) = rtime.state {
        let stop = at.unwrap_or_else(|| Utc::now().naive_local());
        fs::remove_file(String::from(STATE_PATH)).expect("ERRO: Failed removing state_file {}");

        println!("Stopping frame [{}] at {}.", state.tags.join(" "), stop);
        create_frame(state, stop);
    }
}

pub fn status() {
    let rtime = read_rtime();
    match rtime.state {
        Some(start) => println!("Started frame [{}] at {}\n", rtime.tags.join(" "), start),
        None => println!("No frame started."),
    }
}

fn write_all_frames(xs: Vec<Frame>) {
    let _ = fs::remove_file(String::from(FRAMES_PATH));
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(FRAMES_PATH)
        .unwrap();

    if let Err(e) = serde_json::to_writer(&file, &xs) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn write_frame(x: Frame) {
    let mut frames = read_frames();
    frames.push(x);
    write_all_frames(frames);
}

fn create_frame(state: State, at: NaiveDateTime) {
    let frame = Frame::new(state.start, at, state.tags);
    write_frame(frame);
}

fn read_file(path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_state() -> Option<State> {
    let contents = read_file(String::from(STATE_PATH));
    if let Ok(c) = contents {
        let state: State = serde_json::from_str(&c).expect("ERRO: Failed reading state.");
        Some(state)
    } else {
        None
    }
}

fn read_tags() -> Vec<Tag> {
    let contents = read_file(String::from(TAGS_PATH));
    if let Ok(c) = contents {
        let state: Vec<String> = serde_json::from_str(&c).expect("ERRO: Failed reading state.");
        state
    } else {
        vec![]
    }
}

fn read_frames() -> Vec<Frame> {
    let contents = read_file(String::from(FRAMES_PATH));
    if let Ok(c) = contents {
        let frames: Vec<Frame> = serde_json::from_str(&c).expect("ERRO: Failed reading state.");
        frames
    } else {
        vec![]
    }
}

fn read_rtime() -> RTime {
    let state = read_state();
    let tags = read_tags();
    RTime::new(tags, state)
}
