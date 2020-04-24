
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}


impl<'a> StrSplit<'a> {
    // accept a haystack to split using and delimiter to split by
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
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
impl<'a> Iterator for StrSplit<'a> {
     type Item = &'a str;
     // Keeps calling next while returning some and the loop can be terminated after
     fn next(&mut self) -> Option<Self::Item> {
         //find next place where delimiter appears
         //chop string off till that position and return it
         if let Some(next_delim) = self.remainder.find(self.delimiter) {
             // if the delimiter does appear collect contents
             let until_delimiter = &self.remainder[..next_delim];
             // modify self.remainder to be everything following the delimiter
             self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
             // return until delimiter
             return Some(until_delimiter)
         } else if self.remainder.is_empty() {
             None
         } else {
             let rest = self.remainder;
             self.remainder = "";
             Some(rest)
         }
     }
}


#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}
