use crate::{
    cli::{Commands, Format},
    settings::Settings,
};
use chrono::{NaiveDateTime, Utc};
use core::panic;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
};
extern crate sha2; // 0.9.1
use sha2::{Digest, Sha256}; // 0.9.1

const FRAMES_PATH: &str = "/data/frames";
const STATE_PATH: &str = "/data/state";
const TAGS_PATH: &str = "/data/tags";

#[derive(Debug, Serialize, Deserialize)]
struct ID(String);
impl ToString for ID {
    fn to_string(&self) -> String {
        String::from(self.0.as_str())
    }
}

type Tag = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
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

#[derive(Debug, Default)]
pub struct RTime {
    tags: Vec<Tag>,
    state: Option<State>,
    frames: Vec<Frame>,
    settings: Settings,
}

impl RTime {
    pub fn new(settings: Settings) -> Self {
        let state = read_state(&settings);
        let tags = read_tags(&settings);
        let frames = read_frames(&settings);
        Self {
            tags,
            state,
            frames,
            settings,
        }
    }
    pub fn start(&self, cmd: Commands) {
        if let Commands::Start { at, tags } = cmd {
            if tags.len() == 0 {
                eprintln!("ERRO: At least one tag is required.");
                return;
            }
            let home = self.settings.rustytime.home.clone();
            self.stop(at);

            let start_1 = at.unwrap_or_else(|| Utc::now().naive_local());
            let new_state = State {
                start: start_1,
                tags,
            };

            let new_state_str = serde_json::to_string(&new_state);
            match new_state_str {
                Err(serde_json::Error { .. }) => {
                    eprintln!("ERRO: Failed serializing state");
                }

                Ok(state_str) => {
                    let path = format!("{}{}", home, STATE_PATH);
                    let prefix = match std::path::Path::new(path.as_str()).parent() {
                        Some(prefix) => prefix,
                        None => panic!("ERRO: Failed getting parent path for: {}", path),
                    };
                    if let Err(e) = std::fs::create_dir_all(prefix) {
                        panic!("ERRO: Failed creating parent path: {}", e);
                    };

                    let _ = match fs::OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .open(format!("{}{}", home, STATE_PATH))
                        .map(|mut file| file.write_all(state_str.as_bytes()))
                    {
                        Ok(f) => f,
                        Err(e) => {
                            panic!("ERRO: Failed opening state file: {}", e);
                        }
                    };

                    println!(
                        "Starting frame [{}] at {}\n",
                        new_state.tags.join(" "),
                        new_state.start
                    );
                }
            }
        }
    }

    pub fn stop(&self, at: Option<NaiveDateTime>) {
        let home = self.settings.rustytime.home.clone();
        if let Some(state) = &self.state {
            let stop = at.unwrap_or_else(|| Utc::now().naive_local());
            fs::remove_file(format!("{}{}", home, STATE_PATH))
                .expect("ERRO: Failed removing state_file {}");

            println!("Stopping frame [{}] at {}.", state.tags.join(" "), stop);
            create_frame(state, stop, &self.settings);
        }
    }

    pub fn status(&self) {
        match &self.state {
            Some(state) => println!(
                "Started frame [{}] at {}\n",
                state.tags.join(" "),
                state.start
            ),
            None => println!("No frame started."),
        }
    }

    pub fn log(&self, format: Format, from: Option<NaiveDateTime>, to: Option<NaiveDateTime>) {
        let mut filtered_frames = self
            .frames
            .iter()
            .filter(|f| from.map_or(true, |t| f.start >= t) && to.map_or(true, |t| f.end <= t))
            .collect::<Vec<_>>();

        filtered_frames.sort_by(|a, b| a.start.cmp(&b.start));
        match filtered_frames.as_slice() {
            [] => println!("No frames tracked."),
            xs @ [..] => match format {
                Format::Pretty => todo!(),
                Format::Json => self.log_json(xs),
                Format::Yaml => self.log_yaml(xs),
                Format::Csv => self.log_csv(xs),
            },
        }
    }

    fn log_json(&self, xs: &[&Frame]) {
        let str = serde_json::to_string(&xs).unwrap();
        println!("{}", str)
    }

    fn log_yaml(&self, xs: &[&Frame]) {
        let str = serde_yaml::to_string(&xs).unwrap();
        println!("{}", str)
    }

    fn log_csv(&self, xs: &[&Frame]) {
        let mut wtr = csv::Writer::from_writer(io::stdout());
        wtr.write_record(["id", "start", "end", "tags"]).unwrap();
        xs.iter().for_each(|f| {
            wtr.write_record(&[
                f.id.to_string(),
                f.start.to_string(),
                f.end.to_string(),
                f.tags.join(" "),
            ])
            .unwrap();
        });
        let _ = wtr.flush();
    }
}

fn write_all_frames(xs: Vec<Frame>, settings: &Settings) {
    let path = format!("{}{}", settings.rustytime.home, FRAMES_PATH);
    let _ = fs::remove_file(&path);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    if let Err(e) = serde_json::to_writer(&file, &xs) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn write_frame(x: Frame, settings: &Settings) {
    let mut frames = read_frames(settings);
    frames.push(x);
    write_all_frames(frames, settings);
}

fn create_frame(state: &State, at: NaiveDateTime, settings: &Settings) {
    let frame = Frame::new(state.start, at, state.tags.clone());
    write_frame(frame, settings);
}
fn read_file(path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_state(settings: &Settings) -> Option<State> {
    let home = settings.rustytime.home.clone();
    let contents = read_file(format!("{}{}", home, STATE_PATH));
    if let Ok(c) = contents {
        let state: State = serde_json::from_str(&c).expect("ERRO: Failed reading state.");
        Some(state)
    } else {
        None
    }
}

fn read_tags(settings: &Settings) -> Vec<Tag> {
    let home = settings.rustytime.home.clone();
    let contents = read_file(format!("{}{}", home, TAGS_PATH));
    if let Ok(c) = contents {
        let state: Vec<String> = serde_json::from_str(&c).expect("ERRO: Failed reading state.");
        state
    } else {
        vec![]
    }
}

fn read_frames(settings: &Settings) -> Vec<Frame> {
    let home = settings.rustytime.home.clone();
    let contents = read_file(format!("{}{}", home, FRAMES_PATH));
    if let Ok(c) = contents {
        let frames: Vec<Frame> = serde_json::from_str(&c).expect("ERRO: Failed reading state.");
        frames
    } else {
        vec![]
    }
}
