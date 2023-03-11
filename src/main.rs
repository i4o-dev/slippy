#![allow(clippy::uninlined_format_args)]

mod errors;
mod message;
mod slideshow;
use crate::errors::*;
use axohtml::{elements::section, html, unsafe_text};
use clap::{Parser, ValueEnum};
use message::Message;
use slideshow::{create_files, html::create_html, markdown::transform_markdown};
use std::{fs, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: String,
    #[arg(short, long)]
    theme: Option<Theme>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Theme {
    Light,
    Dark,
    Cupcake,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = Path::new(&args.file);
    if Path::exists(file_path) && file_path.extension().unwrap().to_str() == Some("md") {
        Message::new(message::MessageType::Info, "Creating your slideshow").print();
        let mut slides_html: Vec<Box<section<String>>> = vec![];
        let content = fs::read_to_string(file_path)?;
        let slides: Vec<&str> = content.as_str().split("\n---\n").collect();

        for slide in slides {
            let slide_html = transform_markdown(slide);
            slides_html.extend(html!(<section><div>{unsafe_text!(slide_html)}</div></section>))
        }

        let final_html = create_html(slides_html);
        create_files(final_html, args.theme)?;
        Message::new(message::MessageType::Success, "Your slides are done!").print();
    } else {
        AxoSlidesError::FileNotFound {
            filedesc: "markdown slideshow".to_owned(),
            path: args.file,
        };
    }

    Ok(())
}
