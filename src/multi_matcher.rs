pub struct Source {
    text: Vec<char>
}

impl Source{
    pub fn new (s:String) -> Self{
        Self { 
            text:s.chars().collect(),
         }
    }
    pub fn matches_from(&self, p:&str, from:usize ) ->Option<usize>{

        for i in from..self.text.len() - (p.len()-1){
          if self.matches_at_index(p,i){
            return Some(i);
          }
        }
        None
    }

    fn matches_at_index(&self, p: &str, from:usize) -> bool {
        let mut counter=0;
        for pi in p.chars(){
            let c = self.text.get(from+counter);
            if c.is_none()  {return false};
            let c2=c.unwrap();
              if c2 != &pi{
                return  false;
              }
              counter+=1;
          }
          println!("matches from: {}  pattern: {} ", from, p);
          true
    }
}

pub struct PatternMatch {
     index: Option<usize>,
     pattern :String
}
pub struct SourceMatch{
    source: Source,
    patterns: Vec<PatternMatch>,
}

impl SourceMatch {
    pub fn new (source:Source, ps: Vec<String>) -> Self{
        Self { 
            source,
            patterns: Self::create_patterns(ps)    
         }
    }

    fn create_patterns(ps: Vec<String>) -> Vec<PatternMatch>{
        let mut result= vec![];
        for pattern in ps.iter(){
            result.push(PatternMatch { index: None, pattern: pattern.to_owned() })
        }
        return result;
    }
    pub fn match_fixed_parts(mut self) {
         for a in 0..self.patterns.len() {
            
            
            let result=self.source.matches_from(&self.patterns[a].pattern, 0);
            self.patterns[a].index = result; 
         }
    }
}

impl Iterator for SourceMatch{
    type Item = Vec<String>;   //non-fixed parts

    
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { 
        
        return Some(vec!["".to_owned()]);
        
       //self.source.text.get(0);
    }
 
}
