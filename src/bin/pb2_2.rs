
use std::{fs::File, io::{Read, Write}};


fn main() {

    let file_list = ["file1.txt", "file2.txt", "file3.txt"];
    let mut docs = Vec::new();
    for path in file_list {

        let mut file = File::open(path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        
        let mut word_count = 0;
        for _ in text.split_whitespace() {
            word_count += 1;
        }
        docs.push((path, word_count));
    }

    docs.sort_by(|a, b| b.1.cmp(&a.1));

    let mut html = String::new();
    html.push_str("<body>");
    html.push_str("<style>");
    html.push_str("table, th,td { border: 1px solid black; border-collapse : collapse; padding : 15px; }");
    html.push_str("</style>");
    html.push_str("<table>");
    html.push_str("<th>File Name</th> <th>Number of Words</th>");
    for (name, words) in docs {
        html.push_str(format!("<tr><td>{name}</td>  <td>{words}</td> </tr>").as_str());
    }

    html.push_str("</table>");
    html.push_str("</body>");

    let mut html_file = File::create("html_2_2.html").unwrap();
    html_file.write_all(html.as_bytes()).unwrap();
}


