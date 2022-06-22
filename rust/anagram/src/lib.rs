use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>
{
    let mut anagrams: HashSet<&str> = HashSet::new();

    for _word in possible_anagrams
    {
        if !_word.to_lowercase().eq(word.to_lowercase().as_str())
        {
            let mut sorted_word = _word.as_bytes()
                .to_ascii_lowercase()
                .to_owned();
            sorted_word.sort_unstable();

            let mut sorted_original_word = word.as_bytes()
                .to_ascii_lowercase()
                .to_owned();
            sorted_original_word.sort_unstable();

            if sorted_original_word.eq_ignore_ascii_case(sorted_word.as_slice())
            {
                anagrams.insert(_word);
            }
        }
    }

    anagrams
}
