mod emoji_search;

use clap::{Parser, ValueEnum};

/// A blazingly fast emoji search CLI for macOS
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query for emojis (can be multiple words)
    #[arg(default_value = "")]
    query: Vec<String>,

    /// Maximum number of results to return
    #[arg(short, long, default_value_t = 10)]
    limit: usize,

    /// Gender variant (for emojis that support it)
    #[arg(short, long, value_enum)]
    gender: Option<Gender>,

    /// Skin tone variant (for emojis that support it)
    #[arg(short, long, value_enum)]
    skin_tone: Option<SkinTone>,

    /// List all emojis including slang and substitutions
    #[arg(short = 'a', long)]
    list_all: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum Gender {
    Male,
    Female,
    Neutral,
}

impl From<Gender> for emoji_search::Gender {
    fn from(g: Gender) -> Self {
        match g {
            Gender::Male => emoji_search::Gender::Male,
            Gender::Female => emoji_search::Gender::Female,
            Gender::Neutral => emoji_search::Gender::Neutral,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum SkinTone {
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl From<SkinTone> for emoji_search::SkinTone {
    fn from(st: SkinTone) -> Self {
        match st {
            SkinTone::Light => emoji_search::SkinTone::Light,
            SkinTone::MediumLight => emoji_search::SkinTone::MediumLight,
            SkinTone::Medium => emoji_search::SkinTone::Medium,
            SkinTone::MediumDark => emoji_search::SkinTone::MediumDark,
            SkinTone::Dark => emoji_search::SkinTone::Dark,
        }
    }
}

fn main() {
    let args = Args::parse();

    // If --list-all flag is set, list all emojis and exit
    if args.list_all {
        println!("The --list-all flag is not yet implemented for the refactored version.");
        println!("Search for specific emojis using queries instead.");
        return;
    }

    // Join multi-word queries
    let query_joined = args.query.join(" ");

    // Check if query is empty
    if query_joined.trim().is_empty() {
        eprintln!("Error: Please provide a search query");
        std::process::exit(1);
    }

    let query_normalized = query_joined.replace(' ', "").to_lowercase();

    // Try normalized query first (spaces removed)
    let mut results = emoji_search::search_emojis(&query_normalized, args.limit);

    // If no results and query had spaces, try with spaces
    if results.is_empty() && query_joined.contains(' ') {
        results = emoji_search::search_emojis(&query_joined.to_lowercase(), args.limit);
    }

    // If still no results, try the original query as-is
    if results.is_empty() {
        results = emoji_search::search_emojis(&query_joined, args.limit);
    }

    if results.is_empty() {
        eprintln!("No emojis found for: {}", query_joined);
        std::process::exit(1);
    }

    for (keyword, emoji) in results {
        let emoji_str = emoji_search::get_emoji_string(&keyword, emoji);

        let mut modified_emoji = emoji_str;

        // Apply skin tone modifier if specified and supported
        if let Some(ref skin_tone) = args.skin_tone {
            modified_emoji = emoji_search::apply_skin_tone(emoji, &skin_tone.clone().into());
        }

        // Apply gender modifier if specified
        if let Some(ref gender) = args.gender {
            modified_emoji = emoji_search::apply_gender(&modified_emoji, &gender.clone().into());
        }

        // Display keyword without the __raw__ marker
        let display_keyword = if keyword.starts_with("__raw__:") {
            query_normalized.as_str()
        } else {
            keyword.as_str()
        };

        println!("{} ({})", modified_emoji, display_keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let results = emoji_search::search_emojis("heart", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_prefix_match() {
        let results = emoji_search::search_emojis("fir", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_substring_match() {
        let results = emoji_search::search_emojis("face", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_flag_search() {
        let results = emoji_search::search_emojis("aus", 10);
        assert!(!results.is_empty());
        // Should find Australia and Austria flags
    }

    #[test]
    fn test_limit() {
        let results = emoji_search::search_emojis("a", 5);
        assert!(results.len() <= 5);
    }
}
