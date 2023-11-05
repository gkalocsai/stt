mod multi_matcher;
use std::io;

use multi_matcher::SourceMatch;

use crate::multi_matcher::Source;




fn main() {

   
 
    //let v=vec!["a".to_owned(), "bb".to_owned(), "ca".to_owned()];
    let mm = Source::new("Hello, world aa bb cc dd a bb cc".to_owned());
    let patterns=vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    let sm:SourceMatch = SourceMatch::new(mm, patterns);
    sm.match_fixed_parts();

    



}


