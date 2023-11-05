pub struct Source {
    text: Vec<char>
}

impl Source{
    pub fn new (s:String) -> Self{
        Self { 
            text:s.chars().collect(),
         }
    }
    pub fn matchFrom(&self, p:&str, from:usize ) ->Option<usize>{

        for i in from..self.text.len() - (p.len()-1){
          if self.fun_name(p,i){
            return Some(i);
          }
        }
        None

    }

    fn fun_name(&self, p: &str, from:usize) -> bool {
        let mut counter=0;
        println!("P: {} ",p);
        for pi in p.chars(){
            let c = self.text.get(from+counter).unwrap();
            println!("Comparing: {} to pattern char {} ", c, &pi);
              if c != &pi{
                return  false;
              }
              counter+=1;
          }
          println!("matches from: {}  pattern: {} ", from, p);
          true
    }
}

pub struct PatternMatch {
     index: Option<u32>,
     pattern :String
}

impl PatternMatch {
    pub fn new (s:String) -> Self{
        Self { 
            pattern: s,
            index: None
         }
    }
}

fn create_patterns(ps: Vec<String>) -> Vec<PatternMatch>{

    let mut result= vec![];
    for pattern in ps.iter(){
        result.push(PatternMatch { index: None, pattern: pattern.to_owned() })
    }
    return result;
}
pub struct SourceMatch{
    source: Source,
    patterns: Vec<PatternMatch>,
}

impl SourceMatch {
    pub fn new (source:Source, ps: Vec<String>) -> Self{
        Self { 
            source,
            patterns: create_patterns(ps)    
         }
    }
    pub fn match_fixed_parts(&self) {
   
     
       
    }
}

impl Iterator for SourceMatch{
    type Item = Vec<String>;   //non-fixed parts

    
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { 
        
        return Some(vec!["".to_owned()]);
        
       //self.source.text.get(0);
    }
 
}




