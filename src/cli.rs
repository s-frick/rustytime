use core::fmt;
use std::error::Error;

use chrono::{NaiveDateTime, NaiveTime, ParseError, Utc};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "rt")] //  rr - rusty times
#[command(bin_name = "rt")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    Start {
        #[arg(value_parser = parse_date, short, long)]
        at: Option<NaiveDateTime>,
        #[arg(value_parser = parse_tags)]
        tags: Vec<String>,
    },
    Stop {
        #[arg(value_parser = parse_date, short, long)]
        at: Option<NaiveDateTime>,
    },
    Status,
    Log {
        #[arg(short, long, num_args = 0..=1, value_name="FORMAT", default_value_t = Format::Pretty, default_missing_value = "pretty", value_enum)]
        format: Format,
    },
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
    #[value(alias("p"))]
    Pretty,
    #[value(alias("j"))]
    Json,
    #[value(alias("y"))]
    Yaml,
    #[value(alias("c"))]
    Csv,
}

#[derive(Debug)]
struct TagParseError {
    details: String,
}

impl TagParseError {
    fn new(details: &str) -> Self {
        Self {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for TagParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for TagParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn parse_tags(arg: &str) -> Result<String, TagParseError> {
    arg.strip_prefix('+')
        .map(|t| t.to_string())
        .ok_or(TagParseError::new("Tag must start with '+'"))
}

fn parse_date(arg: &str) -> Result<NaiveDateTime, ParseError> {
    let today = Utc::now().naive_local().date();
    // TODO: add more formats
    NaiveDateTime::parse_from_str(arg, "%Y-%m-%d %H:%M:%S")
        .or(NaiveDateTime::parse_from_str(arg, "%m-%d %H:%M:%S"))
        .or(NaiveDateTime::parse_from_str(arg, "%m-%d %H:%M:%S"))
        .or(NaiveDateTime::parse_from_str(arg, "%m-%d %H:%M"))
        .or(NaiveTime::parse_from_str(arg, "%H:%M:%S").map(|t| today.and_time(t)))
        .or(NaiveTime::parse_from_str(arg, "%H:%M").map(|t| today.and_time(t)))
}
