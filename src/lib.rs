pub mod mymod {

    pub fn make_document(text : &str)-> Vec<String> {

        let mut paragraphs = Vec::new();
    
        let mut paragh = String::new();
    
        for line in text.lines() {
    
            if line == "" {
                paragraphs.push(paragh.clone());
                paragh = "".to_string();
                continue;
            }else {
                paragh.push_str(line);
            }
        }
    
        paragraphs.push(paragh.clone());
    
        paragraphs
    }
    
    
    pub fn rank_documents( docs : &Vec<Vec<String>>) -> Vec<Vec<String>> {
    
        let mut docs = docs.clone();
        docs.sort_by(|a, b|b.len().cmp(&a.len()));
    
        docs.to_vec()
    }

    pub fn rank_words( docs : &Vec<(String, Vec<String>)>) -> Vec<(String,Vec<String>)> {
    
        let mut docs = docs.clone();
        println!("{:?}", docs);


        docs
    }
}