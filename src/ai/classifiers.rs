use crate::ai::ask;

fn answer_match_to_bool(s: &String) -> bool {
    if s == "1" {
        true
    } else if s.contains("yes") {
        true
    } else if s == "0" {
        false
    } else if s.contains("not sure") {
        false
    } else if s.contains("unsure") {
        false
    } else if s.contains("not sure") {
        false
    } else if s.contains("can't tell") {
        false
    } else if s.contains("no") {
        false
    } else {
        false
    }
}

pub fn nsfw(input: &String) -> bool {
    let output_text = ask(
        &"neural-chat",
        &"Does this paragraph have sexual context? Write ONLY Yes or No; if not sure, write ONLY No. Here is the paragraph: {}. Please only write YES or NO. Nothing else. Please. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    // println!("{}", output_text);
    answer_match_to_bool(&output_text)
}

pub fn scientific(input: &String) -> bool {
    let output_text = ask(
        &"neural-chat",
        &"You are a Scientific Question Classifier. Does this paragraph have a scientific, mathematical, statistical, technical, or a logical context that requires to be answered by a SCIENTIST? Write ONLY Yes or No; if not sure, write ONLY No. If the question can easily be answered by Aritifical Intellegence, you MUST ONLY write No; for example, for 'What is 2+2?', you MUST write No. Here is the paragraph: {}. Please only write YES or NO. Nothing else. Please. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    let whether_not_scientific = ask(
        &"neural-chat",
        &"Is this paragraph very easy to answer by Aritifical Intellegence, for example, 'What is 2+2?'? Write ONLY Yes or No; if not sure, write ONLY No. Here is the paragraph: {}. Please write only YES or NO. Nothing else. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    let common_knowledge = ask(
        &"neural-chat",
        &"Does this paragraph contain a Common Knowledge question or a question about a language? Write ONLY Yes or No; if not sure, write ONLY No. Here is the paragraph: {}. Please write only YES or NO. Nothing else. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    // println!("{}", output_text);
    answer_match_to_bool(&output_text)
        && !answer_match_to_bool(&whether_not_scientific)
        && !answer_match_to_bool(&common_knowledge)
}

pub fn technology(input: &String) -> bool {
    let technology_question = ask(
        &"neural-chat",
        &"Does this paragraph contain a difficult question about a technology that could have changed or been updated over time, which may require Internet search to return correct results? Write ONLY Yes or No; if not sure, write ONLY No. Here is the paragraph: {}. Please write only YES or NO. Nothing else. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    let hardware_and_games = ask(
        &"neural-chat",
        &"Does this paragraph contain a question about a game, hardware, or computer parts? Write ONLY Yes or No; if not sure, write ONLY Yes. Here is the paragraph: {}. Please write only YES or NO. Nothing else. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    let common_knowledge = ask(
        &"neural-chat",
        &"Does this paragraph contain an Easy (for example, 2+2) OR a Common Knowledge question or a question about Nature? Write ONLY Yes or No; if not sure, write ONLY No. Here is the paragraph: {}. Please write only YES or NO. Nothing else. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    let difficult_question = ask(
        &"neural-chat",
        &"Does this paragraph contain a DIFFICULT question that MUST be answered by Aritifical Intellegence? Write ONLY Yes or No; if not sure, write ONLY No. Here is the paragraph: {}. Please write ONLY Yes or No. Nothing else. I do not have hands, so I cannot remove the useless words you type.",
        &input
    ).unwrap();

    (answer_match_to_bool(&technology_question) || answer_match_to_bool(&hardware_and_games))
        // || !answer_match_to_bool(&common_knowledge)
        && ((!answer_match_to_bool(&common_knowledge) || !answer_match_to_bool(&difficult_question)) || !(answer_match_to_bool(&hardware_and_games) || answer_match_to_bool(&technology_question)))
}

pub fn internet_search_queries(input: &String) -> Vec<String> {
    // Things to search about on the Internet
    let searches = ask(
        &"neural-chat",
        // &"You are given the following paragraph: {}. Give me a few queries or keywords to search about on the Internet. Separate the queries with semicolons. Separate the queries ONLY with semicolons. Please, only write the queries!",
        &"You are given the following paragraph: {}. Give me a few queries or keywords to search about on the Internet. Separate the queries with commas. Separate the queries ONLY with commas. Please, only write the queries!",
        &input,
    ).unwrap();

    searches
        .split(';')
        .map(|e| {
            e.trim()
                .to_owned()
                .split(',')
                .map(|j| j.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
}
