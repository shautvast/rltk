/// Pads a sequence of words with defaults; prepends "<s>" and appends "<s>"
///
/// sentence: sequence of words, tokens, to pad, in the form of an Iterator of string slices.
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_both_ends<'a>(text: impl Iterator<Item=&'a str> + 'static, n: usize) -> impl Iterator<Item=&'a str> {
    crate::util::Padder::new(Box::new(text), true, "<s>", true,"</s>", n)
}




