
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    // making this generic over anything
    // that can find itself it a String
    delimiter: D,
}


impl<'haystack, D> StrSplit<'haystack, D> {
    // accept a haystack to split using and delimiter to split by
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

pub trait Delimiter {
  fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

//let x: StrSplit
//for part in x {
//}
//
//<'_> is an anonymous lifetime, telling the compiler to infer the lifetime
//valid where there is only one possible guess
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
// We implement the Iterator only when D implements Delimiter
where
    D: Delimiter,
{
     type Item = &'haystack str;
     // Keeps calling next while returning some and the loop can be terminated after
     fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            //find next place where delimiter appears
            //chop string off till that position and return it
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(&remainder) {
                // if the delimiter does appear collect contents
                let until_delimiter = &remainder[..delim_start];
                // modify self.remainder to be everything following the delimiter
                *remainder = &remainder[delim_end..];
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

impl Delimiter for &str {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.find(self).map(|start| (start, start + self.len()))
  }
}

impl Delimiter for char {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.char_indices()
      .find(|(_, c)| c == self)
      .map(|(start, _)| (start, start + self.len_utf8()))
  }
}

fn until_char<'s> (s: &'s str, c: char ) -> &'_ str {
  StrSplit::new(s, c)
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
