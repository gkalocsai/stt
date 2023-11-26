
pub struct GString{
    char_vector:Vec<char>,
}

impl GString {
  pub fn new(a:String) -> GString {
      let t = GString{
        char_vector: a.chars().collect(),
      };
      return t;   
   }
   pub fn new2(a:&GString) -> GString {
    let t = GString{
      char_vector: a.to_string().chars().collect(),
    };
    return t;   
 }
  pub fn char_at(&self,i:usize) ->Option<&char> {
     return self.char_vector.get(i);
  } 
  pub fn to_string(&self) ->String {
    return self.char_vector.iter().collect();
  }
  pub fn length(&self) ->usize {
    return self.char_vector.len();
  }
  pub fn index_from(&self, pattern:GString, from:usize) -> Option<usize> {
    for i in from..self.length() {
        if self.same_from(i, &pattern) { return  Some(i)};
    }
    return None;
  }
  pub fn index_of(&self, pattern:GString) -> Option<usize> { 
      self.index_from(pattern,0)
  }

  pub fn same_from(&self, from:usize, pattern:&GString) -> bool{
    for i in 0..pattern.length() {
        let p = pattern.char_at(i);
        let c=self.char_at(from+i);
        if c.is_none() {return false;}
        if p.unwrap() != c.unwrap() {return false;}     
    }
    return true;
  }
  pub fn append_char(& mut self, a:char ) {
     self.char_vector.push(a);
  }

  pub fn append_gstring(& mut self, a:GString ) {
    for c in a.char_vector {
    self.char_vector.push(c);
    }
 }


}
