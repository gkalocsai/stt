

struct PatternIndex<'a> {
    pattern:&'a str,
    index: usize ,
}

pub fn multimatch<'a>(source:&'a str, mut pattern_index: Vec<PatternIndex> )  {
    if(pattern_index.len() >=1 && pattern_index.get(0).unwrap() ) {
       
    if indexes.is_none() {
        indexes= create_endindexes(source, &patterns);
    }   
    if indexes.is_none() {return; } 
   

    for i in indexes.unwrap().iter().rev() {
        let slice = &source[*i..];
        println!("{}", slice);
    }
}
}




pub fn multimatch_init<'a>(source:&'a str, patterns: Vec<&str> ) -> Option<Vec<&'a str>> {
    let end_indexes_wrapped= create_endindexes(source, &patterns);
    if end_indexes_wrapped.is_none() {return None;}
    let end_indexes= end_indexes_wrapped.unwrap();
     
    println!("{:?}",end_indexes);
    let mut res:Vec<&str> = vec![];
    let mut ei:usize=0;
    let mut begin:usize=0;
    for  p in patterns {
        let eis=end_indexes.get(ei).unwrap();
        res.push(&source[begin..eis-p.len()]);
        begin =*eis;
        ei+=1;
    }
    res.push(&source[*end_indexes.last().unwrap()..]);
    return Some(res);
}  

pub fn create_endindexes (source: &str, patterns: &Vec<&str> ) -> Option<Vec<usize>> {
    let mut end_indexes: Vec<usize> = vec![];
    let mut from: usize = 0;
    for p in patterns {
    let inner= &source[from.. ];
    let found= inner.find(p);
    if found.is_none() {return None}
    from+= found.unwrap();
    from+=p.len();
    end_indexes.push(from);
    }
    Some(end_indexes)

} 