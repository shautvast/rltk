/// Pads a sequence of words
/// sentence: sequence to pad, in the form of an Iterator of string slices.
/// pad_left: if set to true, prepends a padding symbol to the sentence
/// left_pad_symbol: the padding symbol to prepend
/// pad_right: if set to true, appends a padding symbol after the sentence
/// right_pad_symbol: the padding symbol to append
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_sequence<'a>(sentence: impl Iterator<Item=&'a str> + 'static, pad_left: bool, left_pad_symbol: &'static str, pad_right: bool, right_pad_symbol: &'static str, n: usize) -> impl Iterator<Item=&'a str> {
    Padder::new(Box::new(sentence), pad_left, left_pad_symbol, pad_right, right_pad_symbol, n)
}

/// Pads a sequence of words, left-padding only. Convenience function that prevents useless arguments
/// sentence: sequence to pad, in the form of an Iterator of string slices.
/// left_pad_symbol: the padding symbol to prepend
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_sequence_left<'a>(text: impl Iterator<Item=&'a str> + 'static, left_pad_symbol: &'static str, n: usize) -> impl Iterator<Item=&'a str> {
    Padder::new(Box::new(text), true, left_pad_symbol, false, "", n)
}

/// Pads a sequence of words, right-padding only. Convenience function that prevents useless arguments
///
/// sentence: sequence to pad, in the form of an Iterator of string slices.
/// pad_right: if set to true, appends a padding symbol after the sentence
/// right_pad_symbol: the padding symbol to append
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_sequence_right<'a>(text: impl Iterator<Item=&'a str> + 'static, right_pad_symbol: &'static str, n: usize) -> impl Iterator<Item=&'a str> {
    Padder::new(Box::new(text), false, "", true, right_pad_symbol, n)
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