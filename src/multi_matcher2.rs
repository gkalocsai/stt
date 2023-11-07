

pub fn multimatch<'a>(source:&'a str, patterns: Vec<&str> ) -> Option<Vec<&'a str>> {
    let mut end_indexes: Vec<usize>=vec![];
    
    let mut from:usize =0;
    for  p in &patterns {
        let inner= &source[from..];
        let found=inner.find(p);
        if found.is_none()  {return None};
        
        from += found.unwrap();
        from+=p.len();
        end_indexes.push(from);
    } 
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
    res.push(&source[end_indexes.pop().unwrap()..]);
    return Some(res);
}  