// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = magazine.to_vec();
    for word in note
    {
        if words.contains(word)
        {
            let index = words.iter().position(|_word| word == _word).unwrap();
            words.remove(index);
        }
        else
        {
            return false;
        }
    };

    true
}