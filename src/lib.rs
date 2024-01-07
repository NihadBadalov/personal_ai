pub mod ai;

#[cfg(test)]
mod tests {
    use crate::ai::classifiers::{nsfw, scientific, technology};

    /* NSFW Classification */
    #[test]
    fn nsfw_many_swear_words() {
        assert_eq!(nsfw(&"what the fuck is this shit".to_string()), false);
    }

    #[test]
    fn nsfw_kys_threats() {
        assert_eq!(nsfw(&"kill yourself bud".to_string()), false);
    }

    #[test]
    fn nsfw_insult() {
        assert_eq!(nsfw(&"fuck you".to_string()), false);
    }

    #[test]
    fn nsfw_f_this_im_out() {
        assert_eq!(nsfw(&"fuck this shit im out".to_string()), false);
    }

    #[test]
    fn nsfw_nsfw_nature() {
        assert_eq!(nsfw(&"how to have sex".to_string()), true);
    }

    /* Scientific Classification */
    #[test]
    fn scientific_mathematical() {
        // Common knowledge question
        // Any AI could answer this
        assert_eq!(scientific(&"What is 2 + 2?".to_string()), false);
    }

    #[test]
    fn scientific_logical_apples_question() {
        // Common knowledge question?
        // We simply don't need to waste our mathematical resources on this
        assert_eq!(
            scientific(
                &"I have 32 apples now. I ate 4 yesterday. How many apples do I have now?"
                    .to_string()
            ),
            false
        );
    }

    #[test]
    fn scientific_explain_logarithmics() {
        assert_eq!(scientific(&"Explain logarithms.".to_string()), true);
    }

    #[test]
    fn scientific_phylosopy_question() {
        assert_eq!(
            scientific(&"What is the meaning of life".to_string()),
            false
        );
    }

    #[test]
    fn scientific_english_question() {
        assert_eq!(
            scientific(&"What is the difference between the subject and the object?".to_string()),
            false
        );
    }

    /* Technology Classification */
    #[test]
    fn technology_english_question2() {
        assert_eq!(
            technology(&"What is the difference between the subject and the object?".to_string()),
            false
        );
    }

    #[test]
    fn technology_math_question() {
        assert_eq!(technology(&"What is 2 + 3?".to_string()), false);
    }

    #[test]
    fn technology_hardware_question() {
        assert_eq!(technology(&"What is a computer?".to_string()), false);
    }

    #[test]
    fn technology_gpu_question() {
        assert_eq!(
            technology(&"Which GPU is better: 4070Ti or 4080?".to_string()),
            true
        );
    }

    #[test]
    fn technology_csgo_vs_cod_question() {
        assert_eq!(
            technology(&"Which shooter is now more popular: CSGO or Call of Duty?".to_string()),
            true
        );
    }

    #[test]
    fn technology_birds_question() {
        assert_eq!(
            technology(&"Why do birds live in warm places?".to_string()),
            false
        );
    }
}
