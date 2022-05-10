__RLTK__

An attempt to manually port some of nltk to rust.

from https://www.nltk.org/api/nltk.lm.html:

_So as to avoid re-creating the text in memory, both train and vocab are lazy iterators. They are evaluated on demand at training time._

rltk has the same philosophy: everything is done using iterators (on iterators) on string slices.

Currently in it's infancy (but growing): 
* rltk::lm::preprocessing::pad_both_ends
* rltk::util::pad_sequence
* rltk::util::pad_sequence_left
* rltk::util::pad_sequence_right
* rltk::util::ngrams
* rltk::util::bigrams 
* rltk::util::trigrams
* rltk::util::everygrams
* rltk::util::flatten
* rltk::metrics::distance::edit_distance

