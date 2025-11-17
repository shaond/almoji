use clap::Parser;
use phf::phf_map;

/// A blazingly fast emoji search CLI for macOS
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query for emojis
    query: String,

    /// Maximum number of results to return
    #[arg(short, long, default_value_t = 10)]
    limit: usize,
}

/// Static compile-time hash map for extremely fast keyword -> emoji lookups
/// This uses perfect hash functions for O(1) lookup performance
static EMOJI_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    // Smileys & Emotion
    "smile" => "😊",
    "happy" => "😊",
    "grin" => "😀",
    "laugh" => "😂",
    "lol" => "😂",
    "rofl" => "🤣",
    "joy" => "😂",
    "heart" => "❤️",
    "love" => "❤️",
    "hearts" => "💕",
    "kiss" => "😘",
    "wink" => "😉",
    "blush" => "😊",
    "hug" => "🤗",
    "thinking" => "🤔",
    "hmm" => "🤔",
    "cool" => "😎",
    "sunglasses" => "😎",
    "star" => "⭐",
    "stars" => "✨",
    "sparkles" => "✨",
    "fire" => "🔥",
    "hot" => "🔥",
    "100" => "💯",
    "hundred" => "💯",
    "ok" => "👌",
    "thumbsup" => "👍",
    "thumbs" => "👍",
    "thumbsdown" => "👎",
    "clap" => "👏",
    "applause" => "👏",
    "pray" => "🙏",
    "please" => "🙏",
    "thanks" => "🙏",

    // Emotions
    "sad" => "😢",
    "cry" => "😢",
    "tear" => "😢",
    "angry" => "😠",
    "mad" => "😡",
    "rage" => "😡",
    "confused" => "😕",
    "worried" => "😟",
    "shock" => "😱",
    "scream" => "😱",
    "surprised" => "😮",
    "tired" => "😫",
    "sleepy" => "😴",
    "sleep" => "😴",
    "sick" => "🤒",
    "ill" => "🤢",
    "nausea" => "🤢",
    "vomit" => "🤮",
    "dead" => "💀",
    "skull" => "💀",
    "ghost" => "👻",
    "alien" => "👽",
    "robot" => "🤖",

    // Gestures
    "wave" => "👋",
    "hi" => "👋",
    "hello" => "👋",
    "bye" => "👋",
    "point" => "👉",
    "fingers" => "🤞",
    "crossed" => "🤞",
    "luck" => "🤞",
    "fist" => "✊",
    "punch" => "👊",
    "muscle" => "💪",
    "strong" => "💪",
    "strength" => "💪",
    "flex" => "💪",

    // People
    "man" => "👨",
    "woman" => "👩",
    "boy" => "👦",
    "girl" => "👧",
    "baby" => "👶",
    "child" => "👶",
    "person" => "🧑",
    "people" => "👥",
    "family" => "👨‍👩‍👧‍👦",
    "couple" => "👫",

    // Animals & Nature
    "dog" => "🐕",
    "cat" => "🐈",
    "kitty" => "🐱",
    "puppy" => "🐶",
    "mouse" => "🐭",
    "hamster" => "🐹",
    "rabbit" => "🐰",
    "bunny" => "🐰",
    "fox" => "🦊",
    "bear" => "🐻",
    "panda" => "🐼",
    "koala" => "🐨",
    "tiger" => "🐯",
    "lion" => "🦁",
    "cow" => "🐮",
    "pig" => "🐷",
    "frog" => "🐸",
    "monkey" => "🐵",
    "penguin" => "🐧",
    "bird" => "🐦",
    "duck" => "🦆",
    "eagle" => "🦅",
    "owl" => "🦉",
    "bat" => "🦇",
    "wolf" => "🐺",
    "unicorn" => "🦄",
    "horse" => "🐴",
    "bee" => "🐝",
    "bug" => "🐛",
    "butterfly" => "🦋",
    "snail" => "🐌",
    "snake" => "🐍",
    "dragon" => "🐉",
    "dinosaur" => "🦕",
    "dino" => "🦕",
    "turtle" => "🐢",
    "fish" => "🐟",
    "dolphin" => "🐬",
    "whale" => "🐳",
    "shark" => "🦈",
    "octopus" => "🐙",
    "crab" => "🦀",
    "tree" => "🌲",
    "palm" => "🌴",
    "cactus" => "🌵",
    "flower" => "🌸",
    "rose" => "🌹",
    "tulip" => "🌷",
    "sunflower" => "🌻",
    "blossom" => "🌼",
    "leaf" => "🍃",
    "leaves" => "🍂",
    "mushroom" => "🍄",

    // Food & Drink
    "pizza" => "🍕",
    "burger" => "🍔",
    "hamburger" => "🍔",
    "fries" => "🍟",
    "hotdog" => "🌭",
    "taco" => "🌮",
    "burrito" => "🌯",
    "sushi" => "🍣",
    "ramen" => "🍜",
    "noodles" => "🍝",
    "pasta" => "🍝",
    "bread" => "🍞",
    "croissant" => "🥐",
    "bagel" => "🥯",
    "pancakes" => "🥞",
    "bacon" => "🥓",
    "egg" => "🥚",
    "cheese" => "🧀",
    "meat" => "🍖",
    "steak" => "🥩",
    "chicken" => "🍗",
    "salad" => "🥗",
    "popcorn" => "🍿",
    "cake" => "🍰",
    "birthday" => "🎂",
    "cupcake" => "🧁",
    "pie" => "🥧",
    "chocolate" => "🍫",
    "candy" => "🍬",
    "lollipop" => "🍭",
    "doughnut" => "🍩",
    "donut" => "🍩",
    "cookie" => "🍪",
    "icecream" => "🍦",
    "ice" => "🍦",
    "cream" => "🍦",
    "apple" => "🍎",
    "orange" => "🍊",
    "lemon" => "🍋",
    "banana" => "🍌",
    "watermelon" => "🍉",
    "grapes" => "🍇",
    "strawberry" => "🍓",
    "cherry" => "🍒",
    "peach" => "🍑",
    "pineapple" => "🍍",
    "coconut" => "🥥",
    "avocado" => "🥑",
    "tomato" => "🍅",
    "eggplant" => "🍆",
    "potato" => "🥔",
    "carrot" => "🥕",
    "corn" => "🌽",
    "pepper" => "🌶️",
    "cucumber" => "🥒",
    "broccoli" => "🥦",
    "coffee" => "☕",
    "tea" => "🍵",
    "beer" => "🍺",
    "beers" => "🍻",
    "wine" => "🍷",
    "champagne" => "🍾",
    "cocktail" => "🍹",
    "drink" => "🥤",
    "juice" => "🧃",
    "milk" => "🥛",
    "water" => "💧",

    // Activities & Sports
    "soccer" => "⚽",
    "football" => "🏈",
    "basketball" => "🏀",
    "baseball" => "⚾",
    "tennis" => "🎾",
    "volleyball" => "🏐",
    "rugby" => "🏉",
    "golf" => "⛳",
    "cricket" => "🏏",
    "hockey" => "🏒",
    "ski" => "⛷️",
    "snowboard" => "🏂",
    "surf" => "🏄",
    "swim" => "🏊",
    "bike" => "🚴",
    "run" => "🏃",
    "running" => "🏃",
    "walk" => "🚶",
    "gym" => "🏋️",
    "lift" => "🏋️",
    "yoga" => "🧘",
    "dance" => "💃",
    "music" => "🎵",
    "note" => "🎶",
    "guitar" => "🎸",
    "piano" => "🎹",
    "microphone" => "🎤",
    "mic" => "🎤",
    "headphones" => "🎧",
    "game" => "🎮",
    "gaming" => "🎮",
    "dice" => "🎲",
    "chess" => "♟️",
    "art" => "🎨",
    "paint" => "🎨",
    "movie" => "🎬",
    "film" => "🎬",
    "camera" => "📷",
    "photo" => "📸",

    // Travel & Places
    "plane" => "✈️",
    "airplane" => "✈️",
    "flight" => "✈️",
    "car" => "🚗",
    "taxi" => "🚕",
    "bus" => "🚌",
    "train" => "🚆",
    "metro" => "🚇",
    "subway" => "🚇",
    "bicycle" => "🚲",
    "scooter" => "🛴",
    "motorcycle" => "🏍️",
    "truck" => "🚚",
    "ship" => "🚢",
    "boat" => "⛵",
    "rocket" => "🚀",
    "satellite" => "🛰️",
    "helicopter" => "🚁",
    "anchor" => "⚓",
    "construction" => "🚧",
    "fuel" => "⛽",
    "hotel" => "🏨",
    "house" => "🏠",
    "home" => "🏡",
    "building" => "🏢",
    "office" => "🏢",
    "hospital" => "🏥",
    "school" => "🏫",
    "bank" => "🏦",
    "church" => "⛪",
    "castle" => "🏰",
    "tent" => "⛺",
    "camping" => "🏕️",
    "mountain" => "⛰️",
    "beach" => "🏖️",
    "desert" => "🏜️",
    "island" => "🏝️",
    "park" => "🏞️",
    "stadium" => "🏟️",
    "tokyo" => "🗼",
    "statue" => "🗽",
    "liberty" => "🗽",
    "bridge" => "🌉",

    // Objects
    "phone" => "📱",
    "mobile" => "📱",
    "iphone" => "📱",
    "computer" => "💻",
    "laptop" => "💻",
    "mac" => "💻",
    "keyboard" => "⌨️",
    "computermouse" => "🖱️",
    "printer" => "🖨️",
    "watch" => "⌚",
    "clock" => "🕐",
    "time" => "⏰",
    "alarm" => "⏰",
    "calendar" => "📅",
    "date" => "📆",
    "book" => "📖",
    "books" => "📚",
    "notebook" => "📓",
    "pen" => "🖊️",
    "pencil" => "✏️",
    "crayon" => "🖍️",
    "briefcase" => "💼",
    "folder" => "📁",
    "file" => "📄",
    "clipboard" => "📋",
    "pushpin" => "📌",
    "pin" => "📍",
    "paperclip" => "📎",
    "link" => "🔗",
    "chain" => "⛓️",
    "scissors" => "✂️",
    "lock" => "🔒",
    "unlock" => "🔓",
    "key" => "🔑",
    "hammer" => "🔨",
    "tool" => "🔧",
    "wrench" => "🔧",
    "nut" => "🔩",
    "gear" => "⚙️",
    "settings" => "⚙️",
    "magnet" => "🧲",
    "bomb" => "💣",
    "gun" => "🔫",
    "knife" => "🔪",
    "shield" => "🛡️",
    "cigarette" => "🚬",
    "smoke" => "💨",
    "pill" => "💊",
    "syringe" => "💉",
    "bandage" => "🩹",
    "stethoscope" => "🩺",
    "door" => "🚪",
    "bed" => "🛏️",
    "couch" => "🛋️",
    "toilet" => "🚽",
    "shower" => "🚿",
    "bath" => "🛁",
    "soap" => "🧼",
    "toothbrush" => "🪥",
    "towel" => "🧻",
    "basket" => "🧺",
    "broom" => "🧹",
    "clean" => "🧹",

    // Symbols
    "check" => "✅",
    "checkmark" => "✓",
    "x" => "❌",
    "cross" => "❌",
    "no" => "🚫",
    "stop" => "🛑",
    "warning" => "⚠️",
    "caution" => "⚠️",
    "question" => "❓",
    "info" => "ℹ️",
    "exclamation" => "❗",
    "bang" => "❗",
    "plus" => "➕",
    "minus" => "➖",
    "multiply" => "✖️",
    "divide" => "➗",
    "dollar" => "💲",
    "money" => "💰",
    "cash" => "💵",
    "credit" => "💳",
    "yen" => "💴",
    "euro" => "💶",
    "pound" => "💷",
    "chart" => "📈",
    "up" => "⬆️",
    "down" => "⬇️",
    "left" => "⬅️",
    "right" => "➡️",
    "arrow" => "➡️",
    "arrows" => "🔄",
    "refresh" => "🔄",
    "reload" => "🔄",
    "repeat" => "🔁",
    "loop" => "🔁",
    "shuffle" => "🔀",
    "new" => "🆕",
    "free" => "🆓",
    "sos" => "🆘",
    "id" => "🆔",
    "atm" => "🏧",
    "wc" => "🚾",
    "parking" => "🅿️",
    "wheelchair" => "♿",
    "recycle" => "♻️",
    "wifi" => "📶",
    "signal" => "📶",
    "battery" => "🔋",
    "power" => "🔌",
    "search" => "🔍",
    "find" => "🔍",
    "zoom" => "🔎",
    "bell" => "🔔",
    "notification" => "🔔",
    "mute" => "🔇",
    "sound" => "🔊",
    "volume" => "🔊",
    "speaker" => "🔊",
    "light" => "💡",
    "bulb" => "💡",
    "idea" => "💡",
    "sun" => "☀️",
    "sunny" => "☀️",
    "moon" => "🌙",
    "cloud" => "☁️",
    "rain" => "🌧️",
    "rainy" => "☔",
    "umbrella" => "☂️",
    "snow" => "❄️",
    "snowing" => "🌨️",
    "wind" => "💨",
    "tornado" => "🌪️",
    "fog" => "🌫️",
    "rainbow" => "🌈",
    "lightning" => "⚡",
    "bolt" => "⚡",
    "zap" => "⚡",
    "comet" => "☄️",
    "earth" => "🌍",
    "globe" => "🌎",
    "world" => "🌏",
    "map" => "🗺️",
    "compass" => "🧭",

    // Flags (popular ones)
    "flag" => "🏳️",
    "usa" => "🇺🇸",
    "us" => "🇺🇸",
    "america" => "🇺🇸",
    "uk" => "🇬🇧",
    "britain" => "🇬🇧",
    "canada" => "🇨🇦",
    "france" => "🇫🇷",
    "germany" => "🇩🇪",
    "japan" => "🇯🇵",
    "china" => "🇨🇳",
    "india" => "🇮🇳",
    "brazil" => "🇧🇷",
    "russia" => "🇷🇺",
    "australia" => "🇦🇺",
    "spain" => "🇪🇸",
    "italy" => "🇮🇹",
    "mexico" => "🇲🇽",
    "korea" => "🇰🇷",

    // Misc
    "rainbowflag" => "🏳️‍🌈",
    "pride" => "🏳️‍🌈",
    "pirate" => "🏴‍☠️",
    "checkered" => "🏁",
    "finish" => "🏁",
    "white" => "⚪",
    "black" => "⚫",
    "red" => "🔴",
    "blue" => "🔵",
    "yellow" => "🟡",
    "green" => "🟢",
    "purple" => "🟣",
    "brown" => "🟤",
    "orangecircle" => "🟠",
    "circle" => "⭕",
    "square" => "🟥",
    "diamond" => "💎",
    "gem" => "💎",
    "crown" => "👑",
    "king" => "👑",
    "queen" => "👑",
    "trophy" => "🏆",
    "medal" => "🏅",
    "winner" => "🥇",
    "gold" => "🥇",
    "silver" => "🥈",
    "bronze" => "🥉",
    "gift" => "🎁",
    "present" => "🎁",
    "balloon" => "🎈",
    "party" => "🎉",
    "celebrate" => "🎊",
    "confetti" => "🎊",
    "tada" => "🎉",
    "christmas" => "🎄",
    "christmastree" => "🎄",
    "santa" => "🎅",
    "snowman" => "⛄",
    "halloween" => "🎃",
    "pumpkin" => "🎃",
};

/// Search for emojis matching the query
fn search_emojis(query: &str, limit: usize) -> Vec<(&'static str, &'static str)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    // First, try exact match and prefix matches
    for (keyword, emoji) in EMOJI_MAP.entries() {
        if results.len() >= limit {
            break;
        }

        // Exact match (highest priority)
        if *keyword == query_lower.as_str() {
            results.push((*keyword, *emoji));
        }
    }

    // Then, try prefix matches
    for (keyword, emoji) in EMOJI_MAP.entries() {
        if results.len() >= limit {
            break;
        }

        // Skip if we already added this keyword
        if results.iter().any(|(k, _)| k == keyword) {
            continue;
        }

        // Prefix match
        if keyword.starts_with(&query_lower) {
            results.push((*keyword, *emoji));
        }
    }

    // If we still need more results, do substring matching
    if results.len() < limit {
        for (keyword, emoji) in EMOJI_MAP.entries() {
            if results.len() >= limit {
                break;
            }

            // Skip if we already added this keyword
            if results.iter().any(|(k, _)| k == keyword) {
                continue;
            }

            // Substring match
            if keyword.contains(&query_lower) {
                results.push((*keyword, *emoji));
            }
        }
    }

    results
}

fn main() {
    let args = Args::parse();

    let results = search_emojis(&args.query, args.limit);

    if results.is_empty() {
        eprintln!("No emojis found for: {}", args.query);
        std::process::exit(1);
    }

    for (keyword, emoji) in results {
        println!("{} ({})", emoji, keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let results = search_emojis("heart", 10);
        assert!(!results.is_empty());
        assert!(results.iter().any(|(_, e)| *e == "❤️"));
    }

    #[test]
    fn test_prefix_match() {
        let results = search_emojis("fir", 10);
        assert!(!results.is_empty());
        assert!(results.iter().any(|(k, _)| k.starts_with("fir")));
    }

    #[test]
    fn test_substring_match() {
        let results = search_emojis("app", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_limit() {
        let results = search_emojis("a", 5);
        assert!(results.len() <= 5);
    }
}
