```

 ___  ___ ___ _ __ ___
/ __|/ __/ __| '_ ` _ \
\__ \ (__\__ \ | | | | |
|___/\___|___/_| |_| |_|


```

The tool is written in Rust, and it aims to save the current info on the
machine into a JSON.

## Installation

For now, there are no pre-compiled binaries, so you have to compile by yourself
using [ cargo ](https://github.com/rust-lang/cargo).

1. Clone the repo `git clone https://github.com/joao-vitor-sr/scsm.git`
2. Enter the directory and run the cargo: `cargo build --release`
3. Clone the bin at `target/release/scsm` to your `$PATH`

## Usage

To run just type `scsm` at your terminal, the output is the JSON with the info
of the machine

## Format

The following JSON is an example of the JSON

```
{
  "components": [
    "acpitz temp1",
    "coretemp Core 0",
    "coretemp Core 1",
    "coretemp Core 2",
    "coretemp Core 3",
    "coretemp Package id 0"
  ],
  "cpus": [
    {
      "cpu_usage": 18.518518447875977,
      "frequency": 2422,
      "name": "cpu0"
    },
    {
      "cpu_usage": 14.814814567565918,
      "frequency": 2560,
      "name": "cpu1"
    },
    {
      "cpu_usage": 11.538461685180664,
      "frequency": 2321,
      "name": "cpu2"
    },
    {
      "cpu_usage": 14.285715103149414,
      "frequency": 2608,
      "name": "cpu3"
    },
    {
      "cpu_usage": 18.518518447875977,
      "frequency": 2429,
      "name": "cpu4"
    },
    {
      "cpu_usage": 14.814814567565918,
      "frequency": 2980,
      "name": "cpu5"
    },
    {
      "cpu_usage": 11.111111640930176,
      "frequency": 2273,
      "name": "cpu6"
    },
    {
      "cpu_usage": 33.333335876464844,
      "frequency": 2493,
      "name": "cpu7"
    }
  ],
  "disks": [
    "/dev/sda2",
    "/dev/sda1"
  ],
  "host_name": "myHostName",
  "kernel_version": "6.2.10-arch1-1",
  "len_cpu": 8,
  "memory": 8208797696,
  "network": {
    "docker0": [
      0,
      0
    ],
    "wlan0": [
      60,
      0
    ],
  },
  "os_version": null,
  "processes": [
    {
      "cpu_usage": 0,
      "memory": 7876608,
      "name": "zsh",
      "pid": 84006
    },
    {
      "cpu_usage": 0,
      "memory": 6201344,
      "name": "rust-analyzer-p",
      "pid": 84316
    },
    {
      "cpu_usage": 0,
      "memory": 5111808,
      "name": "tmux: client",
      "pid": 1760
    },
    {
      "cpu_usage": 0,
      "memory": 48386048,
      "name": "nvim",
      "pid": 84238
    },
  ],
  "swap": 4104122368,
  "system_name": "MyMachine Linux",
  "used_memory": 2757672960,
  "used_swap": 0
}

```

