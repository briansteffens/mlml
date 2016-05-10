use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::io::BufWriter;
use std::env;

struct StreamFrame {
    previous: Option<char>,
    current: Option<char>,
    next: Option<char>,
}

struct StreamReader<'a> {
    chars: Vec<char>,
    frame: StreamFrame,
    stream_ended: bool,
    reader: &'a mut BufRead,
}

impl<'a> StreamReader<'a> {
    pub fn new(input: &mut BufRead) -> StreamReader {
        StreamReader {
            chars: vec!(),
            frame: StreamFrame {
                previous: None,
                current: None,
                next: None,
            },
            stream_ended: false,
            reader: input,
        }
    }
}

impl<'a> Iterator for StreamReader<'a> {
    type Item = StreamFrame;

    fn next(&mut self) -> Option<StreamFrame> {
        if !self.stream_ended && self.chars.len() == 0 {
            // Buffer next line
            self.chars.clear();

            let mut line = String::new();

            if self.reader.read_line(&mut line).unwrap() == 0 {
                self.stream_ended = true;
            } else {
                for c in line.chars() {
                    self.chars.push(c);
                }
            }
        }

        // The first time this function is called, the next block needs to run
        // twice (otherwise it would return with only self.next set).
        let times = match self.frame.current.is_none() {
            false => 1,
            true  => 2,
        };

        for _ in 0..times {
            self.frame.previous = self.frame.current;
            self.frame.current = self.frame.next;
            self.frame.next = match self.stream_ended {
                false => Some(self.chars.remove(0)),
                true  => None
            };
        }

        if self.frame.current.is_none() {
            return None;
        }

        Some(StreamFrame {
            previous: self.frame.previous,
            current: self.frame.current,
            next: self.frame.next,
        })
    }
}

fn process(input: &mut Read, output: &mut Write) -> Result<(), io::Error> {
    const IGNORED_TAGS: &'static [ &'static str ] = &[ "script", "style" ];

    //let mut writer = BufWriter::new(output);
    let mut bufreader = BufReader::new(input);
    let reader = StreamReader::new(&mut bufreader);
    let mut buffer: Vec<char> = vec!();

    let mut in_double_quotes = false;
    let mut in_single_quotes = false;
    let mut in_tag = false;

    let mut tag = String::new();

    let mut in_ignored_tag = false;
    let mut ignored_tag = "";

    let mut line_continuation = false;
    let mut skip_chars = 0;

    for frame in reader {
        let current = frame.current.unwrap();

        if line_continuation && !current.is_whitespace() && current != '"' {
            panic!("Syntax error - expected double-quote line continuation");
        }

        let previous_escaped = !frame.previous.is_none() &&
                                frame.previous.unwrap() == '\\';

        if !in_single_quotes && current == '"' && !previous_escaped {
            in_double_quotes = !in_double_quotes;

            // Complete line continuation
            if line_continuation {
                line_continuation = false;
                skip_chars = 1;
            }
        }

        if !in_double_quotes && current == '\'' && !previous_escaped {
            in_single_quotes = !in_single_quotes;
        }

        if !in_double_quotes && !in_single_quotes {
            // Tag start
            if current == '<' {
                in_tag = true;
            }

            // Tag stop
            if current == '>' {
                in_tag = false;

                if !tag.starts_with("/") {
                    // Starting tag
                    for ignored in IGNORED_TAGS {
                        if tag.starts_with(ignored) {
                            in_ignored_tag = true;
                            ignored_tag = ignored;
                        }
                    }
                } else {
                    // Closing tag
                    let tag_offset = &tag.clone()[1..];

                    if tag_offset.starts_with(ignored_tag) {
                        in_ignored_tag = false;
                    }
                }

                tag.clear();
            }
        }

        if in_tag && current != '<' {
            tag.push(current);
        }

        if in_tag && !in_ignored_tag && !in_double_quotes &&
                !in_single_quotes && current == '\n' {

            // A line continuation must have a double-quote, optional
            // whitespace, a plus sign, optional whitespace, and a newline.
            // Check the end of the most recent line in the buffer for this
            // pattern.

            let mut found_plus = false;

            for n in (0..buffer.len()).rev() {
                if buffer[n].is_whitespace() {
                    continue;
                }

                // Line continuation character
                if buffer[n] == '+' {
                    // Only one plus is valid
                    if found_plus {
                        break;
                    }

                    found_plus = true;
                }

                // Found a line continuation, remove end of this line
                if buffer[n] == '"' && found_plus {
                    buffer.truncate(n);
                    line_continuation = true;
                    break;
                }
            }
        }

        // Skip characters if we are currently processing a line continuation.
        if !line_continuation {
            if skip_chars == 0 {
                buffer.push(current);
            }

            if skip_chars > 0 {
                skip_chars -= 1;
            }
        }
    }

    let mut asdf = String::new();
    for c in buffer {
        asdf.push(c.clone());
    }
    println!("{}", asdf);

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
