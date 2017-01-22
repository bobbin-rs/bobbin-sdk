extern crate xml;
extern crate clap;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};
use clap::{Arg, App};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let matches = App::new("svd2chip")
        .arg(Arg::with_name("input"))
        .get_matches();
    if !matches.is_present("input") {
        println!("{}", matches.usage());
    }
    let input = matches.value_of("input").unwrap();

    let file = File::open(input).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}({}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
                println!("{})", indent(depth));
            }
            Ok(XmlEvent::Characters(data)) => {
                println!("{}{}", indent(depth), data);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }        
    }


}