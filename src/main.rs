
//mod multi_matcher;

pub mod gstring;
use gstring::GString;
pub mod text;
use text::Text;





 
fn main() {



    let mut l=GString::new("Hello, world !!!xxxxxxxxxxxxxxxxxxxx".to_owned());
    let p=GString::new(" ss".to_owned());
    l.append_gstring(p);


    let text = Text::new(&l);

    for a in text.tokens {
        println!("{:?}",a.to_string());

    }


    

   


}


