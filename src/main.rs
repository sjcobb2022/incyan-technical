use clap::Parser;
use serde_json::{Result, Value};

/// Parser for JSON  
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON string
    #[arg(short, long, required_unless_present("file"), conflicts_with("file"))]
    json: Option<String>,

    /// Input file
    #[arg(short, long, required_unless_present("json"), conflicts_with("json"))]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut v: Value;

    if let Some(json) = args.json {
        v = serde_json::from_str(&json).unwrap();
    } else {
        v = serde_json::from_str(&std::fs::read_to_string(args.file.unwrap()).unwrap()).unwrap();
    }

    let title = v["title"].as_str().unwrap();
    let xtitle = v["xtitle"].as_str().unwrap();
    let ytitle = v["ytitle"].as_str().unwrap();
    let items = v["items"].as_array().unwrap();

    let mut max = 0;

    for item in items {
        let value = item
            .as_object()
            .unwrap()
            .values()
            .next()
            .unwrap()
            .as_i64()
            .unwrap();
        if value > max {
            max = value;
        }
    }

    println!("                    {}", title);
    println!("                    ___________");

    println!("{} ", ytitle);
    println!("_____");

    for i in (0..max + 1).rev() {
        for item in items {
            let value = item
                .as_object()
                .unwrap()
                .values()
                .next()
                .unwrap()
                .as_i64()
                .unwrap();
            if value >= i {
                print!("     *    ");
            } else {
                print!("          ");
            }
        }
        println!();
    }

    print!("  ");
    for items in items {
        print!("  {}  ", items.as_object().unwrap().keys().next().unwrap());
    }

    println!();

    println!("                {}", xtitle);
    println!("                ______");
}
