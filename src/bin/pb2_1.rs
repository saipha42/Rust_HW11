use std::{fs::File, io::{Read, Write}};

fn main() {

    let file_list = ["file1.txt", "file2.txt", "file3.txt"];
    let mut docs = Vec::new();

    for path in file_list {

        let mut file = File::open(path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        
        let doc = make_document(&text);
        docs.push((path, doc));
    }

    docs.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    

    let mut html = String::new();
    html.push_str("<body>");
    html.push_str("<style>");
    html.push_str("table, th,td { border: 1px solid black; border-collapse : collapse; padding : 15px; }");
    html.push_str("</style>");
    html.push_str("<table>");
    html.push_str("<th>File Name</th> <th>Number of Paragraph</th>");
    for (name, doc) in docs {
        let doc_len = doc.len();
        html.push_str(format!("<tr><td>{name}</td>  <td>{doc_len}</td> </tr>").as_str());
    }

    html.push_str("</table>");
    html.push_str("</body>");

    let mut html_file = File::create("html_2_1.html").unwrap();
    html_file.write_all(html.as_bytes()).unwrap();

}


fn make_document(text : &str)-> Vec<String> {

    let mut paragraphs = Vec::new();

    let mut paragh = String::new();

    for line in text.lines() {

        if line == "" && paragh.len() > 0{
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
