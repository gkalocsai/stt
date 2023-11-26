
use crate::GString;

pub struct Text {
    pub tokens:Vec<GString>,
}
impl Text {
    pub fn new (string: &GString)  -> Text{
        let tms: [Box<dyn TokenMatcher>; 2] = [
            Box::new(WordMatcher {}),
            Box::new(SingleCharMatcher{}),
        ];
        let mut result:Text = Text{tokens: vec![]};
        
        let mut from = 0;
        loop{  
            let mut gsw; 
            for tm in &tms {
                gsw=tm.match_token(string, from);
                if  gsw.is_none() { continue;}
                let token =  &gsw.unwrap();
                from+=token.length();
                result.tokens.push(GString::new2(token));
            } 
            if  from >=string.length() {
                return  result;
            }
        }   
    }
}
pub trait TokenMatcher {
    fn match_token(&self, s: &GString, from: usize) -> Option<GString>;
}
struct  WordMatcher{}
impl TokenMatcher for WordMatcher {
    fn match_token(&self, s: &GString,from: usize) -> Option<GString> {
        let wrapped_char = s.char_at(from);
        if wrapped_char.is_none() {
            return None; 
        }
        let mut c=wrapped_char.unwrap();
        if !c.is_alphabetic() {
            return None;
        }

        let mut result=GString::new("".to_owned());
        let mut offset:usize = 1;
        while c.is_alphabetic() {
            result.append_char(*c);
            let wrapped_char = s.char_at(from+offset);
            if wrapped_char.is_none() { return Some(result);} 
            c=wrapped_char.unwrap();
            offset +=1;

        }
        
        Some(result)
    }
}
struct SingleCharMatcher {}
impl TokenMatcher for SingleCharMatcher {
    fn match_token(&self, s: &GString, from: usize) -> Option<GString> {
        let wrapped_char = s.char_at(from);
        if wrapped_char.is_none() {
            return None; 
        }
        let c=wrapped_char.unwrap();
        let mut result=GString::new("".to_owned());
        result.append_char(*c);
        Some(result)
    }
}