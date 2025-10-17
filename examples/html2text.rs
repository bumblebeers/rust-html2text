use html2text::config::Config;
use html2text::render::text_renderer::TrivialDecorator;
use std::io::{self, Read};
use clap::Parser;
use std::fs;

/// A command-line tool to convert HTML to plain text.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to a CSS file to apply to the HTML
    #[arg(short, long)]
    css: Option<String>,

    /// Width of the output in characters
    #[arg(short, long, default_value_t = 80)]
    width: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut html = Vec::new();
    io::stdin().read_to_end(&mut html)?;

    // The TrivialDecorator is used for plain text output.
    let decorator = TrivialDecorator::new();

    // This uses a function from the config module to create the Config object.
    let mut config = html2text::config::with_decorator(decorator);

    // This tells the converter to also look for <style> tags in the HTML.
    config = config.use_doc_css();

    // If a CSS file path is provided, read the file and add it to the config.
    if let Some(css_path) = args.css {
        let css = fs::read_to_string(css_path)?;
        config = config.add_css(&css)?;
    }

    // Perform the conversion.
    let text = config.string_from_read(&mut html.as_slice(), args.width)?;

    println!("{}", text);

    Ok(())
}
