

fn main() {

    let str1 = "abc";
    let str2 = "a\n\nb\n\nc";
    let str3 = "a\n\nb\n\nc\nd\ne\n\nf";


    let doc1 = make_document(str1); // 1 paragraph
    let doc2 = make_document(str2); // 2 paragraphs
    let doc3 = make_document(str3); // 3 paragraphs
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
    println!("{:#?}", rnk_docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);

}

//Problem 1.1
fn make_document(text : &str)-> Vec<String> {

    let mut paragraphs = Vec::new();

    let mut paragh = String::new();

    for line in text.lines() {

        if line == "" && paragh.len() > 0 {
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

//Problem 1.2
fn rank_documents( docs : &Vec<Vec<String>>) -> Vec<Vec<String>> {

    let mut docs = docs.clone();
    docs.sort_by(|a, b|b.len().cmp(&a.len()));

    docs.to_vec()
}



#[test]
fn test_paragraph() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";


    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}