mod multi_matcher;
use std::io;

use crate::multi_matcher::Source;




fn main() {

   
 
    //let v=vec!["a".to_owned(), "bb".to_owned(), "ca".to_owned()];
    let mm = Source::new("Hello, world aa bb cc dd a bb cc".to_owned());
    mm.matchFrom("llo", 0);




}


fn read_line() -> String {
    let mut input=String::new();
            match io::stdin().read_line(&mut input) {
               Err(e) => println!("OOps! Something went wrong {} ", e),
               Ok(_) =>  {},
            }
   return input;
}


