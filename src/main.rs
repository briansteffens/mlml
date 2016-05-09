use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::io::BufWriter;
use std::env;

struct StreamReader<'a> {
    curr: u32,
    reader: &'a mut Read,
}

impl<'a> StreamReader<'a> {
    pub fn new(input: &mut Read) -> StreamReader {
        StreamReader {
            curr: 123,
            reader: input,
        }
    }
}
/*
impl Iterator for StreamReader {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let mut buffer = String::new();
        self.reader.read_line(buffer);
    }
}
*/
fn process(input: &mut Read, output: &mut Write) -> Result<(), io::Error> {
    const IGNORED_TAGS: &'static [ &'static str ] = &[ "script", "style" ];

    for it in IGNORED_TAGS {
        println!("{}", it);
    }

    let mut reader = BufReader::new(input);
    //let mut writer = BufWriter::new(output);
    let mut buffer = String::new();

    let mut in_quotes = false;
    let mut in_tag = false;

    let mut tag = String::new();

    let mut in_ignored_tag = false;
    let mut ignored_tag = "";

    for line in reader.lines() {
        for char in line.unwrap().chars() {
            println!("{}", char);

            if char == '"' {
                in_quotes = !in_quotes;
                println!("in_quotes: {}", in_quotes);
            }

            if !in_quotes {
                // Tag start
                if char == '<' {
                    in_tag = true;
                }

                // Tag stop
                if char == '>' {
                    in_tag = false;

                    if !tag.starts_with("/") {
                        // Starting tag
                        for ignored in IGNORED_TAGS {
                            if tag.starts_with(ignored) {
                                in_ignored_tag = true;
                                ignored_tag = ignored;
                                println!("IGNORING {}", ignored_tag);
                            }
                        }
                    } else {
                        // Closing tag
                        let tag_offset = &tag.clone()[1..];

                        if tag_offset.starts_with(ignored_tag) {
                            in_ignored_tag = false;
                            println!("NOT IGNORING");
                        }
                    }

                    tag.clear();
                }
            }

            if in_tag && char != '<' {
                tag.push(char);
                println!("tag: {}", tag);
            }

            buffer.push(char);
        }
        buffer.push('\n');
    }
    println!("{}", buffer);
/*
    loop {
        let bytes_read = try!(reader.read_line(&mut buffer));

        if bytes_read == 0 {
            break;
        }

        try!(writer.write_all(buffer.as_bytes()));
        buffer.clear();
    }*/

    Ok(())
}

fn make_output_filename(source_fn: &String) -> String {
    match source_fn.ends_with(".mlml") {
        true => source_fn.clone().replace(".mlml", "") + ".html",
        false => source_fn.clone() + ".html"
    }
}

fn main() {
    if env::args().count() == 2 {
        // File-mode
        let source_fn = env::args().skip(1).next().unwrap();
        let output_fn = make_output_filename(&source_fn);

        let mut source_file = File::open(source_fn)
                .expect("Failed to open input file");

        let mut output_file = File::create(output_fn)
                .expect("Failed to open output file");

        process(&mut source_file, &mut output_file).unwrap();
    } else {
        // STDIN/STDOUT-mode
        process(&mut io::stdin(), &mut io::stdout()).unwrap();
    }
}
