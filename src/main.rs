// 1. The graph must be drawn by writing to a console or the equivalent command in the language that you choose - eg println(), console.log(), echo.
// 1. The output **must** be a **vertical** bar chart
// 1. The output does not need to look *exactly* the same as the below, but this is to provide a guide.
// 1. The output **may** be in colour, but this is not necessary.
// 1. Outline any assumptions made about the json payload using code comments.
//
// The goal of this exercise is to assess coding ability and thought processes in a language agnostic way.
//
// Example payload:
// ```json
// {
//     "title": "stock count",
//     "xtitle": "asset",
//     "ytitle": "count",
//     "items": [
//         {"chairs": 20},
//         {"tables": 5},
//         {"stands": 7},
//         {"lamps": 8},
//         {"cups": 10}
//     ]
// }
// ```

use clap::Parser;
use serde_json::{Result, Value};

/// Parser for JSON  
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long, default_value = "input.json")]
    file: std::path::PathBuf,

    #[arg(short, long, required_unless_present("file"), conflicts_with("file"))]
    json: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut v: Value;

    if let Some(json) = args.json {
        v = serde_json::from_str(&json).unwrap();
    } else {
        v = serde_json::from_str(&std::fs::read_to_string(args.file).unwrap()).unwrap();
    }

    let title = v["title"].as_str().unwrap();
    let xtitle = v["xtitle"].as_str().unwrap();
    let ytitle = v["ytitle"].as_str().unwrap();
    let items = v["items"].as_array().unwrap();

    let mut max = 0;
    for item in items {
        let value = item.as_object().unwrap().values().next().unwrap().as_i64().unwrap();
        if value > max {
            max = value;
        }
    }

    println!("{}: {}", title, ytitle);
    
    for item in items {
        let key = item.as_object().unwrap().keys().next().unwrap();
        let value = item.as_object().unwrap().values().next().unwrap().as_i64().unwrap();
        let mut bar = String::new();
        for _ in 0..value {
            bar.push_str("█");
        }
        println!("{:10} | {:<10} | {}", key, value, bar);
    }
    println!("{}: {}", xtitle, max);

}
