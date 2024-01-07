use ai::{
    ask,
    classifiers::{internet_search_queries, nsfw, scientific, technology},
};
use reqwest::Error;
use search::internet_search;
use std::future::IntoFuture;

pub mod ai;
pub mod search;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // User input one line
    let line = std::io::stdin();
    let mut input = String::new();
    line.read_line(&mut input).unwrap();
    input = input.trim().to_string();

    println!("Running classifications...\n\n");

    // Classifications
    let is_nsfw = nsfw(&input);
    println!("Is NSFW: {}", is_nsfw);

    let is_scientific = scientific(&input);
    println!("Is scientific: {}", is_scientific);

    let is_technological = technology(&input);
    println!("Is technological: {}\n", is_technological);

    let mut web_context: String = "".to_owned();

    if is_technological {
        println!("Finding possible internet queries");

        let possible_searches: Vec<String> = internet_search_queries(&input);

        println!("Possible searches: {:?}", possible_searches);
        println!("Running search...\n\n");

        for query in &possible_searches {
            web_context.push_str(
                &internet_search(&query)
                    .into_future()
                    .await
                    .unwrap_or(String::from("no context")),
            );
        }
    }

    let mut system_message = "Answer the following question THOROUGHLY as detailed as possible: {},".to_owned();
    system_message
        .push_str(format!(" having the following context: {}", web_context.as_str()).as_str());

    let output_text = ask(&"neural-chat", &system_message.as_str(), &input)
        .unwrap()
        .replace("{ ", "")
        .replace(" }", "");

    println!(
        "\n\nAI response:{}",
        output_text
    );

    Ok(())
}
