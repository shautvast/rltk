pub(crate) mod padding;
mod ngrams;

use padding::Padder;

/// Returns a padded sequence of items before ngram extraction.
///
/// sequence: sequence of items to pad, in the form of an Iterator of string slices.
/// pad_left: if set to true, prepends a padding symbol to the sentence
/// left_pad_symbol: the padding symbol to prepend
/// pad_right: if set to true, appends a padding symbol after the sentence
/// right_pad_symbol: the padding symbol to append
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_sequence<'a>(sentence: impl Iterator<Item=&'a str> + 'static, pad_left: bool, left_pad_symbol: &'static str, pad_right: bool, right_pad_symbol: &'static str, n: usize) -> impl Iterator<Item=&'a str> {
    Padder::new(Box::new(sentence), pad_left, left_pad_symbol, pad_right, right_pad_symbol, n)
}

/// Returns a padded sequence of items before ngram extraction, left-padding only. Convenience function that prevents useless arguments
/// sequence: sequence of items to pad, in the form of an Iterator of string slices.
/// left_pad_symbol: the padding symbol to prepend
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_sequence_left<'a>(sequence: impl Iterator<Item=&'a str> + 'static, left_pad_symbol: &'static str, n: usize) -> impl Iterator<Item=&'a str> {
    Padder::new(Box::new(sequence), true, left_pad_symbol, false, "", n)
}

/// Returns a padded sequence of items before ngram extraction, right-padding only. Convenience function that prevents useless arguments
///
/// sequence: sequence of items to pad, in the form of an Iterator of string slices.
/// pad_right: if set to true, appends a padding symbol after the sentence
/// right_pad_symbol: the padding symbol to append
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_sequence_right<'a>(sequence: impl Iterator<Item=&'a str> + 'static, right_pad_symbol: &'static str, n: usize) -> impl Iterator<Item=&'a str> + 'a {
    Padder::new(Box::new(sequence), false, "", true, right_pad_symbol, n)
}

/// Return the ngrams generated from a sequence of items, as an iterator.
///
/// sequence: the sequence items in the form of an Iterator over &&str
/// use like:
/// ```
/// let sequence = vec!["a", "b", "c"];
/// let mut bigrams = rltk::util::ngrams(sequence.iter(), 2);
///
/// let bigram1 = vec!["a", "b"];
/// let bigram2 = vec!["b", "c"];
/// let expected = vec![bigram1.iter(), bigram2.iter()];
///
/// for (mut left_outer,mut right_outer) in bigrams.zip(expected.into_iter()){
///     for (left_inner,right_inner) in left_outer.zip(right_outer){
///         assert_eq!(left_inner, right_inner);
///     }
/// }
/// ```
///
pub fn ngrams<'a>(sequence: impl Iterator<Item=&'a &'a str> + 'a, n: usize) -> impl Iterator<Item=impl Iterator<Item=&'a &'a str> + 'a> + 'a {
    ngrams::NGramSequenceIter::new(sequence, n)
}


#[cfg(test)]
mod tests {
    use std::slice::Iter;
    use super::*;

    #[test]
    fn test_pad_both_ends_default_n2() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence(text, true, "<s>", true, "</s>", 2);
        should_be_equal_lists(padded, vec!["<s>", "a", "b", "c", "</s>"]);
    }

    #[test]
    fn test_pad_left() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence_left(text, "<s>", 2);
        should_be_equal_lists(padded, vec!["<s>", "a", "b", "c"]);
    }

    #[test]
    fn test_pad_right() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence_right(text, "</s>", 2);

        should_be_equal_lists(padded, vec!["a", "b", "c", "</s>"]);
    }

    #[test]
    fn test_pad_both_ends_default_n_eq_3() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence(text, true, "<s>", true, "</s>", 3);
        should_be_equal_lists(padded, vec!["<s>", "<s>", "a", "b", "c", "</s>", "</s>"]);
    }

    #[test]
    fn test_pad_both_ends_non_default_symbols() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence(text, true, "left", true, "right", 2);

        should_be_equal_lists(padded, vec!["left", "a", "b", "c", "right"]);
    }

    #[test]
    fn test_bigrams() {
        let sequence = vec!["a", "b", "c", "d"];
        let mut bigrams = ngrams(sequence.iter(), 2);
        let bigram1 = vec!["a", "b"];
        let bigram2 = vec!["b", "c"];
        let bigram3 = vec!["c", "d"];
        let expected = vec![bigram1.iter(), bigram2.iter(), bigram3.iter()];

        should_be_equal_list_of_lists(&mut bigrams, expected)
    }

    #[test]
    fn test_trigrams() {
        let sequence = vec!["a", "b", "c", "d", "e"];
        let mut bigrams = ngrams(sequence.iter(), 3);
        let trigram1 = vec!["a", "b", "c"];
        let trigram2 = vec!["b", "c", "d"];
        let trigram3 = vec!["c", "d", "e"];
        let expected = vec![trigram1.iter(), trigram2.iter(), trigram3.iter()];

        should_be_equal_list_of_lists(&mut bigrams, expected)
    }

    #[test]
    fn test_bigrams_n_gt_len() {
        let sequence = vec!["a"];
        let mut bigrams = ngrams(sequence.iter(), 2);
        assert!(bigrams.next().is_none());
    }

    #[test]
    fn test_bigrams_empty_sequence() {
        let sequence = vec![];
        let mut bigrams = ngrams(sequence.iter(), 10);
        assert!(bigrams.next().is_none());
    }

    #[test]
    fn test_bigrams_n_eq_len() {
        let sequence = vec!["a", "b"];
        let mut bigrams = ngrams(sequence.iter(), 2);
        let bigram1 = vec!["a", "b"];
        let expected = vec![bigram1.iter()];

        should_be_equal_list_of_lists(&mut bigrams, expected)
    }

    fn should_be_equal_list_of_lists<'a>(bigrams: &mut impl Iterator<Item=impl Iterator<Item=&'a &'a str>>, expected: Vec<Iter<&'a str>>) {
        for (mut left_outer, mut right_outer) in bigrams.zip(expected.into_iter()) {
            for (left_inner, right_inner) in left_outer.zip(right_outer) {
                assert_eq!(left_inner, right_inner);
            }
        }
    }

    fn should_be_equal_lists<'a>(left: impl Iterator<Item=&'a str>, right: Vec<&'a str>) {
        for (left, right) in left.zip(right.into_iter()) {
            assert_eq!(left, right);
        }
    }
}