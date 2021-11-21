use reqwest;
use std::env;

fn main() {
    let help = "Options:
        -a or --all for all 
        --win for Win64 
        --wow for WOW64 
        --webkit for AppleWebKit

        There must be one argument,and one only";
    // Argument handeling
    let args: Vec<String> = env::args().collect();
    let param = &match args.get(1) {
        Some(ok) => ok,
        None => {
            println!("{}", help);
            std::process::exit(0);
        }
    };
    let conf: Conf = match param.as_str() {
        "-a" | "--all" => Conf::All,
        "--win" => Conf::Win,
        "--wow" => Conf::Wow,
        "--webkit" => Conf::Webkit,
        _ => {
            println!("{}", &help);
            std::process::exit(0);
        }
    };

    // Get the page
    let resp = reqwest::blocking::get(
        "https://www.whatismybrowser.com/guides/the-latest-user-agent/chrome",
    )
    .unwrap()
    .text()
    .unwrap();

    let mut user_agents: Vec<String> = Vec::new();

    // Get useragent from html
    for i in resp.lines() {
        if i.contains("Windows NT 10.0") {
            let i = &i
                .replace("<li><span class=\"code\">", "")
                .replace("</span></li>", "")
                .trim()
                .to_string();
            user_agents.push(i.to_owned());
        }
    }

    // Do as the user wishes
    match conf {
        Conf::All => {
            for i in 0..user_agents.len() {
                println!("{}", user_agents[i]);
            }
        }
        Conf::Win => {
            println!("{}", user_agents[0]);
        }
        Conf::Wow => {
            println!("{}", user_agents[1]);
        }
        Conf::Webkit => {
            println!("{}", user_agents[2]);
        }
    }
}
enum Conf {
    All,
    Win,
    Wow,
    Webkit,
}
