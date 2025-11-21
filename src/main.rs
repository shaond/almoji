use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;

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

    /// Format output as Alfred-compatible JSON
    #[arg(long)]
    alfred: bool,
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

#[derive(Serialize, Debug)]
struct AlfredItem {
    uid: String,
    title: String,
    subtitle: String,
    arg: String,
    text: AlfredItemText,
    valid: bool,
}

#[derive(Serialize, Debug)]
struct AlfredItemText {
    copy: String,
    largetype: String,
}

#[derive(Serialize, Debug)]
struct AlfredResponse {
    items: Vec<AlfredItem>,
}

/// Custom slang and cultural reference mappings
/// Maps common slang terms and cultural references to their corresponding emojis
static SLANG_MAP: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Cannabis/Marijuana slang
    map.insert("weed", vec!["🥦", "🌿", "🍃"]);
    map.insert("marijuana", vec!["🥦", "🌿", "🍃"]);
    map.insert("cannabis", vec!["🥦", "🌿", "🍃"]);
    map.insert("pot", vec!["🥦", "🌿", "🍁"]);
    map.insert("herb", vec!["🌿", "🥦"]);
    map.insert("420", vec!["🥦", "🌿", "🍃"]);
    map.insert("joint", vec!["🌿", "🥦"]);
    map.insert("bud", vec!["🌿"]);
    map.insert("hash", vec!["🍀"]);
    map.insert("chronic", vec!["🌲"]);
    map.insert("blunt", vec!["🌴"]);
    map.insert("nug", vec!["🥦"]);

    // Adult/Sexual slang
    map.insert("penis", vec!["🍆"]);
    map.insert("dick", vec!["🍆", "🍌"]);
    map.insert("cock", vec!["🍆"]);
    map.insert("dong", vec!["🌭"]);
    map.insert("schlong", vec!["🍆"]);
    map.insert("pecker", vec!["🍆"]);
    map.insert("wang", vec!["🍌"]);
    map.insert("butt", vec!["🍑"]);
    map.insert("ass", vec!["🍑"]);
    map.insert("booty", vec!["🍑"]);
    map.insert("arse", vec!["🍑"]);
    map.insert("bum", vec!["🍑"]);
    map.insert("pussy", vec!["🌮"]);
    map.insert("cunt", vec!["🌮", "💅", "👑", "💅👑"]);
    map.insert("vagina", vec!["🌮"]);
    map.insert("coochie", vec!["🌮"]);
    map.insert("punani", vec!["🌮"]);
    map.insert("sexy", vec!["🍆", "🍑", "💦"]);
    map.insert("sex", vec!["🍆", "🍑"]);
    map.insert("horny", vec!["🍆", "💦", "👅"]);
    map.insert("ejaculate", vec!["💦"]);
    map.insert("cum", vec!["💦"]);
    map.insert("jizz", vec!["💦"]);
    map.insert("spunk", vec!["💦"]);
    map.insert("wet", vec!["💦"]);
    map.insert("moist", vec!["💦"]);
    map.insert("breasts", vec!["🍈", "🍉"]);
    map.insert("boobs", vec!["🍈", "🍉"]);
    map.insert("tits", vec!["🍈", "🍒"]);
    map.insert("boobies", vec!["🍈"]);
    map.insert("titties", vec!["🍈", "🍒"]);
    map.insert("jugs", vec!["🍈"]);
    map.insert("knockers", vec!["🍈"]);
    map.insert("rack", vec!["🍈", "🍉"]);
    map.insert("spicy", vec!["🌶️"]);
    map.insert("slut", vec!["💦", "🍑", "👅"]);
    map.insert("whore", vec!["💦", "👅"]);
    map.insert("hoe", vec!["💦"]);
    map.insert("thot", vec!["💦", "🍑"]);
    map.insert("bitch", vec!["🐕", "💁"]);
    map.insert("milf", vec!["🍑", "👩"]);
    map.insert("dilf", vec!["🍆", "👨"]);
    map.insert("daddy", vec!["👨", "🍆"]);
    map.insert("mommy", vec!["👩", "🍑"]);
    map.insert("kinky", vec!["😈", "🔗"]);
    map.insert("bdsm", vec!["⛓️", "😈"]);
    map.insert("bondage", vec!["⛓️"]);
    map.insert("dom", vec!["😈", "👑"]);
    map.insert("sub", vec!["😇", "⛓️"]);
    map.insert("kink", vec!["😈"]);
    map.insert("fetish", vec!["👠", "😈"]);
    map.insert("oral", vec!["👅", "🍆", "👅🍆", "💋🍆"]);
    map.insert("blowjob", vec!["👅", "🍆", "👅🍆", "💋🍆"]);
    map.insert("bj", vec!["👅", "🍆", "👅🍆", "💋🍆"]);
    map.insert("deepthroat", vec!["🍆", "👅", "🍆👅"]);
    map.insert("anal", vec!["🍑", "🍆", "🍑🍆"]);
    map.insert("pegging", vec!["🍑", "🍆", "🍑🍆"]);
    map.insert("69", vec!["👅", "💦", "👅💦"]);
    map.insert("sixtynine", vec!["👅", "💦", "👅💦"]);
    map.insert("facial", vec!["💦", "😮", "💦😮"]);
    map.insert("creampie", vec!["💦", "🥧", "💦🥧"]);
    map.insert("squirt", vec!["💦"]);
    map.insert("orgasm", vec!["💦", "😩", "💦😩"]);
    map.insert("climax", vec!["💦", "😫", "💦😫"]);
    map.insert("masturbate", vec!["✊", "💦", "✊💦"]);
    map.insert("jerkoff", vec!["✊", "💦", "✊💦"]);
    map.insert("wank", vec!["✊", "💦", "✊💦"]);
    map.insert("fap", vec!["✊", "💦", "✊💦"]);
    map.insert("fingering", vec!["👆", "💦", "👆💦"]);
    map.insert("handjob", vec!["✊🍆"]);
    map.insert("clitoris", vec!["💎"]);
    map.insert("clit", vec!["💎"]);
    map.insert("balls", vec!["🏀", "⚽"]);
    map.insert("testicles", vec!["🥜"]);
    map.insert("nuts", vec!["🥜"]);
    map.insert("scrotum", vec!["🥜"]);
    map.insert("erection", vec!["🍆", "⬆️"]);
    map.insert("boner", vec!["🍆", "⬆️"]);
    map.insert("hardon", vec!["🍆"]);
    map.insert("stiffy", vec!["🍆"]);
    map.insert("chub", vec!["🍆"]);
    map.insert("nude", vec!["👙", "🔞"]);
    map.insert("naked", vec!["👙", "🔞"]);
    map.insert("nudes", vec!["📸", "🔞"]);
    map.insert("strip", vec!["👙"]);
    map.insert("stripper", vec!["💃", "💵"]);
    map.insert("pornstar", vec!["⭐", "🔞"]);
    map.insert("porn", vec!["🔞", "📹"]);
    map.insert("xxx", vec!["🔞"]);
    map.insert("r18", vec!["🔞"]);
    map.insert("adult", vec!["🔞"]);

    // Other drug slang
    map.insert("coke", vec!["❄️"]);
    map.insert("powder", vec!["⛄"]);
    map.insert("eight", vec!["🎱"]);
    map.insert("molly", vec!["💊"]);
    map.insert("redpill", vec!["💊"]);
    map.insert("shroom", vec!["🍄"]);
    map.insert("plug", vec!["🔌"]);
    map.insert("high", vec!["🚀"]);
    map.insert("fent", vec!["🧀"]);
    map.insert("h", vec!["🐉"]);

    // LGBTQIA+ and Pride
    map.insert("gay", vec!["🏳️‍🌈"]);
    map.insert("pride", vec!["🏳️‍🌈"]);
    map.insert("lgbtq", vec!["🏳️‍🌈"]);
    map.insert("lgbtqia", vec!["🏳️‍🌈"]);
    map.insert("queer", vec!["🏳️‍🌈"]);
    map.insert("rainbow", vec!["🏳️‍🌈", "🌈"]);
    map.insert("trans", vec!["🏳️‍⚧️"]);
    map.insert("transgender", vec!["🏳️‍⚧️"]);
    map.insert("lesbian", vec!["👩‍❤️‍👩", "🏳️‍🌈"]);
    map.insert("bi", vec!["🏳️‍🌈", "💗💜💙"]);
    map.insert("bisexual", vec!["🏳️‍🌈", "💗💜💙"]);
    map.insert("ace", vec!["🖤🩶🤍💜"]);
    map.insert("asexual", vec!["🖤🩶🤍💜"]);
    map.insert("nonbinary", vec!["🏳️‍⚧️"]);
    map.insert("enby", vec!["🏳️‍⚧️"]);
    map.insert("pan", vec!["🏳️‍🌈", "💗💛💙"]);
    map.insert("pansexual", vec!["🏳️‍🌈", "💗💛💙"]);
    map.insert("genderfluid", vec!["🏳️‍⚧️"]);
    map.insert("genderqueer", vec!["🏳️‍⚧️"]);

    // Gen Z slang
    map.insert("peace", vec!["✌️", "☮️"]);
    map.insert("dead", vec!["💀"]);
    map.insert("dying", vec!["💀"]);
    map.insert("skull", vec!["💀"]);
    map.insert("fire", vec!["🔥"]);
    map.insert("lit", vec!["🔥"]);
    map.insert("dope", vec!["🔥"]);
    map.insert("cap", vec!["🧢"]);
    map.insert("lying", vec!["🧢", "🤥"]);
    map.insert("fake", vec!["🧢", "🐍"]);
    map.insert("nocap", vec!["❌🧢", "🚫🧢"]);
    map.insert("clown", vec!["🤡"]);
    map.insert("clowning", vec!["🤡"]);
    map.insert("foolish", vec!["🤡"]);
    map.insert("stupid", vec!["🤡"]);
    map.insert("cringe", vec!["😬"]);
    map.insert("oof", vec!["😬", "😅"]);
    map.insert("yeet", vec!["💨", "🚀"]);
    map.insert("vibes", vec!["✨", "🌊"]);
    map.insert("vibe", vec!["💚", "✨"]);
    map.insert("slay", vec!["💅", "👑", "😭", "🎯", "💅👑", "💅🏼👑"]);
    map.insert("sus", vec!["🤨", "🔍", "💩"]);
    map.insert("suspicious", vec!["🤨", "🔍"]);
    map.insert("shook", vec!["😱", "🤯"]);
    map.insert("mood", vec!["💯"]);
    map.insert("bet", vec!["💯"]);
    map.insert("flex", vec!["💪", "💎", "🏆", "💪💎"]);
    map.insert("drip", vec!["💧", "💎", "💧💎"]);
    map.insert("ghosted", vec!["👻"]);
    map.insert("ghost", vec!["👻"]);
    map.insert("shade", vec!["☕", "😎"]);
    map.insert("tea", vec!["☕", "🫖"]);
    map.insert("gossip", vec!["☕", "👀", "☕👀"]);
    map.insert("lurking", vec!["👀"]);
    map.insert("salty", vec!["🧂", "🧚‍♀️✨"]);
    map.insert("savage", vec!["😈", "🔥"]);
    map.insert("lowkey", vec!["🤫"]);
    map.insert("highkey", vec!["📢"]);
    map.insert("fam", vec!["👪", "🤝", "💙"]);
    map.insert("bruh", vec!["🤦", "😑"]);
    map.insert("bro", vec!["🤜🤛", "👊"]);
    map.insert("bros", vec!["😎"]);
    map.insert("bestie", vec!["👯", "💕", "🧡", "😊"]);
    map.insert("simp", vec!["🤡", "💕"]);
    map.insert("stan", vec!["🙌", "❤️"]);
    map.insert("stans", vec!["💳"]);
    map.insert("blessed", vec!["🙏", "✨", "🧿"]);
    map.insert("canceled", vec!["🚫", "❌"]);
    map.insert("ew", vec!["👍"]);
    map.insert("bae", vec!["❤️"]);
    map.insert("homie", vec!["💛"]);
    map.insert("moody", vec!["🖤"]);
    map.insert("cozy", vec!["🤎"]);
    map.insert("thicc", vec!["🍑"]);
    map.insert("freaky", vec!["😈"]);
    map.insert("wasted", vec!["🥴"]);
    map.insert("toxic", vec!["🚩"]);
    map.insert("shy", vec!["👉👈"]);
    map.insert("goat", vec!["🐐"]);
    map.insert("gem", vec!["💎"]);
    map.insert("nerd", vec!["🤓"]);
    map.insert("stonks", vec!["📈"]);
    map.insert("shred", vec!["💪"]);
    map.insert("drama", vec!["🍿"]);
    map.insert("feds", vec!["🍩"]);
    map.insert("nsfw", vec!["🌽"]);
    map.insert("expire", vec!["⌛"]);
    map.insert("boom", vec!["💥"]);
    map.insert("incel", vec!["🫘"]);
    map.insert("sarcasm", vec!["🙃"]);
    map.insert("uwu", vec!["🥺"]);
    map.insert("zoned", vec!["😶‍🌫️"]);
    map.insert("word", vec!["🤝"]);
    map.insert("girly", vec!["🎀"]);
    map.insert("boo", vec!["🍅"]);
    map.insert("hustle", vec!["😤"]);
    map.insert("idc", vec!["🤷"]);
    map.insert("bussin", vec!["😋", "🔥", "😋🔥"]);
    map.insert("rizz", vec!["😏", "💫", "😏💫"]);
    map.insert("mid", vec!["👎", "😐"]);
    map.insert("w", vec!["🏆", "💪"]);
    map.insert("l", vec!["📉", "😔"]);
    map.insert("ratio", vec!["📊", "💀", "📊💀"]);
    map.insert("rentfree", vec!["🧠", "💭", "🧠💭"]);
    map.insert("hitsdifferent", vec!["💫", "✨", "💫✨"]);
    map.insert("based", vec!["💯", "🗿", "💯🗿"]);
    map.insert("touchgrass", vec!["🌱", "☀️", "🌱☀️"]);
    map.insert("grasstouch", vec!["🌱", "☀️"]);
    map.insert("gaslight", vec!["🔥💡", "🤥"]);
    map.insert("gatekeep", vec!["🚪🔒", "🚪"]);
    map.insert("girlboss", vec!["💅👩‍💼", "💅", "👩‍💼"]);
    map.insert("ick", vec!["🤢", "😬"]);
    map.insert("maincharacter", vec!["🌟🎬", "🌟"]);
    map.insert("npc", vec!["🤖", "😐", "🤖😐"]);
    map.insert("delulu", vec!["☁️", "💭", "☁️💭"]);
    map.insert("delusional", vec!["☁️", "💭"]);
    map.insert("fr", vec!["💯"]);
    map.insert("forreal", vec!["💯"]);
    map.insert("ong", vec!["🙏", "💯", "🙏💯"]);
    map.insert("periodt", vec!["💅", "💯", "💅💯"]);
    map.insert("ate", vec!["😋", "💯"]);
    map.insert("devoured", vec!["😋", "💯", "🔥", "😋💯🔥"]);
    map.insert("serve", vec!["💅", "✨", "💅✨"]);
    map.insert("serving", vec!["💅", "✨"]);
    map.insert("icon", vec!["⭐", "👑"]);
    map.insert("legend", vec!["👑", "⚡", "👑⚡"]);
    map.insert("feral", vec!["🐺", "😤"]);
    map.insert("unhinged", vec!["😵‍💫", "🔓", "😵‍💫🔓"]);
    map.insert("intrusivethoughts", vec!["😈", "💭", "😈💭"]);
    map.insert("valid", vec!["✅", "💯"]);
    map.insert("slaps", vec!["🔥", "👋", "🔥👋"]);
    map.insert("bops", vec!["🎵", "💃", "🎵💃"]);
    map.insert("aesthetic", vec!["✨", "🎨", "✨🎨"]);
    map.insert("vibecheck", vec!["✅", "✨", "💚", "✅✨"]);
    map.insert("energy", vec!["⚡", "✨"]);
    map.insert("aura", vec!["🌟", "😎", "🌟😎"]);
    map.insert("caughtin4k", vec!["📸", "😳", "📸😳"]);
    map.insert("downbad", vec!["😩", "📉", "😩📉"]);
    map.insert("copium", vec!["💊", "😮‍💨", "💊😮‍💨"]);
    map.insert("hopium", vec!["💊", "🌈", "💊🌈"]);
    map.insert("malding", vec!["😡", "🧑‍🦲", "😡🧑‍🦲"]);
    map.insert("seething", vec!["😤", "💢"]);
    map.insert("mald", vec!["😡"]);
    map.insert("sheesh", vec!["🥶", "🔥"]);
    map.insert("ayo", vec!["🤨", "📸"]);
    map.insert("pausebutton", vec!["⏸️", "🤨"]);
    map.insert("pause", vec!["⏸️", "🤨"]);
    map.insert("holdup", vec!["✋", "🤨"]);
    map.insert("caught", vec!["📸", "😳"]);
    map.insert("exposed", vec!["📸", "😳"]);
    map.insert("redflags", vec!["🚩", "🚩🚩🚩"]);
    map.insert("redflag", vec!["🚩"]);
    map.insert("greenflags", vec!["🟢", "✅"]);
    map.insert("greenflag", vec!["🟢", "✅"]);
    map.insert("beige", vec!["🟤", "😐"]);
    map.insert("vanilla", vec!["🍦"]);
    map.insert("basic", vec!["☕", "😐"]);
    map.insert("pickme", vec!["🤡", "💁"]);
    map.insert("girlsgirl", vec!["👯‍♀️", "💖"]);
    map.insert("boysmom", vec!["👩", "👦"]);

    // Country/Location slang
    map.insert("aussie", vec!["🇦🇺"]);
    map.insert("oz", vec!["🇦🇺"]);
    map.insert("straya", vec!["🇦🇺"]);
    map.insert("usa", vec!["🇺🇸"]);
    map.insert("america", vec!["🇺🇸"]);
    map.insert("murica", vec!["🇺🇸"]);
    map.insert("yank", vec!["🇺🇸"]);
    map.insert("yankee", vec!["🇺🇸"]);
    map.insert("uk", vec!["🇬🇧"]);
    map.insert("britain", vec!["🇬🇧"]);
    map.insert("brit", vec!["🇬🇧"]);
    map.insert("england", vec!["🇬🇧", "🏴󠁧󠁢󠁥󠁮󠁧󠁿"]);
    map.insert("canuck", vec!["🇨🇦"]);
    map.insert("canada", vec!["🇨🇦"]);
    map.insert("french", vec!["🇫🇷"]);
    map.insert("france", vec!["🇫🇷"]);
    map.insert("deutschland", vec!["🇩🇪"]);
    map.insert("germany", vec!["🇩🇪"]);
    map.insert("japan", vec!["🇯🇵"]);
    map.insert("nippon", vec!["🇯🇵"]);
    map.insert("china", vec!["🇨🇳"]);
    map.insert("korea", vec!["🇰🇷"]);
    map.insert("india", vec!["🇮🇳"]);
    map.insert("mexico", vec!["🇲🇽"]);
    map.insert("brazil", vec!["🇧🇷"]);
    map.insert("russia", vec!["🇷🇺"]);
    map.insert("italy", vec!["🇮🇹"]);
    map.insert("spain", vec!["🇪🇸"]);

    // British English slang
    map.insert("innit", vec!["🇬🇧"]);
    map.insert("pint", vec!["🍺"]);
    map.insert("football", vec!["⚽"]);
    map.insert("ace", vec!["👌"]);
    map.insert("bollocks", vec!["🤦"]);
    map.insert("guarding", vec!["💂"]);
    map.insert("mate", vec!["🤝", "👥"]);
    map.insert("mates", vec!["👥", "🤝"]);
    map.insert("cheers", vec!["🍻", "🥂"]);
    map.insert("bloke", vec!["👨", "🧔"]);
    map.insert("cheeky", vec!["😏", "😜"]);
    map.insert("knackered", vec!["😴", "😩"]);
    map.insert("gutted", vec!["😞", "😔"]);
    map.insert("brilliant", vec!["✨", "🌟"]);
    map.insert("lovely", vec!["🥰", "💕"]);

    // Australian English slang
    map.insert("roo", vec!["🦘"]);
    map.insert("arvo", vec!["🏖️"]);
    map.insert("ripper", vec!["😎"]);
    map.insert("outback", vec!["🦎"]);
    map.insert("huntsman", vec!["🕷️"]);
    map.insert("goon", vec!["🍷"]);
    map.insert("croc", vec!["🐊"]);
    map.insert("tropical", vec!["🌴"]);

    // Canadian English slang
    map.insert("eh", vec!["🇨🇦", "🤔"]);
    map.insert("toque", vec!["🧢", "🇨🇦"]);
    map.insert("loonie", vec!["🪙", "🇨🇦"]);
    map.insert("double-double", vec!["☕", "🇨🇦"]);
    map.insert("doubledouble", vec!["☕"]);

    // American English slang
    map.insert("dude", vec!["🤙", "😎"]);
    map.insert("awesome", vec!["🤩", "🔥"]);
    map.insert("trash", vec!["🗑️", "💩"]);
    map.insert("garbage", vec!["🗑️", "🚮"]);
    map.insert("jerk", vec!["😠", "🤬"]);

    // Singlish slang
    map.insert("lah", vec!["🇸🇬"]);
    map.insert("makan", vec!["🍜"]);
    map.insert("sotong", vec!["🦑"]);
    map.insert("paiseh", vec!["😅"]);
    map.insert("shiok", vec!["💪", "😋"]);
    map.insert("alamak", vec!["🤔"]);
    map.insert("wahlau", vec!["🙄"]);

    // Indian English slang
    map.insert("desi", vec!["🇮🇳"]);
    map.insert("namaste", vec!["🙏"]);
    map.insert("curry", vec!["🍛"]);
    map.insert("om", vec!["🕉️"]);
    map.insert("puja", vec!["💐"]);
    map.insert("diwali", vec!["🪔", "🎆"]);
    map.insert("cricket", vec!["🏏"]);
    map.insert("shabash", vec!["🙌"]);
    map
});

/// Search for emojis matching the query using the comprehensive Unicode emoji database
fn search_emojis(query: &str, limit: usize) -> Vec<(String, &'static emojis::Emoji)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut seen: HashMap<String, bool> = HashMap::new();

    // Check custom slang mappings first - exact match
    if let Some(slang_emojis) = SLANG_MAP.get(query_lower.as_str()) {
        for emoji_str in slang_emojis {
            if results.len() >= limit {
                break;
            }

            let key = emoji_str.to_string();
            if seen.contains_key(&key) {
                continue;
            }

            // Try to find the emoji in the database
            if let Some(emoji_obj) = emojis::get(emoji_str) {
                results.push((query_lower.clone(), emoji_obj));
                seen.insert(key, true);
            } else {
                // For compound emojis not in the database, try to find by iterating
                let mut found = false;
                for emoji in emojis::iter() {
                    if emoji.as_str() == *emoji_str {
                        results.push((query_lower.clone(), emoji));
                        seen.insert(key.clone(), true);
                        found = true;
                        break;
                    }
                }

                // If not found in database, it might be a group of emojis (e.g., "💗💜💙")
                // In this case, we'll treat it as a raw emoji string
                if !found {
                    // Use a dummy emoji object, but store the actual emoji string in the result
                    // We'll use a marker in the keyword to indicate this is a raw emoji string
                    if let Some(dummy_emoji) = emojis::iter().next() {
                        results.push((format!("__raw__:{}", emoji_str), dummy_emoji));
                        seen.insert(key, true);
                    }
                }
            }
        }

        // If we found slang matches and hit the limit, return early
        if results.len() >= limit {
            return results;
        }
    }

    // Check custom slang mappings - partial match (substring)
    if results.len() < limit {
        for (slang_term, slang_emojis) in SLANG_MAP.iter() {
            // Skip exact matches (already handled above)
            if *slang_term == query_lower.as_str() {
                continue;
            }

            // Check if the query is a substring of the slang term
            if slang_term.contains(&query_lower) {
                for emoji_str in slang_emojis {
                    if results.len() >= limit {
                        break;
                    }

                    let key = emoji_str.to_string();
                    if seen.contains_key(&key) {
                        continue;
                    }

                    // Try to find the emoji in the database
                    if let Some(emoji_obj) = emojis::get(emoji_str) {
                        results.push((slang_term.to_string(), emoji_obj));
                        seen.insert(key, true);
                    } else {
                        // For compound emojis not in the database, try to find by iterating
                        let mut found = false;
                        for emoji in emojis::iter() {
                            if emoji.as_str() == *emoji_str {
                                results.push((slang_term.to_string(), emoji));
                                seen.insert(key.clone(), true);
                                found = true;
                                break;
                            }
                        }

                        // If not found in database, it might be a group of emojis (e.g., "💗💜💙")
                        // In this case, we'll treat it as a raw emoji string
                        if !found {
                            // Use a dummy emoji object, but store the actual emoji string in the result
                            // We'll use a marker in the keyword to indicate this is a raw emoji string
                            if let Some(dummy_emoji) = emojis::iter().next() {
                                results.push((format!("__raw__:{}", emoji_str), dummy_emoji));
                                seen.insert(key, true);
                            }
                        }
                    }
                }
            }

            if results.len() >= limit {
                break;
            }
        }
    }

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
            if name_normalized
                .split_whitespace()
                .any(|word| word.starts_with(&query_lower))
            {
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

        // For compound emojis with ZWJ sequences, we need to insert the skin tone
        // modifier after the first emoji character, not at the end
        let chars: Vec<char> = base.chars().collect();
        if chars.len() > 1 {
            // Find the first emoji character and insert skin tone after it
            let first_char = chars[0];
            let rest: String = chars[1..].iter().collect();
            return format!("{}{}{}", first_char, modifier, rest);
        } else {
            // Simple emoji, just append
            return format!("{}{}", base, modifier);
        }
    }

    base.to_string()
}

/// Apply gender modifier to emoji if applicable
/// Note: Gender variants are already in the emoji database (e.g., "man firefighter", "woman firefighter")
/// so we don't apply gender modifiers programmatically as they can render incorrectly
fn apply_gender(emoji_str: &str, _gender: &Gender) -> String {
    // Return emoji as-is since gendered variants are already in the database
    emoji_str.to_string()
}

/// List all emojis including slang and substitutions
fn list_all_emojis() {
    println!("=== SLANG AND SUBSTITUTIONS ===\n");

    // Collect and sort slang terms
    let mut slang_terms: Vec<(&str, &Vec<&str>)> = SLANG_MAP.iter().map(|(k, v)| (*k, v)).collect();
    slang_terms.sort_by_key(|(term, _)| *term);

    for (slang_term, emojis) in slang_terms {
        let emoji_str: Vec<String> = emojis.iter().map(|e| e.to_string()).collect();
        println!("{}: {}", slang_term, emoji_str.join(" "));
    }

    println!("\n=== ALL UNICODE EMOJIS ===\n");

    for emoji in emojis::iter() {
        let shortcodes: Vec<String> = emoji.shortcodes().map(|s| s.to_string()).collect();

        if shortcodes.is_empty() {
            println!("{} - {}", emoji.as_str(), emoji.name());
        } else {
            println!(
                "{} - {} ({})",
                emoji.as_str(),
                emoji.name(),
                shortcodes.join(", ")
            );
        }
    }
}

fn find_emojis(
    query: &[String],
    limit: usize,
    skin_tone: &Option<SkinTone>,
    gender: &Option<Gender>,
) -> Vec<(String, String)> {
    let query_joined = query.join(" ");
    if query_joined.trim().is_empty() {
        return Vec::new();
    }

    let query_normalized = query_joined.replace(' ', "").to_lowercase();

    let mut results = search_emojis(&query_normalized, limit);

    if results.is_empty() && query_joined.contains(' ') {
        results = search_emojis(&query_joined.to_lowercase(), limit);
    }

    if results.is_empty() {
        results = search_emojis(&query_joined, limit);
    }

    results
        .into_iter()
        .map(|(keyword, emoji)| {
            let mut modified_emoji = emoji.as_str().to_string();
            if let Some(ref st) = skin_tone {
                modified_emoji = apply_skin_tone(emoji, st);
            }
            if let Some(ref g) = gender {
                modified_emoji = apply_gender(&modified_emoji, g);
            }
            (keyword, modified_emoji)
        })
        .collect()
}

fn main() {
    let args = Args::parse();

    if args.list_all {
        list_all_emojis();
        return;
    }

    let results = find_emojis(&args.query, args.limit, &args.skin_tone, &args.gender);

    if results.is_empty() {
        if args.alfred {
            let response = AlfredResponse {
                items: vec![AlfredItem {
                    uid: "no-results".to_string(),
                    title: "No emojis found".to_string(),
                    subtitle: "Try a different search term".to_string(),
                    arg: "".to_string(),
                    text: AlfredItemText {
                        copy: "".to_string(),
                        largetype: "".to_string(),
                    },
                    valid: false,
                }],
            };
            println!("{}", serde_json::to_string(&response).unwrap());
        } else {
            eprintln!("No emojis found for: {}", args.query.join(" "));
            std::process::exit(1);
        }
        return;
    }

    if args.alfred {
        let alfred_items: Vec<AlfredItem> = results
            .into_iter()
            .map(|(keyword, emoji)| AlfredItem {
                uid: keyword.clone(),
                title: emoji.clone(),
                subtitle: keyword,
                arg: emoji.clone(),
                text: AlfredItemText {
                    copy: emoji.clone(),
                    largetype: emoji,
                },
                valid: true,
            })
            .collect();

        let response = AlfredResponse { items: alfred_items };
        println!("{}", serde_json::to_string(&response).unwrap());
    } else {
        for (keyword, emoji) in results {
            println!("{} ({})", emoji, keyword);
        }
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
