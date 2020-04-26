
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    // Keep this a str as it keeps the code more performant
    // Avoiding needing an Allocator as opposed to when using a String
    delimiter: &'delimiter str,
}


impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    // accept a haystack to split using and delimiter to split by
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

//let x: StrSplit
//for part in x {
//}
//
//<'_> is an anonymous lifetime, telling the compiler to infer the lifetime
//valid where there is only one possible guess
impl<'haystack> Iterator for StrSplit<'haystack, '_> {
     type Item = &'haystack str;
     // Keeps calling next while returning some and the loop can be terminated after
     fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            //find next place where delimiter appears
            //chop string off till that position and return it
            if let Some(next_delim) = remainder.find(self.delimiter) {
                // if the delimiter does appear collect contents
                let until_delimiter = &remainder[..next_delim];
                // modify self.remainder to be everything following the delimiter
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                // return until delimiter
                return Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
     }
}

fn until_char<'s> (s: &'s str, c: char ) -> &'_ str {
  let delim = format!("{}", c);
  StrSplit::new(s, &delim)
    .next()
    .expect("StrSplit always returns at least one result")
}

#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
  let haystack = "a b c d ";
  let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
  assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn until_char_test() {
  assert_eq!(until_char("hello world", 'o'), "hell");
}
