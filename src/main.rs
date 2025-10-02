use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    description: Option<String>,
    globs: Vec<String>,
    content: String,
    collection: String,
}

#[derive(Debug, Clone, ValueEnum, Default)]
enum InputFormat {
    #[default]
    Cline,
}

#[derive(Debug, Clone, ValueEnum, Default)]
enum Format {
    #[default]
    Cline,
    Agents,
    Qwen,
    Claude,
    Gemini,
    Cursor,
    Windsurf,
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Target directory where rules will be copied
    #[clap(short, long)]
    target: PathBuf,

    /// Input format (currently only .clinerules supported)
    #[clap(short = 'i', long, value_enum, default_value = "cline")]
    input: InputFormat,

    /// Collections to process (comma-separated)
    #[clap(short = 'c', long, value_delimiter = ',')]
    collections: Option<Vec<String>>,

    /// Exclude specific rule files (comma-separated, without extensions)
    #[clap(short = 'e', long, value_delimiter = ',')]
    exclude: Option<Vec<String>>,

    /// Formats for output (comma-separated)
    #[clap(short = 'F', long, value_enum, value_delimiter = ',')]
    format: Vec<Format>,

    /// Force overwrite if target already exists
    #[clap(short = 'f', long = "force")]
    force: bool,

    /// List available collections
    #[clap(long)]
    list: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parse rules from input format
    let rules = parse_rules(&args.input)?;

    // List collections if requested
    if args.list {
        let collections: std::collections::HashSet<_> =
            rules.iter().map(|r| &r.collection).collect();
        for collection in collections {
            println!("{}", collection);
        }
        return Ok(());
    }

    // Filter rules by collections if specified
    let filtered_rules = if let Some(collections) = &args.collections {
        rules
            .iter()
            .filter(|r| collections.contains(&r.collection))
            .cloned()
            .collect()
    } else {
        rules
    };

    // Filter out excluded rule files if specified
    let final_rules = if let Some(excluded_files) = &args.exclude {
        filtered_rules
            .iter()
            .filter(|r| !excluded_files.contains(&r.name))
            .cloned()
            .collect()
    } else {
        filtered_rules
    };

    // If no formats specified, default to Cline format
    let formats = if args.format.is_empty() {
        vec![Format::default()]
    } else {
        args.format
    };

    // Process each specified format
    for format in &formats {
        process_format(format, &final_rules, &args.target, args.force)?;
    }

    Ok(())
}

fn parse_rules(input_format: &InputFormat) -> Result<Vec<Rule>> {
    match input_format {
        InputFormat::Cline => {
            let source = PathBuf::from(".clinerules");

            if !source.exists() {
                anyhow::bail!("Error: .clinerules directory not found");
            }

            let mut rules = Vec::new();
            let entries =
                std::fs::read_dir(&source).context("Failed to read .clinerules directory")?;

            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    if let Some(collection_name) = entry.file_name().to_str() {
                        let collection_rules = parse_collection_rules(&source, collection_name)?;
                        rules.extend(collection_rules);
                    }
                }
            }

            Ok(rules)
        }
    }
}

fn parse_collection_rules(source: &Path, collection: &str) -> Result<Vec<Rule>> {
    let collection_dir = source.join(collection);
    let mut rules = Vec::new();
    let entries = std::fs::read_dir(&collection_dir)
        .with_context(|| format!("Failed to read collection directory: {}", collection))?;

    for entry in entries.flatten() {
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            let file_path = entry.path();
            if let Ok(file_content) = std::fs::read_to_string(&file_path) {
                // Extract metadata from file content (you could parse frontmatter here)
                let (description, globs, content) = extract_metadata(&file_content);

                let rule_name = file_path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                rules.push(Rule {
                    name: rule_name,
                    description,
                    globs,
                    content,
                    collection: collection.to_string(),
                });
            }
        }
    }

    Ok(rules)
}

fn extract_metadata(content: &str) -> (Option<String>, Vec<String>, String) {
    // Simple implementation - you could extend this to parse YAML frontmatter
    let mut lines = content.lines().peekable();
    let mut description = None;
    let mut globs = Vec::new();

    // Look for special comments at the top
    while let Some(line) = lines.peek() {
        if line.starts_with("# Description:") {
            description = Some(
                line.strip_prefix("# Description: ")
                    .unwrap_or("")
                    .trim()
                    .to_string(),
            );
            lines.next();
        } else if line.starts_with("# Globs:") {
            let glob_line = line.strip_prefix("# Globs: ").unwrap_or("");
            globs = glob_line.split(',').map(|s| s.trim().to_string()).collect();
            lines.next();
        } else {
            break;
        }
    }

    let processed_content = if content.starts_with("---") {
        // Remove YAML frontmatter if present
        let lines: Vec<&str> = content.lines().collect();
        let mut frontmatter_end = None;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                frontmatter_end = Some(i + 1);
                break;
            }
        }

        if let Some(end) = frontmatter_end {
            lines[end..].join("\n")
        } else {
            content.to_string()
        }
    } else {
        content.to_string()
    };

    (description, globs, processed_content)
}

fn process_format(format: &Format, rules: &[Rule], target: &Path, force: bool) -> Result<()> {
    match format {
        Format::Cline => write_directory_format(rules, target, ".clinerules", force)?,
        Format::Agents => write_single_file_format(rules, target, "AGENTS.md", "AGENTS", force)?,
        Format::Qwen => write_single_file_format(rules, target, "QWEN.md", "QWEN", force)?,
        Format::Claude => write_single_file_format(rules, target, "CLAUDE.md", "CLAUDE", force)?,
        Format::Gemini => write_single_file_format(rules, target, "GEMINI.md", "GEMINI", force)?,
        Format::Cursor => write_directory_format(rules, target, ".cursor/rules", force)?,
        Format::Windsurf => write_directory_format(rules, target, ".windsurf/rules", force)?,
    }
    Ok(())
}

fn write_directory_format(
    rules: &[Rule],
    target: &Path,
    relative_path: &str,
    force: bool,
) -> Result<()> {
    let target_dir = target.join(relative_path);

    if !force && target_dir.exists() {
        anyhow::bail!(
            "Target directory {} already exists. Use -f to force overwrite.",
            target_dir.display()
        );
    }

    std::fs::create_dir_all(&target_dir)?;
    write_cline_format(rules, &target_dir)?;
    Ok(())
}

fn write_single_file_format(
    rules: &[Rule],
    target: &Path,
    filename: &str,
    title: &str,
    force: bool,
) -> Result<()> {
    let target_file = target.join(filename);

    if !force && target_file.exists() {
        anyhow::bail!(
            "Target file {} already exists. Use -f to force overwrite.",
            target_file.display()
        );
    }

    let content = create_agents_content(rules, title);
    std::fs::write(&target_file, content)?;
    Ok(())
}

fn write_cline_format(rules: &[Rule], target_dir: &Path) -> Result<()> {
    // Group rules by collection
    let mut collections: std::collections::HashMap<String, Vec<&Rule>> =
        std::collections::HashMap::new();
    for rule in rules {
        collections
            .entry(rule.collection.clone())
            .or_default()
            .push(rule);
    }

    // Write each collection as a directory
    for (collection_name, collection_rules) in collections {
        let collection_dir = target_dir.join(collection_name);
        std::fs::create_dir_all(&collection_dir)?;

        for rule in collection_rules {
            let file_path = collection_dir.join(&rule.name);
            std::fs::write(&file_path, &rule.content)?;
        }
    }

    Ok(())
}

fn create_agents_content(rules: &[Rule], title: &str) -> String {
    let mut content = format!("# {}\n\n", title);

    // Group rules by collection
    let mut collections: std::collections::HashMap<String, Vec<&Rule>> =
        std::collections::HashMap::new();
    for rule in rules {
        collections
            .entry(rule.collection.clone())
            .or_default()
            .push(rule);
    }

    // Write each collection
    for (collection_name, collection_rules) in collections {
        content.push_str(&format!("## {}\n\n", collection_name));

        for rule in collection_rules {
            if let Some(description) = &rule.description {
                content.push_str(&format!("### {}\n\n{}\n\n", rule.name, description));
            } else {
                content.push_str(&format!("### {}\n\n", rule.name));
            }

            if !rule.globs.is_empty() {
                content.push_str(&format!("**Applies to:** {}\n\n", rule.globs.join(", ")));
            }

            content.push_str(&rule.content);
            content.push_str("\n\n---\n\n");
        }
    }

    content
}
