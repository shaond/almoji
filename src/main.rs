use clap::{Parser, ValueEnum};
use std::collections::HashMap;

/// A blazingly fast emoji search CLI for macOS
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query for emojis (can be multiple words)
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
}

#[derive(Debug, Clone, ValueEnum)]
enum Gender {
    Male,
    Female,
    Neutral,
}

#[derive(Debug, Clone, ValueEnum)]
enum SkinTone {
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

/// Search for emojis matching the query using the comprehensive Unicode emoji database
fn search_emojis(query: &str, limit: usize) -> Vec<(String, &'static emojis::Emoji)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut seen = HashMap::new();

    // First pass: exact matches on name
    for emoji in emojis::iter() {
        if results.len() >= limit {
            break;
        }

        // Check name (e.g., "smiling face")
        if emoji.name().to_lowercase() == query_lower {
            let key = emoji.as_str().to_string();
            if !seen.contains_key(&key) {
                results.push((emoji.name().to_lowercase().replace(' ', ""), emoji));
                seen.insert(key, true);
            }
        }

        // Check shortcodes (e.g., ":smile:")
        for shortcode in emoji.shortcodes() {
            if shortcode.trim_matches(':').to_lowercase() == query_lower {
                let key = emoji.as_str().to_string();
                if !seen.contains_key(&key) {
                    results.push((shortcode.trim_matches(':').to_string(), emoji));
                    seen.insert(key, true);
                    break;
                }
            }
        }
    }

    // Second pass: prefix matches
    if results.len() < limit {
        for emoji in emojis::iter() {
            if results.len() >= limit {
                break;
            }

            let key = emoji.as_str().to_string();
            if seen.contains_key(&key) {
                continue;
            }

            // Check if name starts with query
            let name_normalized = emoji.name().to_lowercase();
            if name_normalized.starts_with(&query_lower) {
                results.push((name_normalized.replace(' ', ""), emoji));
                seen.insert(key.clone(), true);
                continue;
            }

            // Check if any word in the name starts with query
            if name_normalized.split_whitespace().any(|word| word.starts_with(&query_lower)) {
                results.push((name_normalized.replace(' ', ""), emoji));
                seen.insert(key.clone(), true);
                continue;
            }

            // Check shortcodes for prefix matches
            for shortcode in emoji.shortcodes() {
                let sc = shortcode.trim_matches(':').to_lowercase();
                if sc.starts_with(&query_lower) {
                    results.push((sc, emoji));
                    seen.insert(key.clone(), true);
                    break;
                }
            }
        }
    }

    // Third pass: substring matches
    if results.len() < limit {
        for emoji in emojis::iter() {
            if results.len() >= limit {
                break;
            }

            let key = emoji.as_str().to_string();
            if seen.contains_key(&key) {
                continue;
            }

            // Check if name contains query
            let name_normalized = emoji.name().to_lowercase();
            if name_normalized.contains(&query_lower) {
                results.push((name_normalized.replace(' ', ""), emoji));
                seen.insert(key.clone(), true);
                continue;
            }

            // Check shortcodes for substring matches
            for shortcode in emoji.shortcodes() {
                let sc = shortcode.trim_matches(':').to_lowercase();
                if sc.contains(&query_lower) {
                    results.push((sc, emoji));
                    seen.insert(key.clone(), true);
                    break;
                }
            }
        }
    }

    results
}

/// Apply skin tone modifier to emoji if applicable
fn apply_skin_tone(emoji: &emojis::Emoji, skin_tone: &SkinTone) -> String {
    let base = emoji.as_str();

    // Check if emoji has skin tone variants
    if emoji.skin_tones().is_some() {
        let modifier = match skin_tone {
            SkinTone::Light => "\u{1F3FB}",       // 🏻
            SkinTone::MediumLight => "\u{1F3FC}", // 🏼
            SkinTone::Medium => "\u{1F3FD}",      // 🏽
            SkinTone::MediumDark => "\u{1F3FE}",  // 🏾
            SkinTone::Dark => "\u{1F3FF}",        // 🏿
        };

        // Append skin tone modifier to the emoji
        return format!("{}{}", base, modifier);
    }

    base.to_string()
}

/// Apply gender modifier to emoji if applicable
fn apply_gender(emoji_str: &str, gender: &Gender) -> String {
    match gender {
        Gender::Male => {
            // Add male variant using ZWJ sequence
            format!("{}\u{200D}♂\u{FE0F}", emoji_str)
        }
        Gender::Female => {
            // Add female variant using ZWJ sequence
            format!("{}\u{200D}♀\u{FE0F}", emoji_str)
        }
        Gender::Neutral => emoji_str.to_string(),
    }
}

fn main() {
    let args = Args::parse();

    // Join multi-word queries
    let query_joined = args.query.join(" ");
    let query_normalized = query_joined.replace(' ', "").to_lowercase();

    // Try normalized query first (spaces removed)
    let mut results = search_emojis(&query_normalized, args.limit);

    // If no results and query had spaces, try with spaces
    if results.is_empty() && query_joined.contains(' ') {
        results = search_emojis(&query_joined.to_lowercase(), args.limit);
    }

    // If still no results, try the original query as-is
    if results.is_empty() {
        results = search_emojis(&query_joined, args.limit);
    }

    if results.is_empty() {
        eprintln!("No emojis found for: {}", query_joined);
        std::process::exit(1);
    }

    for (keyword, emoji) in results {
        let mut modified_emoji = emoji.as_str().to_string();

        // Apply skin tone modifier if specified and supported
        if let Some(ref skin_tone) = args.skin_tone {
            modified_emoji = apply_skin_tone(emoji, skin_tone);
        }

        // Apply gender modifier if specified
        if let Some(ref gender) = args.gender {
            modified_emoji = apply_gender(&modified_emoji, gender);
        }

        println!("{} ({})", modified_emoji, keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let results = search_emojis("heart", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_prefix_match() {
        let results = search_emojis("fir", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_substring_match() {
        let results = search_emojis("face", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_flag_search() {
        let results = search_emojis("aus", 10);
        assert!(!results.is_empty());
        // Should find Australia and Austria flags
    }

    #[test]
    fn test_limit() {
        let results = search_emojis("a", 5);
        assert!(results.len() <= 5);
    }
}
