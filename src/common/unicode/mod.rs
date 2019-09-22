pub use grapheme::{Graphemes, GraphemeIndices};
pub use grapheme::{GraphemeCursor, GraphemeIncomplete};
// pub use word::{UWordBounds, UWordBoundIndices, UnicodeWords};
// pub use sentence::{USentenceBounds, USentenceBoundIndices, UnicodeSentences};

mod grapheme;
mod tables;
// mod word;
// mod sentence;

pub mod wcwidth;

/// Methods for segmenting strings according to
/// [Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/).
pub trait UnicodeSegmentation {
    /// Returns an iterator over the [grapheme clusters][graphemes] of `self`.
    ///
    /// [graphemes]: http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
    ///
    /// If `is_extended` is true, the iterator is over the
    /// *extended grapheme clusters*;
    /// otherwise, the iterator is over the *legacy grapheme clusters*.
    /// [UAX#29](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
    /// recommends extended grapheme cluster boundaries for general processing.
    fn graphemes<'a>(&'a self, is_extended: bool) -> Graphemes<'a>;

    /// Returns an iterator over the grapheme clusters of `self` and their
    /// byte offsets. See `graphemes()` for more information.
    fn grapheme_indices<'a>(&'a self, is_extended: bool) -> GraphemeIndices<'a>;

    // /// Returns an iterator over the words of `self`, separated on
    // /// [UAX#29 word boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries).
    // ///
    // /// Here, "words" are just those substrings which, after splitting on
    // /// UAX#29 word boundaries, contain any alphanumeric characters. That is, the
    // /// substring must contain at least one character with the
    // /// [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic)
    // /// property, or with
    // /// [General_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values).
    // fn unicode_words<'a>(&'a self) -> UnicodeWords<'a>;

    // /// Returns an iterator over substrings of `self` separated on
    // /// [UAX#29 word boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries).
    // ///
    // /// The concatenation of the substrings returned by this function is just the original string.
    // fn split_word_bounds<'a>(&'a self) -> UWordBounds<'a>;

    // /// Returns an iterator over substrings of `self`, split on UAX#29 word boundaries,
    // /// and their offsets. See `split_word_bounds()` for more information.
    // fn split_word_bound_indices<'a>(&'a self) -> UWordBoundIndices<'a>;

    // /// Returns an iterator over substrings of `self` separated on
    // /// [UAX#29 sentence boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries).
    // ///
    // /// The concatenation of the substrings returned by this function is just the original string.
    // fn unicode_sentences<'a>(&'a self) -> UnicodeSentences<'a>;

    // /// Returns an iterator over substrings of `self` separated on
    // /// [UAX#29 sentence boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries).
    // ///
    // /// Here, "sentences" are just those substrings which, after splitting on
    // /// UAX#29 sentence boundaries, contain any alphanumeric characters. That is, the
    // /// substring must contain at least one character with the
    // /// [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic)
    // /// property, or with
    // /// [General_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values).
    // fn split_sentence_bounds<'a>(&'a self) -> USentenceBounds<'a>;

    // /// Returns an iterator over substrings of `self`, split on UAX#29 sentence boundaries,
    // /// and their offsets. See `split_sentence_bounds()` for more information.
    // fn split_sentence_bound_indices<'a>(&'a self) -> USentenceBoundIndices<'a>;
}

impl UnicodeSegmentation for str {
    // #[inline]
    fn graphemes(&self, is_extended: bool) -> Graphemes {
        grapheme::new_graphemes(self, is_extended)
    }

    // #[inline]
    fn grapheme_indices(&self, is_extended: bool) -> GraphemeIndices {
        grapheme::new_grapheme_indices(self, is_extended)
    }

    // // #[inline]
    // fn unicode_words(&self) -> UnicodeWords {
    //     word::new_unicode_words(self)
    // }

    // // #[inline]
    // fn split_word_bounds(&self) -> UWordBounds {
    //     word::new_word_bounds(self)
    // }

    // // #[inline]
    // fn split_word_bound_indices(&self) -> UWordBoundIndices {
    //     word::new_word_bound_indices(self)
    // }

    // // #[inline]
    // fn unicode_sentences(&self) -> UnicodeSentences {
    //     sentence::new_unicode_sentences(self)
    // }

    // // #[inline]
    // fn split_sentence_bounds(&self) -> USentenceBounds {
    //     sentence::new_sentence_bounds(self)
    // }

    // // #[inline]
    // fn split_sentence_bound_indices(&self) -> USentenceBoundIndices {
    //     sentence::new_sentence_bound_indices(self)
    // }
}
