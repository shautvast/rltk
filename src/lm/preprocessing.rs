/// Pads a sequence of words with defaults; prepends "<s>" and appends "<s>"
///
/// sentence: sequence of words, tokens, to pad, in the form of an Iterator of string slices.
/// n: the n in n-grams; so for bigrams set to 2, etc
pub fn pad_both_ends<'a>(text: impl Iterator<Item=&'a &'a str> + 'a, n: usize) -> impl Iterator<Item=&'a &'a str> {
    crate::util::padding::Padder::new(Box::new(text), true, &"<s>", true,&"</s>", n)
}

pub fn padded_everygrams<'a>(sentence: impl Iterator<Item=&'a &'a str> + 'a, n: usize) -> impl Iterator<Item=Box<dyn Iterator<Item=&'a &'a str> + 'a>> + 'a  {
    crate::util::everygrams(pad_both_ends(sentence, n), n)
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test(){
        let sentence = vec!["a","b", "c"];
        let mut bigrams = padded_everygrams(sentence.iter(),2);

        let bigram1 = vec!["<s>"];
        let bigram2 = vec!["<s>", "a"];
        let bigram3 = vec!["a"];
        let bigram4 = vec!["a", "b"];
        let bigram5 = vec!["b"];
        let bigram6 = vec!["b", "c"];
        let bigram7 = vec!["c"];
        let bigram8 = vec!["c", "</s>"];
        let bigram9 = vec!["</s>"];
        let expected = vec![bigram1.iter(), bigram2.iter(), bigram3.iter(), bigram4.iter(), bigram5.iter(), bigram6.iter(),bigram7.iter(),bigram8.iter(),bigram9.iter()];

        crate::test::should_be_equal_list_of_lists(&mut bigrams, expected)
    }
}