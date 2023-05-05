use std::fs;
use std::path::Path;


#[derive(Default, Debug)]
struct Book {
    title: String,
    sections: Vec<Section>
}

#[derive(Default, Clone, Debug)]
struct Section {
    title: String,
    paragraph: Vec<String>
}

// TODO: parse file

fn main() {
    // let file_path = "files/test.fb2";
    let file_path = "files/t.fb2";

    if !Path::new(file_path).exists() {
        eprintln!("File {:?} does not exists!", file_path);
        return;
    }

    let content = fs::read_to_string(file_path).unwrap();

    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();

    // TODO: Need to check if < or > could be inside of a tag
    // TODO: What to do with `newline`?
    for c in content.chars() {
        match c {
            '<' => { // start of tag
                if current_token.is_empty() {
                    // If token is not empty, that means we have an actual text inside of the
                    // current_token and we need to push it before we start parsing tag
                    current_token.push(c);
                    continue;
                }

                tokens.push(current_token.clone());
                current_token = c.to_string();
            }
            '>' => { // end of tag
                current_token.push(c);
                tokens.push(current_token.clone());
                current_token.clear();
            }
            _ => {
                current_token.push(c);
            },
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token.clone());
    }


    let mut book = Book::default();

    let mut index = 0;

    let mut current_section: Option<Section> = None;

    while index < tokens.len() {
        let token = tokens[index].as_str();

        match token {
            "<section>" => {
                current_section = Some(Section::default());
            },
            "</section>" => {
                match current_section {
                    Some(value) => {
                        book.sections.push(value.clone());
                    },
                    None => {
                        eprintln!("Found {} tag but current section is not found", token);
                    },
                }
                current_section = None;
            },
            _ => {
                eprintln!("Unknown token: {}", token);
            }
        }

        index += 1;
    }

    println!("Book: {:?}", book)

}
