use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::io::BufWriter;
use std::env;

struct StreamReader<'a> {
    chars: Vec<char>,
    previous: Option<char>,
    current: Option<char>,
    next: Option<char>,
    reader: &'a mut BufRead,
}

impl<'a> StreamReader<'a> {
    pub fn new(input: &mut BufRead) -> StreamReader {
        StreamReader {
            chars: vec!(),
            previous: None,
            current: None,
            next: None,
            reader: input,
        }
    }
}

impl<'a> Iterator for StreamReader<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.chars.len() == 0 {
            // Buffer next line
            self.chars.clear();

            let mut line = String::new();
            if self.reader.read_line(&mut line).unwrap() == 0 {
                return None::<char>;
            }

            println!("line: {}", line);

            for c in line.chars() {
                self.chars.push(c);
            }
        }

        // The first time this function is called, the next block needs to run
        // twice (otherwise it would return with only self.next set).
        let times = match self.current.is_none() {
            false => 1,
            true  => 2,
        };

        for _ in 0..times {
            self.previous = self.current;
            self.current = self.next;
            self.next = Some(self.chars.remove(0));
        }

        self.current
    }
}

fn process(input: &mut Read, output: &mut Write) -> Result<(), io::Error> {
    const IGNORED_TAGS: &'static [ &'static str ] = &[ "script", "style" ];

    for it in IGNORED_TAGS {
        println!("{}", it);
    }

    //let mut reader = BufReader::new(input);
    //let mut writer = BufWriter::new(output);
    let mut bufreader = BufReader::new(input);
    let mut reader = StreamReader::new(&mut bufreader);
    let mut buffer = String::new();

    let mut in_quotes = false;
    let mut in_tag = false;

    let mut tag = String::new();

    let mut in_ignored_tag = false;
    let mut ignored_tag = "";

    for char in reader {
        println!("{}", char);
/*
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

        buffer.push(char);*/
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
