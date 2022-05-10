
/// These examples are taken from
/// https://www.nltk.org/api/nltk.lm.html
fn main() {
    let text = vec![vec!["a", "b", "c"], vec!["a", "c", "d", "c", "e", "f"]];

    println!("bigrams of {:?}:", text[0]);
    let bigrams = rltk::util::bigrams(text[0].iter());
    print(bigrams);

    println!("\npadding {:?}", text[0]);
    let padded: Vec<&&str> = rltk::util::pad_sequence(text[0].iter(), true, &"<s>", true, &"</s>", 2).collect();
    println!("{:?}", padded);

    println!("\ncombining bigrams and padding");
    let combined = rltk::util::bigrams(rltk::lm::preprocessing::pad_both_ends(text[0].iter(),2));
    print(combined);

    println!("\neverygrams:");
    let padded_bigrams: Vec<&&str> = rltk::lm::preprocessing::pad_both_ends(text[0].iter(),2).collect();
    println!("padded {:?}",padded_bigrams);
    let everygrams = rltk::util::everygrams(padded_bigrams.into_iter(), 2);
    print(everygrams);

    print!("or the same with padded_everygrams: ");
    let padded_everygrams = rltk::lm::preprocessing::padded_everygrams(text[0].iter(),2);
    print(padded_everygrams);


    println!("\ncombining padding and flattening: {:?}:", text);
    let flattened: Vec<&&str> = text.iter().map(|sent| rltk::lm::preprocessing::pad_both_ends(sent.iter(), 2)).flatten().collect();
    println!("{:?}", flattened);


}

fn print<'a>(nested: impl Iterator<Item=impl Iterator<Item=&'a &'a str>>) {
    print!("[");

    for group in nested {
        print!("[");
        for word in group {
            print!("{},", word);
        }
        print!("],");
    }
    println!("]");
}
