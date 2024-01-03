use clap::Parser;
use serde_json::{Result, Value};

/// Parser for JSON. We can only have file or JSON, not both  
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

    // order does not matter here, just testing whether json exists.
    if let Some(json) = args.json {
        v = serde_json::from_str(&json).unwrap();
    } else {
        v = serde_json::from_str(&std::fs::read_to_string(args.file.unwrap()).unwrap()).unwrap();
    }

    // We assume that we will always have a title, xtitle, ytitle and items
    let title = v["title"].as_str().unwrap();
    let xtitle = v["xtitle"].as_str().unwrap();
    let ytitle = v["ytitle"].as_str().unwrap();
    let items = v["items"].as_array().unwrap();

    // get the maximal value of all the items
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

    // print the graph.
    // Probably not the best way to print it since it is not very flexible.
    // Works for example config, but larger inputs may look a little weird.
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
    println!();

    println!("                {}", xtitle);
    println!("                ______");
}
