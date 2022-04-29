use std::slice::Iter;

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
// this is a windowing function on a list
// pub fn ngrams<'a>(mut sequence: impl Iterator<Item=&'a str> + 'static, n: usize) -> impl Iterator<Item=impl Iterator<Item=&'a str> + 'a> + 'a {
pub fn ngrams<'a>(sequence: &'a Vec<&'a str>, n: usize) -> impl Iterator<Item=impl Iterator<Item=&'a &'a str> + 'a> + 'a {
    let mut ngram = Vec::new();

    NGramSequenceIter { sequence: sequence, n, current_ngram: ngram, index: 0, sequence_iter: None }
}

struct NGramSequenceIter<'a> {
    sequence_iter: Option<Box<dyn Iterator<Item=&'a &'a str> + 'a>>,
    sequence: &'a Vec<&'a str>,
    n: usize,
    current_ngram: Vec<&'a &'a str>,
    index: usize,
}

impl<'a> Iterator for NGramSequenceIter<'a> {
    type Item = Box<dyn Iterator<Item=&'a &'a str> + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ngram.len() == 0 {
            self.sequence_iter = Some(Box::new(self.sequence.iter()));
            for i in 0..self.n {
                self.current_ngram.push(self.sequence_iter.as_mut().unwrap().next().unwrap());
                self.index += 1;
            }

            return Some(Box::new(self.current_ngram.clone().into_iter()));
        } else {
            self.current_ngram.remove(0);
            let maybe_next = self.sequence_iter.as_mut().unwrap().next();
            self.index += 1;
            return if maybe_next.is_some() {
                self.current_ngram.push(&maybe_next.unwrap());
                Some(Box::new(self.current_ngram.clone().into_iter()))
            } else {
                None
            };
        }
    }
}

pub(crate) struct Padder<'a> {
    n: usize,
    text: Box<dyn Iterator<Item=&'a str>>,
    pad_left: bool,
    left_index: isize,
    left_pad_symbol: &'static str,
    pad_right: bool,
    right_index: isize,
    right_pad_symbol: &'static str,
}

impl<'a> Iterator for Padder<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pad_left && self.left_index < self.n as isize {
            self.left_index += 1;
            return Some(self.left_pad_symbol);
        } else {
            let maybe_next = self.text.next();
            if maybe_next.is_some() {
                return maybe_next;
            } else {
                if self.pad_right && self.right_index < self.n as isize {
                    self.right_index += 1;
                    return Some(self.right_pad_symbol);
                }
            }
        }

        None
    }
}

impl<'a> Padder<'a> {
    pub(crate) fn new(text: Box<dyn Iterator<Item=&'a str>>, pad_left: bool, left_pad_symbol: &'static str,
                      pad_right: bool, right_pad_symbol: &'static str, n: usize, ) -> Self {
        Self { text, n, pad_left, left_index: 1, left_pad_symbol, pad_right, right_index: 1, right_pad_symbol }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_both_ends_default_n2() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence(text, true, "<s>", true, "</s>", 2);
        assert!(equal(padded, vec!["<s>", "a", "b", "c", "</s>"].into_iter()));
    }

    #[test]
    fn test_pad_left() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence_left(text, "<s>", 2);
        assert!(equal(padded, vec!["<s>", "a", "b", "c"].into_iter()));
    }

    #[test]
    fn test_pad_right() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence_right(text, "</s>", 2);
        assert!(equal(padded, vec!["a", "b", "c", "</s>"].into_iter()));
    }

    #[test]
    fn test_pad_both_ends_default_n_eq_3() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence(text, true, "<s>", true, "</s>", 3);
        assert!(equal(padded, vec!["<s>", "<s>", "a", "b", "c", "</s>", "</s>"].into_iter()));
    }

    #[test]
    fn test_pad_both_ends_non_default_symbols() {
        let text = vec!["a", "b", "c"].into_iter();
        let padded = pad_sequence(text, true, "left", true, "right", 2);
        assert!(equal(padded, vec!["left", "a", "b", "c", "right"].into_iter()));
    }

    #[test]
    fn test_bigrams() {
        let sequence = vec!["a", "b", "c", "d"];
        let mut bigrams = ngrams(&sequence, 2);
        let mut bigram = bigrams.next().unwrap();
        let item = bigram.next().unwrap();
        assert_eq!(*item, "a");
        let item = bigram.next().unwrap();
        assert_eq!(*item, "b");
        assert!(bigram.next().is_none());

        let mut bigram = bigrams.next().unwrap();
        let item = bigram.next().unwrap();
        assert_eq!(*item, "b");
        let item = bigram.next().unwrap();
        assert_eq!(*item, "c");
        assert!(bigram.next().is_none());

        let mut bigram = bigrams.next().unwrap();
        let item = bigram.next().unwrap();
        assert_eq!(*item, "c");
        let item = bigram.next().unwrap();
        assert_eq!(*item, "d");
        assert!(bigram.next().is_none());
    }


    fn equal<'a>(mut l1: impl Iterator<Item=&'a str>, mut l2: impl Iterator<Item=&'a str>) -> bool {
        loop {
            let e1 = l1.next();
            let e2 = l2.next();
            if e1.is_none() {
                return if e2.is_none() {
                    true
                } else {
                    false
                };
            } else if e2.is_none() {
                return false;
            } else {
                if e1.unwrap() != e2.unwrap() {
                    return false;
                }
            }
        }
    }
}