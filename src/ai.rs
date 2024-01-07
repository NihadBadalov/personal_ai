pub mod classifiers;

pub fn ask(model_name: &str, question: &str, user_input: &String) -> Option<String> {
    // Run a terminal command and wait for 1 character (1/0) output
    let formatted_question = format!(
        "\"### System:\n{{ {} }}\n\n### User:\n{{ {} }}\n\n### Assistant:\"",
        question,
        urlencoding::encode(&user_input).to_string().as_str()
    );

    let mut cmd = std::process::Command::new("ollama");
    cmd.arg("run");
    cmd.arg(model_name);
    // cmd.arg(format!("\"You are a robot sexual-message classifier. Is this message sexually-explicit and/or does it have sexual content OR sexual language/words (like boobs, pussy, etc) - ONLY write digit 1 for yes or ONLY digit 0 for no, if the message doesn't have any sexual content. IF YOU ARE UNSURE, WRITE 0: '{}'. Do NOT write anything else besides a single digit! ONLY A SINGLE DIGIT - please, I do not have hands to remove words - NO WORDS, I beg you. Also think of an argument why that would be a sexually-explicit message, but don't send it to me\"", urlencoding::encode(&s).to_owned()));
    // cmd.arg(format!(formatted_question, urlencoding::encode(&user_input).to_owned()));
    cmd.arg(
        // formatted_question.replace("{}", urlencoding::encode(&user_input).to_string().as_str()),
        &formatted_question,
    );

    let output = cmd.output();
    let stdout = output.unwrap().stdout;
    let output_text = std::str::from_utf8(&stdout).unwrap();
    let final_output = output_text.replace("\n", "").to_lowercase();

    /* println!("\n\n------------------");
    println!("question: {:#?}", formatted_question);
    println!("output: {:#?}", output_text);
    println!("final_output: {:#?}", final_output);
    println!("------------------\n\n"); */

    Option::from(final_output)
}
