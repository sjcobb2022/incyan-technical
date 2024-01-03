
# InCyan Technical Task 


## Installation Instructions

To install the project. You must first clone the repository.

```bash
git clone https://github.com/sjcobb2022/incyan-technical
```

You will need [cargo](https://www.rust-lang.org/tools/install) installed on you machine to run this.

To run the project simply run the bash command

```bash
cargo run -- [OPTIONS]
```


```bash
Options:
  -j, --json <JSON>  JSON string that will be parsed
  -f, --file <FILE>  Input file
  -h, --help         Print help
  -V, --version      Print version
```


### Usage


As a JSON string:
```bash
cargo run -- --json='{
      "title": "stock count",
      "xtitle": "asset",
      "ytitle": "count",
      "items": [
          {"chairs": 20},
          {"tables": 5},
          {"stands": 7},
          {"lamps": 8},
          {"cups": 10}
      ]
  }'
```

With a file:
```bash
cargo run -- --file='./path/to/file.json'
```


