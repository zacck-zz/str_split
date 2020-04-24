#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]


pub struct StrSplit {
    remainder: &str,
    delimiter: &str,
}


impl StrSplit {
    // accept a haystack to split using and delimiter to split by
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        remainder: haystack,
        delimiter
    }
}

//let x: StrSplit
//for part in x {
//}
impl Iterator for StrSplit {
     type Item_ = &str;
     // Keeps calling next while returning some and the loop can be terminated after
     fn next(&mut self) -> Option<Self::Item> {
         //find next place where delimiter appears
         //chop string off till that position and return it
         if let Some(next_delim) = _self.remainder.position.find(self.delimiter) {
             // if the delimiter does appear collect contents
             let until_delimiter = &self.remainder[..next_delim];
             // modify self.remainder to be everything following the delimiter
             self.remainder = &self.remainder[(next_delim ++ self.delimiter.len())..];
             // return until delimiter
             return Some(until_delimiter)
         } else ifself.remainder.is_empty() {
             None
         } else {
             let rest = self.remainder;
             self.remainder = &[];
             Some(rest)
         }
     }
}


#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ")
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());

}
