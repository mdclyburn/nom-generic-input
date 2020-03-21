use std::convert::AsRef;

use nom;
use nom::error::ParseError;
use nom::{IResult, InputTakeAtPosition};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Input<'a, O> {
    pub text: &'a str,
    pub opts: &'a O,
}

impl<'a, O> Input<'a, O> {
    pub fn new(input_text: &'a str, options: &'a O) -> Input<'a, O> {
        Input {
            text: input_text,
            opts: options,
        }
    }
}

impl<'a, O> AsRef<str> for Input<'a, O> {
    fn as_ref(&self) -> &str {
        self.text
    }
}

impl<'a, O> InputTakeAtPosition for Input<'a, O> {
    type Item = char;

    fn split_at_position<P, E>(&self, p: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.text.split_at_position(p) {
            Ok((i, o)) => Ok((Input { text: i, ..*self }, Input { text: o, ..*self })),
            Err(nom_err) => Err(match nom_err {
                // ParseError::from_error_kind used here to explicitly tell rustc that the tuple matches E
                nom::Err::Error((i, err)) => {
                    nom::Err::Error(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Failure((i, err)) => {
                    nom::Err::Failure(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Incomplete(info) => nom::Err::Incomplete(info),
            }),
        }
    }

    fn split_at_position1<P, E>(&self, p: P, e: nom::error::ErrorKind) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.text.split_at_position1(p, e) {
            Ok((i, o)) => Ok((Input { text: i, ..*self }, Input { text: o, ..*self })),
            Err(nom_err) => Err(match nom_err {
                // ParseError::from_error_kind used here to explicitly tell rustc that the tuple matches E
                nom::Err::Error((i, err)) => {
                    nom::Err::Error(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Failure((i, err)) => {
                    nom::Err::Failure(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Incomplete(info) => nom::Err::Incomplete(info),
            }),
        }
    }

    fn split_at_position_complete<P, E>(&self, p: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.text.split_at_position_complete(p) {
            Ok((i, o)) => Ok((Input { text: i, ..*self }, Input { text: o, ..*self })),
            Err(nom_err) => Err(match nom_err {
                // ParseError::from_error_kind used here to explicitly tell rustc that the tuple matches E
                nom::Err::Error((i, err)) => {
                    nom::Err::Error(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Failure((i, err)) => {
                    nom::Err::Failure(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Incomplete(info) => nom::Err::Incomplete(info),
            }),
        }
    }

    fn split_at_position1_complete<P, E>(
        &self,
        p: P,
        e: nom::error::ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.text.split_at_position1_complete(p, e) {
            Ok((i, o)) => Ok((Input { text: i, ..*self }, Input { text: o, ..*self })),
            Err(nom_err) => Err(match nom_err {
                // ParseError::from_error_kind used here to explicitly tell rustc that the tuple matches E
                nom::Err::Error((i, err)) => {
                    nom::Err::Error(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Failure((i, err)) => {
                    nom::Err::Failure(ParseError::from_error_kind(Input { text: i, ..*self }, err))
                }
                nom::Err::Incomplete(info) => nom::Err::Incomplete(info),
            }),
        }
    }
}

impl<'a, O> nom::Slice<std::ops::Range<usize>> for Input<'a, O> {
    fn slice(&self, r: std::ops::Range<usize>) -> Self {
        Input {
            text: self.text.slice(r),
            ..*self
        }
    }
}

impl<'a, O> nom::Slice<std::ops::RangeFrom<usize>> for Input<'a, O> {
    fn slice(&self, r: std::ops::RangeFrom<usize>) -> Self {
        Input {
            text: self.text.slice(r),
            ..*self
        }
    }
}

impl<'a, O> nom::Slice<std::ops::RangeTo<usize>> for Input<'a, O> {
    fn slice(&self, r: std::ops::RangeTo<usize>) -> Self {
        Input {
            text: self.text.slice(r),
            ..*self
        }
    }
}

impl<'a, O> nom::InputIter for Input<'a, O> {
    type Item = char;
    type Iter = std::str::CharIndices<'a>;
    type IterElem = std::str::Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.text.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.text.iter_elements()
    }

    fn position<P>(&self, p: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.text.position(p)
    }

    fn slice_index(&self, count: usize) -> Option<usize> {
        self.text.slice_index(count)
    }
}

impl<'a, O> nom::InputLength for Input<'a, O> {
    fn input_len(&self) -> usize {
        self.text.input_len()
    }
}

impl<'a, O> nom::InputTake for Input<'a, O> {
    fn take(&self, count: usize) -> Self {
        Input {
            text: &self.text[..count],
            ..*self
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (
            Input {
                text: &self.text[count..],
                ..*self
            },
            Input {
                text: &self.text[..count],
                ..*self
            },
        )
    }
}

impl<'a, O> nom::Compare<&'a str> for Input<'a, O> {
    fn compare(&self, s: &'a str) -> nom::CompareResult {
        self.text.compare(s)
    }

    fn compare_no_case(&self, s: &'a str) -> nom::CompareResult {
        self.text.compare_no_case(s)
    }
}

impl<'a, O> nom::UnspecializedInput for &'a Input<'a, O> {}
