pub struct Padder<'a> {
    n: usize,
    text: Box<dyn Iterator<Item=&'a &'a str> + 'a>,
    pad_left: bool,
    left_index: isize,
    left_pad_symbol: &'a &'a str,
    pad_right: bool,
    right_index: isize,
    right_pad_symbol: &'a &'a str,
}

impl<'a> Iterator for Padder<'a> {
    type Item = &'a &'a str;

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
    pub(crate) fn new(text: Box<dyn Iterator<Item=&'a &'a str> + 'a>, pad_left: bool, left_pad_symbol:&'a &'a str,
                      pad_right: bool, right_pad_symbol: &'a &'a str, n: usize, ) -> Self {
        Self { text, n, pad_left, left_index: 1, left_pad_symbol, pad_right, right_index: 1, right_pad_symbol }
    }
}
