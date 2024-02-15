## Rusty Time - A simple time tracking tool

A cli(maybe later more) to easily track your project times right in your terminal. This project is in a very early stage, things may change quickly.

This project is inspired by [Watson](https://github.com/TailorDev/Watson), which currently has an uncertain future and is not being maintained. However, since I want to use some features of Watson that are a bit buggy at the moment I started to develop my own time tracking CLI in Rust. Maybe someone will find it useful.

## What's working

A constantly changing list of things that work or should work in future.

- [x] Start a frame
  - [x] with tags
    - [ ] you'll be asked for confirmation if it's the first time you using a tag
  - [x] with start time, `--at "15:04"`
  - [x] with stopping of current running frame, start time also works here
- [x] Stop a frame
  - [x] with stop time, `--at "15:04"`
- [ ] configuration
  - [x] rustytime home, where the data is stored
  - [ ] tag confirmation
  - [ ] stop on start
  - [ ] allow start/stop times in future
  - [ ] ...
- [ ] Frame Log
  - [ ] basic Frame Log functionality `rustytime log`
  - [ ] pretty log `rustytime log --format pretty`
  - [x] json log `rustytime log --format json`
  - [x] csv log `rustytime log --format csv`
  - [x] yaml log `rustytime log --format yaml`
  - [ ] filter by tags, time ranges
- [ ] Aggregations
- [ ] Reports

## Install

```shell
cargo install rustytime
```

## Usage

### Start a frame

Start a frame with tags "rustytime" and "cli" now.

```shell
rt start +rustytime +cli
```

Start a frame with tags "rustytime" and "cli" at "15:04".

```shell
rt start +rustytime +cli --at "15:04"
```

### Stop a frame

Stop the current frame now.

```shell
rt stop
```

Stop the current frame at "15:04".

```shell
rt stop --at "15:04"
```

### Status

Get the current status.

```shell
rt status
```

### Log

#### Json log of all frames.

```shell
rt log --format json
```

or short

```shell
rt log --format j
```

or even shorter

```shell
rt log -f j
```

#### Csv log of all frames.

```shell
rt log --format csv
```

or short

```shell
rt log --format c
```

or even shorter

```shell
rt log -f c
```

#### Yaml log of all frames.

```shell
rt log --format yaml
```

or short

```shell
rt log --format y
```

or even shorter

```shell
rt log -f y
```
