use reqwest;
use std::env;

fn main() {
    let resp = reqwest::blocking::get(
        "https://www.whatismybrowser.com/guides/the-latest-user-agent/chrome",
    )
    .unwrap()
    .text()
    .unwrap();

    let mut user_agents: Vec<String> = Vec::new();
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

    // Argument
    let args: Vec<String> = env::args().collect();
    let param = &match args.get(1) {
        Some(ok) => ok,
        None => {
            println!(
        "Options for this program:\n-a or --all for all \n--win for Win64 \n--wow for WOW64 \n--webkit for AppleWebKit"
        );
            std::process::exit(0);
        }
    };

    // Print all
    match param.as_str() {
        "-a" => {
            for i in 0..user_agents.len() {
                println!("{}", user_agents[i]);
            }
        }
        "--all" => {
            for i in 0..user_agents.len() {
                println!("{}", user_agents[i]);
            }
        }

        "--win" => {
            println!("{}", user_agents[0]);
        }
        "--wow" => {
            println!("{}", user_agents[1]);
        }
        "--webkit" => {
            println!("{}", user_agents[2]);
        }
        _ => {
            println!("Options for this program:\n-a or --all for all \n--win for Win64 \n--wow for WOW64 \n--webkit for AppleWebKit")
        }
    }
}
