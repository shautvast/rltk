__RLTK__

An attempt to manually port some of nltk to rust.

Currently in it's infancy (but growing): 
* rltk::lm::preprocessing::pad_both_ends(\["a","b","c"], 2) -> "\<s>", "a", "b", "c", "\</s>"]
* rltk::util::pad_sequence == same as above with customisation
* rltk::util::pad_sequence_left == same
* rltk::util::pad_sequence_right == same
* rltk::util::ngrams(\["a","b","c"],2) -> \[\["a", "b"], \["b", "c"]]
* rltk::util::bigrams(\["a","b","c"]) == ngrams(..., 2) 
* rltk::util::trigrams(\["a","b","c"]) == ngrams(..., 3)
* rltk::util::everygrams(\["a","b","c"],2) ==  \[\["a"], \["a", "b"], \["b"], \["b", "c"]]
* rltk::util::flatten(\[\["a"], \["a", "b"], \["b"], \["b", "c"]]) ==  \[\"a", "a", "b", "b", "b", "c"]
* rltk::metrics::distance::edit_distance(): calculate the levenshtein distance between two words (see doc)

