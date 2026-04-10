use anyhow::{Context, Result};
use clap::Parser;
use docx_rs::*;
use pulldown_cmark::{Event, HeadingLevel, Parser as MdParser, Tag};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "md-to-docx")]
#[command(about = "Convert Markdown files to DOCX format")]
#[command(version = "0.1.0")]
struct Args {
    /// Input markdown file path
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output docx file path (optional, defaults to input name with .docx extension)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read markdown content
    let md_content = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {:?}", args.input))?;

    // Determine output path
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let stem = args.input.file_stem().unwrap_or_default();
            let default_parent = PathBuf::from(".");
            let parent = args.input.parent().unwrap_or(&default_parent);
            parent.join(format!("{}.docx", stem.to_string_lossy()))
        }
    };

    // Convert markdown to docx
    convert_md_to_docx(&md_content, &output_path)?;

    println!("✓ Converted {:?} → {:?}", args.input, output_path);

    Ok(())
}

fn convert_md_to_docx(md_content: &str, output_path: &PathBuf) -> Result<()> {
    let parser = MdParser::new(md_content);

    let mut docx = Docx::new()
        .add_style(
            Style::new("Heading1", StyleType::Paragraph)
                .name("Heading 1")
                .bold()
                .size(32)
        )
        .add_style(
            Style::new("Heading2", StyleType::Paragraph)
                .name("Heading 2")
                .bold()
                .size(26)
        )
        .add_style(
            Style::new("Heading3", StyleType::Paragraph)
                .name("Heading 3")
                .bold()
                .size(24)
        );

    let mut paragraphs: Vec<Paragraph> = Vec::new();
    let mut current_runs: Vec<Run> = Vec::new();
    let mut current_text = String::new();
    let mut in_code_block: Option<String> = None;
    let mut in_bold = false;
    let mut in_italic = false;
    let mut _in_heading: Option<HeadingLevel> = None;

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::CodeBlock(_) => {
                        in_code_block = Some(String::new());
                    }
                    Tag::Heading(level, _, _) => {
                        _in_heading = Some(level);
                    }
                    Tag::Strong => {
                        in_bold = true;
                    }
                    Tag::Emphasis => {
                        in_italic = true;
                    }
                    _ => {}
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::CodeBlock(_) => {
                        if let Some(code) = in_code_block.take() {
                            let code_para = Paragraph::new()
                                .add_run(Run::new().add_text(code.trim_end().to_string()))
                                .style("Code");
                            paragraphs.push(code_para);
                        }
                    }
                    Tag::Paragraph => {
                        if !current_text.is_empty() {
                            let run = create_run(&current_text, in_bold, in_italic);
                            current_runs.push(run);
                            current_text.clear();
                        }
                        if !current_runs.is_empty() {
                            let mut para = Paragraph::new();
                            for run in current_runs.drain(..) {
                                para = para.add_run(run);
                            }
                            paragraphs.push(para);
                        }
                    }
                    Tag::Heading(level, _, _) => {
                        if !current_text.is_empty() {
                            let style = match level {
                                HeadingLevel::H1 => "Heading1",
                                HeadingLevel::H2 => "Heading2",
                                _ => "Heading3",
                            };
                            let para = Paragraph::new()
                                .add_run(Run::new().add_text(current_text.clone()).bold())
                                .style(style);
                            paragraphs.push(para);
                            current_text.clear();
                        }
                        _in_heading = None;
                    }
                    Tag::Strong => {
                        in_bold = false;
                    }
                    Tag::Emphasis => {
                        in_italic = false;
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                if let Some(ref mut code) = in_code_block {
                    code.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                if !current_text.is_empty() {
                    let run = create_run(&current_text, in_bold, in_italic);
                    current_runs.push(run);
                    current_text.clear();
                }
                let code_run = Run::new()
                    .add_text(code.to_string())
                    .size(20);
                current_runs.push(code_run);
            }
            Event::SoftBreak | Event::HardBreak => {
                current_text.push('\n');
            }
            Event::Rule => {
                if !current_text.is_empty() || !current_runs.is_empty() {
                    let mut para = Paragraph::new();
                    for run in current_runs.drain(..) {
                        para = para.add_run(run);
                    }
                    if !current_text.is_empty() {
                        para = para.add_run(create_run(&current_text, in_bold, in_italic));
                    }
                    paragraphs.push(para);
                }
                let hr = Paragraph::new()
                    .add_run(Run::new().add_text("─".repeat(50)))
                    .align(AlignmentType::Center);
                paragraphs.push(hr);
                current_text.clear();
                current_runs.clear();
            }
            _ => {}
        }
    }

    // Add any remaining content
    if !current_text.is_empty() {
        let run = create_run(&current_text, in_bold, in_italic);
        current_runs.push(run);
    }
    if !current_runs.is_empty() {
        let mut para = Paragraph::new();
        for run in current_runs.drain(..) {
            para = para.add_run(run);
        }
        paragraphs.push(para);
    }

    // Build document
    for para in paragraphs {
        docx = docx.add_paragraph(para);
    }

    // Write to file
    let file = fs::File::create(output_path)
        .with_context(|| format!("Failed to create output file: {:?}", output_path))?;
    
    docx.build().pack(file)
        .with_context(|| "Failed to write DOCX file")?;

    Ok(())
}

fn create_run(text: &str, bold: bool, italic: bool) -> Run {
    let mut run = Run::new().add_text(text.to_string());
    if bold {
        run = run.bold();
    }
    if italic {
        run = run.italic();
    }
    run
}
