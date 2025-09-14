/*",
"Retrieved from https://www.angelfire.com/extreme4/safer_sephiroth/EVERY_WORD_EVER.htm",
"Filtered out proper nouns
*/

pub struct Dictionary {
    pub dictionary: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            dictionary: vec![
                "aaaarrrrgggghhhh".to_string(),
                "aaargh".to_string(),
                "aardvark".to_string(),
                "aardvarks".to_string(),
                "aardwolf".to_string(),
                "abacus".to_string(),
                "abandon".to_string(),
                "abandoned".to_string(),
                "abandoning".to_string(),
                "abandonment".to_string(),
                "abandons".to_string(),
                "abase".to_string(),
                "abate".to_string(),
                "abated".to_string(),
                "abatement".to_string(),
                "abating".to_string(),
                "abattoir".to_string(),
                "abattoirs".to_string(),
                "abbacy".to_string(),
                "abberation".to_string(),
                "abbey".to_string(),
            ],
        }
    }
}
