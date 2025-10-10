use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
enum ProcessingError {
    Io(io::Error),
    EmptyFile(String),
    InvalidContent { filename: String, reason: String },
    OutputWriteFailed(String),
}

impl From<io::Error> for ProcessingError {
    fn from(error: io::Error) -> Self {
        ProcessingError::Io(error)
    }
}

fn read_file_contents(filename: &str) -> Result<String, ProcessingError> {
    let mut file = File::open(filename)?;  // Auto-converts io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    if contents.trim().is_empty() {
        return Err(ProcessingError::EmptyFile(filename.to_string()));
    }
    
    Ok(contents)
}

fn process_text(text: &str, filename: &str) -> Result<String, ProcessingError> {
    // Validate content
    if text.len() > 1_000_000 {
        return Err(ProcessingError::InvalidContent {
            filename: filename.to_string(),
            reason: String::from("File too large (max 1MB)"),
        });
    }
    
    // Process: convert to uppercase and count words
    let processed = text.to_uppercase();
    let word_count = text.split_whitespace().count();
    
    let result = format!(
        "=== PROCESSED FILE: {} ===\nWord Count: {}\n\n{}",
        filename, word_count, processed
    );
    
    Ok(result)
}

fn save_processed_file(content: &str, output_filename: &str) -> Result<(), ProcessingError> {
    let mut file = File::create(output_filename)?;
    file.write_all(content.as_bytes())?;
    
    println!("Saved to: {}", output_filename);
    Ok(())
}

fn process_file_pipeline(input: &str, output: &str) -> Result<(), ProcessingError> {
    // Each step can fail, and errors propagate up
    let contents = read_file_contents(input)?;
    let processed = process_text(&contents, input)?;
    save_processed_file(&processed, output)?;
    
    println!("Successfully processed {} -> {}", input, output);
    Ok(())
}

// Process multiple files
fn batch_process(files: Vec<(&str, &str)>) -> Result<usize, ProcessingError> {
    let mut successful = 0;
    
    for (input, output) in files {
        match process_file_pipeline(input, output) {
            Ok(()) => successful += 1,
            Err(e) => {
                println!("Failed to process {}: {:?}", input, e);
                // Continue processing other files, or return error?
                // For now, we'll continue
            }
        }
    }
    
    if successful == 0 {
        Err(ProcessingError::OutputWriteFailed(
            String::from("Failed to process any files")
        ))
    } else {
        Ok(successful)
    }
}

// Usage:
match process_file_pipeline("input.txt", "output.txt") {
    Ok(()) => println!("Processing complete!"),
    Err(ProcessingError::Io(e)) => {
        println!("IO error: {}", e);
    }
    Err(ProcessingError::EmptyFile(filename)) => {
        println!("File is empty: {}", filename);
    }
    Err(ProcessingError::InvalidContent { filename, reason }) => {
        println!("Invalid content in {}: {}", filename, reason);
    }
    Err(e) => {
        println!("Processing error: {:?}", e);
    }
}