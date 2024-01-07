pub async fn internet_search(input: &String) -> Result<String, std::io::Error> {
    let mut cmd = std::process::Command::new("node");
    cmd.arg("src/search.js");
    // cmd.arg(format!("\"{}\"", input));
    cmd.arg(format!("{}", input));

    let output = cmd.output();

    if let Err(e) = output {
        println!("- \tFailed to run search: {}", e);
        println!(" \tFailed to execute process: {}", e);
        return Err(e);
    }

    let stdout = &output.unwrap().stdout;
    let output_text = std::str::from_utf8(stdout).unwrap();

    let final_output = output_text
        .replace("\n", "")
        .to_lowercase();

    Ok(final_output)
}
