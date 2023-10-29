use colored::Colorize;

/// Trait for displaying hasty lang errors.
pub trait HastyError {
    fn as_hasty_error_string(&self) -> String;
    fn get_error_description(&self) -> String;
}

/// Shows error in a unified form:
/// \[<pipeline_part>] Error \<line>.\<char>: \<error>
///
/// <line_preview>
///
/// <error_pointer>
pub fn unified_error(
    pipeline_part: &str, error: &str,
    line: usize, start: usize,
    lexeme: &str, line_str: &str,
) -> String {
    let mut result = String::new();

    // Error info.
    result.push_str(
        &format!("[{}] {} {}{}{}{} {}\n",
            pipeline_part.yellow(),
            "Error".red(),
            line.to_string().red(),
            ".".red(),
            start.to_string().red(),
            ":".red(),
            error.red()
        )
    );

    if line_str == "" { return result; }

    // Line preview.
    result.push_str(line_str.trim());
    result.push('\n');

    // Error indicator.
    let indicator = "^".to_string().repeat(lexeme.len()).yellow();
    let whitespaces = " ".to_string().repeat(start);

    result.push_str(&format!("{}{} {}", whitespaces, indicator, "Here".to_string().yellow()));

    result
}