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
    map.insert("sigma", vec!["🗿", "🐺"]);
    map.insert("67", vec!["🔩", "6️⃣7️⃣"]);
    map.insert("preppy", vec!["🎀", "💅", "🛍️"]);

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
    map.insert("chips", vec!["🍟"]);
    map.insert("cops", vec!["👮", "👮‍♀️", "👮‍♂️", "🚔", "🚓", "🚨"]);
    map.insert("mate", vec!["🤝"]);

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

    // Emotions and reactions
    map.insert("lol", vec!["😂", "🤣"]);
    map.insert("lmao", vec!["😂", "🤣"]);
    map.insert("lmfao", vec!["🤣", "😂"]);
    map.insert("rofl", vec!["🤣", "😂"]);
    map.insert("rotfl", vec!["🤣", "😂"]);
    map.insert("haha", vec!["😂", "🤣", "😆"]);
    map.insert("hahaha", vec!["😂", "🤣"]);
    map.insert("hehe", vec!["😆", "😄"]);
    map.insert("hehehe", vec!["😆", "😄"]);
    map.insert("hihi", vec!["😊", "😄"]);
    map.insert("hoho", vec!["🎅", "😄"]);
    map.insert("huhu", vec!["😢", "😭"]);
    map.insert("teehee", vec!["🤭", "😊"]);
    map.insert("giggle", vec!["🤭", "😄"]);
    map.insert("chuckle", vec!["😄", "😆"]);
    map.insert("snicker", vec!["😏", "😆"]);
    map.insert("guffaw", vec!["😂", "🤣"]);
    map.insert("cackle", vec!["🤣", "😈"]);
    map.insert("crying", vec!["😭", "😢"]);
    map.insert("laugh", vec!["😂", "🤣"]);
    map.insert("laughing", vec!["😂", "🤣"]);
    map.insert("smh", vec!["🤦", "😔"]);
    map.insert("facepalm", vec!["🤦"]);
    map.insert("fart", vec!["💨"]);
    map.insert("eyeroll", vec!["🙄"]);
    map.insert("shrug", vec!["🤷"]);
    map.insert("idk", vec!["🤷", "🤔"]);
    map.insert("thinking", vec!["🤔"]);
    map.insert("hmm", vec!["🤔"]);
    map.insert("wow", vec!["😮", "🤯"]);
    map.insert("omg", vec!["😱", "🤯"]);
    map.insert("yikes", vec!["😬", "😳"]);
    map.insert("oops", vec!["😬", "🤭"]);
    map.insert("awkward", vec!["😬", "😅"]);
    map.insert("nervous", vec!["😅", "😰"]);
    map.insert("sweating", vec!["😅", "💦"]);
    map.insert("tired", vec!["😴", "😪", "😩"]);
    map.insert("exhausted", vec!["😴", "😫"]);
    map.insert("done", vec!["😑", "💀"]);
    map.insert("upset", vec!["😠", "😡"]);
    map.insert("angry", vec!["😡", "🤬"]);
    map.insert("mad", vec!["😡", "😠"]);
    map.insert("gross", vec!["🤮", "🤢"]);
    map.insert("yuk", vec!["🤮", "🤢"]);
    map.insert("yuck", vec!["🤮", "🤢"]);
    map.insert("barf", vec!["🤮"]);
    map.insert("puke", vec!["🤮"]);
    map.insert("vomit", vec!["🤮"]);
    map.insert("sick", vec!["🤢", "🤮", "🤒"]);
    map.insert("nauseous", vec!["🤢"]);
    map.insert("nauseated", vec!["🤢", "🤮"]);
    map.insert("disgusting", vec!["🤮", "🤢"]);
    map.insert("love", vec!["❤️", "💕", "😍"]);
    map.insert("heart", vec!["❤️", "💕", "💖"]);
    map.insert("cute", vec!["🥰", "😊", "🩷"]);
    map.insert("kiss", vec!["😘", "💋"]);
    map.insert("hug", vec!["🤗", "🫂"]);
    map.insert("cool", vec!["😎", "🆒"]);
    map.insert("nice", vec!["👍", "👌"]);
    map.insert("ok", vec!["👌", "👍"]);
    map.insert("okay", vec!["👌", "👍"]);
    map.insert("yes", vec!["✅", "👍"]);
    map.insert("no", vec!["❌", "👎"]);
    map.insert("stop", vec!["✋", "🛑"]);
    map.insert("pray", vec!["🙏"]);
    map.insert("thanks", vec!["🙏", "👍"]);
    map.insert("please", vec!["🙏"]);

    // Money and success
    map.insert("money", vec!["💰", "💵", "💸"]);
    map.insert("cash", vec!["💵", "💰"]);
    map.insert("dollars", vec!["💵", "💲"]);
    map.insert("rich", vec!["💰", "💎", "🤑"]);
    map.insert("broke", vec!["💸", "😭"]);
    map.insert("expensive", vec!["💰", "💸"]);
    map.insert("cheap", vec!["💵"]);
    map.insert("100", vec!["💯"]);
    map.insert("perfect", vec!["💯", "✨"]);
    map.insert("win", vec!["🏆", "🥇"]);
    map.insert("winner", vec!["🏆", "👑"]);
    map.insert("champion", vec!["🏆", "👑"]);
    map.insert("king", vec!["👑", "🤴"]);
    map.insert("queen", vec!["👑", "👸"]);
    map.insert("boss", vec!["👔", "💼"]);
    map.insert("goals", vec!["🎯", "💯"]);
    map.insert("success", vec!["🎉", "🏆"]);

    // Food slang (beyond existing)
    map.insert("snack", vec!["😋", "🍪"]);
    map.insert("thirsty", vec!["💦", "🥵", "💜"]);
    map.insert("sausage", vec!["🌭", "🍆"]);
    map.insert("taco", vec!["🌮", "🍑"]);
    map.insert("cherry", vec!["🍒"]);
    map.insert("melons", vec!["🍉", "🍈"]);

    // Actions and activities
    map.insert("party", vec!["🎉", "🥳"]);
    map.insert("celebrate", vec!["🎉", "🍾"]);
    map.insert("dance", vec!["💃", "🕺"]);
    map.insert("sleep", vec!["😴", "💤"]);
    map.insert("nap", vec!["😴", "💤"]);
    map.insert("eat", vec!["🍽️", "😋"]);
    map.insert("hungry", vec!["🍔", "😋"]);
    map.insert("workout", vec!["💪", "🏋️"]);
    map.insert("gym", vec!["💪", "🏋️"]);
    map.insert("save", vec!["💾", "💿"]);
    map.insert("saved", vec!["💾", "✅"]);
    map.insert("run", vec!["🏃", "💨"]);
    map.insert("walk", vec!["🚶", "👣"]);
    map.insert("drive", vec!["🚗", "🚙"]);
    map.insert("fly", vec!["✈️", "🛫"]);
    map.insert("travel", vec!["✈️", "🌍"]);
    map.insert("vacation", vec!["🏖️", "✈️"]);
    map.insert("beach", vec!["🏖️", "🌊"]);
    map.insert("sun", vec!["☀️", "🌞"]);
    map.insert("rain", vec!["🌧️", "☔"]);
    map.insert("snow", vec!["❄️", "⛄"]);
    map.insert("storm", vec!["⛈️", "🌩️"]);

    // Technology and internet
    map.insert("computer", vec!["💻", "🖥️"]);
    map.insert("phone", vec!["📱", "☎️"]);
    map.insert("smartphone", vec!["📱"]);
    map.insert("internet", vec!["🌐", "💻"]);
    map.insert("wifi", vec!["📶", "🛜"]);
    map.insert("email", vec!["📧", "✉️"]);
    map.insert("message", vec!["💬", "📱"]);
    map.insert("chat", vec!["💬", "💭"]);
    map.insert("call", vec!["📞", "☎️"]);
    map.insert("video", vec!["📹", "🎥"]);
    map.insert("camera", vec!["📷", "📸"]);
    map.insert("photo", vec!["📷", "🖼️"]);
    map.insert("selfie", vec!["🤳", "📸"]);
    map.insert("game", vec!["🎮", "🕹️"]);
    map.insert("gaming", vec!["🎮", "🕹️"]);
    map.insert("music", vec!["🎵", "🎶"]);
    map.insert("song", vec!["🎵", "🎤"]);

    // Internet and gaming slang
    map.insert("spam", vec!["📧", "🚫"]);
    map.insert("hack", vec!["💻", "🔓"]);
    map.insert("hacker", vec!["👨‍💻", "🔓"]);
    map.insert("glitch", vec!["⚡", "🐛"]);
    map.insert("lag", vec!["🐌", "⏳"]);
    map.insert("afk", vec!["🚶", "⌨️"]);
    map.insert("brb", vec!["🔙", "⏰"]);
    map.insert("ttyl", vec!["👋", "💬"]);
    map.insert("gg", vec!["🎮", "🤝"]);
    map.insert("gamer", vec!["🎮", "👾"]);
    map.insert("noob", vec!["🐥", "🤡"]);
    map.insert("pro", vec!["⭐", "🏆"]);
    map.insert("pwn", vec!["💪", "🏆"]);
    map.insert("pwned", vec!["☠️", "💀"]);
    map.insert("owned", vec!["☠️", "😂"]);
    map.insert("rekt", vec!["💀", "☠️"]);
    map.insert("ez", vec!["😎", "💯"]);
    map.insert("easy", vec!["😎", "👍"]);

    // Misc popular culture
    map.insert("alien", vec!["👽", "🛸"]);
    map.insert("ufo", vec!["🛸", "👽"]);
    map.insert("robot", vec!["🤖"]);
    map.insert("zombie", vec!["🧟"]);
    map.insert("vampire", vec!["🧛"]);
    map.insert("witch", vec!["🧙", "🔮"]);
    map.insert("magic", vec!["✨", "🪄"]);
    map.insert("wizard", vec!["🧙", "🪄"]);
    map.insert("devil", vec!["😈", "👿"]);
    map.insert("angel", vec!["😇", "👼"]);
    map.insert("demon", vec!["👿", "😈"]);
    map.insert("skull", vec!["💀", "☠️"]);
    map.insert("pirate", vec!["🏴‍☠️", "☠️"]);
    map.insert("ninja", vec!["🥷"]);
    map.insert("superhero", vec!["🦸"]);
    map.insert("hero", vec!["🦸", "🏆"]);

    // Programming languages and tech
    map.insert("python", vec!["🐍"]);
    map.insert("rust", vec!["🦀"]);
    map.insert("javascript", vec!["🟨", "☕"]);
    map.insert("js", vec!["🟨"]);
    map.insert("typescript", vec!["🔷", "🔵"]);
    map.insert("ts", vec!["🔷"]);
    map.insert("java", vec!["☕"]);
    map.insert("go", vec!["🐹"]);
    map.insert("golang", vec!["🐹"]);
    map.insert("ruby", vec!["💎", "🔴"]);
    map.insert("php", vec!["🐘"]);
    map.insert("swift", vec!["🐦", "🧡"]);
    map.insert("kotlin", vec!["🅺", "🟣"]);
    map.insert("csharp", vec!["🎵", "💜"]);
    map.insert("cplusplus", vec!["⚙️"]);
    map.insert("cpp", vec!["⚙️"]);
    map.insert("c++", vec!["⚙️"]);
    map.insert("clang", vec!["🔧"]);
    map.insert("perl", vec!["🐪"]);
    map.insert("haskell", vec!["🎓", "λ"]);
    map.insert("elixir", vec!["💧", "🟣"]);
    map.insert("scala", vec!["⚖️", "🔴"]);
    map.insert("dart", vec!["🎯", "🔵"]);
    map.insert("rlang", vec!["📊", "📈"]);
    map.insert("julia", vec!["🟣", "🔴", "🟢"]);
    map.insert("lua", vec!["🌙", "🔵"]);
    map.insert("html", vec!["🌐", "📄"]);
    map.insert("css", vec!["🎨", "🎭"]);
    map.insert("sql", vec!["🗄️", "💾"]);
    map.insert("bash", vec!["🐚", "💻"]);
    map.insert("shell", vec!["🐚", "💻"]);
    map.insert("powershell", vec!["💙", "💻"]);
    map.insert("assembly", vec!["⚙️", "🔩"]);
    map.insert("asm", vec!["⚙️"]);

    // Programming and dev culture
    map.insert("bug", vec!["🐛"]);
    map.insert("debug", vec!["🐛🔨", "🔍"]);
    map.insert("code", vec!["💻", "👨‍💻"]);
    map.insert("coding", vec!["💻", "⌨️"]);
    map.insert("deploy", vec!["🚀", "📦"]);
    map.insert("shipped", vec!["🚢", "✅"]);
    map.insert("merge", vec!["🔀"]);
    map.insert("commit", vec!["💾", "✅"]);
    map.insert("push", vec!["⬆️", "📤"]);
    map.insert("pull", vec!["⬇️", "📥"]);
    map.insert("fork", vec!["🍴"]);
    map.insert("clone", vec!["👯‍♂️", "💾"]);
    map.insert("branch", vec!["🌳", "🔀"]);
    map.insert("git", vec!["🐙", "📦"]);
    map.insert("github", vec!["🐙", "😺"]);
    map.insert("gitlab", vec!["🦊"]);
    map.insert("stackoverflow", vec!["📚", "❓"]);
    map.insert("production", vec!["🏭", "🚀"]);
    map.insert("localhost", vec!["🏠💻", "💻"]);
    map.insert("server", vec!["🖥️", "☁️"]);
    map.insert("docker", vec!["🐳"]);
    map.insert("kubernetes", vec!["☸️"]);
    map.insert("k8s", vec!["☸️"]);
    map.insert("api", vec!["🔌", "🔗"]);
    map.insert("database", vec!["🗄️", "💾"]);
    map.insert("frontend", vec!["🎨", "💻"]);
    map.insert("backend", vec!["⚙️", "🖥️"]);
    map.insert("fullstack", vec!["🎨⚙️", "💻"]);
    map.insert("devops", vec!["🔧🚀", "⚙️"]);
    map.insert("ci", vec!["🔄", "✅"]);
    map.insert("cd", vec!["🚀", "📦"]);
    map.insert("cicd", vec!["🔄🚀"]);
    map.insert("cloud", vec!["☁️", "☁️💻"]);
    map.insert("aws", vec!["🟧", "☁️"]);
    map.insert("azure", vec!["🔵", "☁️"]);
    map.insert("gcp", vec!["🔵🔴🟡", "☁️"]);
    map.insert("linux", vec!["🐧"]);
    map.insert("unix", vec!["🖥️", "💻"]);
    map.insert("macos", vec!["🍎"]);
    map.insert("windows", vec!["🪟", "💻"]);
    map.insert("android", vec!["🤖", "📱"]);
    map.insert("ios", vec!["🍎", "📱"]);
    map.insert("vim", vec!["⌨️", "🟢"]);
    map.insert("emacs", vec!["⌨️", "🟣"]);
    map.insert("vscode", vec!["🔵", "💻"]);
    map.insert("ide", vec!["💻", "🔧"]);
    map.insert("chatgpt", vec!["🤖", "🧠"]);
    map.insert("gpt", vec!["🤖", "🧠"]);
    map.insert("gpt-4", vec!["🤖", "🧠✨"]);
    map.insert("gpt4", vec!["🤖", "🧠✨"]);
    map.insert("openai", vec!["🔵✨", "🤖"]);
    map.insert("codex", vec!["🧠", "💻"]);
    map.insert("github copilot", vec!["🧑‍✈️", "💻"]);
    map.insert("copilot", vec!["🧑‍✈️", "💻"]);
    map.insert("gemini", vec!["💎"]);
    map.insert("google gemini", vec!["💎"]);
    map.insert("claude", vec!["✳️"]);
    map.insert("claude code", vec!["✳️", "💻"]);
    map.insert("anthropic", vec!["✳️"]);
    map.insert("grok", vec!["🧠", "💡"]);
    map.insert("qwen", vec!["🧠", "🌏"]);
    map.insert("tongyi qianwen", vec!["🧠", "🌏"]);
    map.insert("llama", vec!["🦙", "🤖"]);
    map.insert("llama2", vec!["🦙", "🤖"]);
    map.insert("llama 2", vec!["🦙", "🤖"]);
    map.insert("mistral", vec!["🌬️", "🤖"]);
    map.insert("perplexity", vec!["❓", "🤖"]);
    map.insert("midjourney", vec!["🎨", "🤖"]);
    map.insert("stable diffusion", vec!["🎨", "🖌️"]);
    map.insert("runwayml", vec!["🎬", "🤖"]);

    // Science and academic
    map.insert("dna", vec!["🧬"]);
    map.insert("helix", vec!["🧬"]);
    map.insert("genes", vec!["🧬"]);
    map.insert("genome", vec!["🧬"]);
    map.insert("molecule", vec!["⚛️"]);
    map.insert("atom", vec!["⚛️"]);
    map.insert("science", vec!["🔬", "🧪"]);
    map.insert("chemistry", vec!["🧪", "⚗️"]);
    map.insert("biology", vec!["🧬", "🔬"]);
    map.insert("physics", vec!["⚛️", "🔬"]);
    map.insert("math", vec!["➗", "🔢"]);
    map.insert("calculus", vec!["∫", "📐"]);
    map.insert("algebra", vec!["✖️", "➕"]);
    map.insert("geometry", vec!["📐", "△"]);

    // Gitmoji
    map.insert("art", vec!["🎨"]);
    map.insert("zap", vec!["⚡"]);
    map.insert("ambulance", vec!["🚑"]);
    map.insert("sparkles", vec!["✨"]);
    map.insert("memo", vec!["📝"]);
    map.insert("rocket", vec!["🚀"]);
    map.insert("lipstick", vec!["💄"]);
    map.insert("tada", vec!["🎉"]);
    map.insert("white_check_mark", vec!["✅"]);
    map.insert("lock", vec!["🔒"]);
    map.insert("closed_lock_with_key", vec!["🔐"]);
    map.insert("bookmark", vec!["🔖"]);
    map.insert("rotating_light", vec!["🚨"]);
    map.insert("construction", vec!["🚧"]);
    map.insert("green_heart", vec!["💚"]);
    map.insert("arrow_down", vec!["⬇️"]);
    map.insert("arrow_up", vec!["⬆️"]);
    map.insert("pushpin", vec!["📌"]);
    map.insert("construction_worker", vec!["👷"]);
    map.insert("chart_with_upwards_trend", vec!["📈"]);
    map.insert("recycle", vec!["♻️"]);
    map.insert("heavy_plus_sign", vec!["➕"]);
    map.insert("heavy_minus_sign", vec!["➖"]);
    map.insert("wrench", vec!["🔧"]);
    map.insert("hammer", vec!["🔨"]);
    map.insert("globe_with_meridians", vec!["🌐"]);
    map.insert("pencil2", vec!["✏️"]);
    map.insert("poop", vec!["💩"]);
    map.insert("rewind", vec!["⏪"]);
    map.insert("twisted_rightwards_arrows", vec!["🔀"]);
    map.insert("package", vec!["📦"]);
    map.insert("truck", vec!["🚚"]);
    map.insert("page_facing_up", vec!["📄"]);
    map.insert("bento", vec!["🍱"]);
    map.insert("ok_hand", vec!["👌"]);
    map.insert("wheelchair", vec!["♿"]);
    map.insert("bulb", vec!["💡"]);
    map.insert("beers", vec!["🍻"]);
    map.insert("speech_balloon", vec!["💬"]);
    map.insert("card_file_box", vec!["🗃️"]);
    map.insert("loud_sound", vec!["🔊"]);
    map.insert("mute", vec!["🔇"]);
    map.insert("busts_in_silhouette", vec!["👥"]);
    map.insert("children_crossing", vec!["🚸"]);
    map.insert("building_construction", vec!["🏗️"]);
    map.insert("iphone", vec!["📱"]);
    map.insert("clown_face", vec!["🤡"]);
    map.insert("egg", vec!["🥚"]);
    map.insert("see_no_evil", vec!["🙈"]);
    map.insert("camera_flash", vec!["📸"]);
    map.insert("alembic", vec!["⚗️"]);
    map.insert("mag", vec!["🔍"]);
    map.insert("label", vec!["🏷️"]);
    map.insert("seedling", vec!["🌱"]);
    map.insert("triangular_flag_on_post", vec!["🚩"]);
    map.insert("goal_net", vec!["🥅"]);
    map.insert("dizzy", vec!["💫"]);
    map.insert("wastebasket", vec!["🗑️"]);
    map.insert("passport_control", vec!["🛂"]);
    map.insert("adhesive_bandage", vec!["🩹"]);
    map.insert("monocle_face", vec!["🧐"]);
    map.insert("coffin", vec!["⚰️"]);
    map.insert("test_tube", vec!["🧪"]);
    map.insert("necktie", vec!["👔"]);
    map.insert("stethoscope", vec!["🩺"]);
    map.insert("bricks", vec!["🧱"]);
    map.insert("technologist", vec!["🧑‍💻"]);
    map.insert("money_with_wings", vec!["💸"]);
    map.insert("thread", vec!["🧵"]);
    map.insert("safety_vest", vec!["🦺"]);

    // Gitmoji developer-friendly aliases
    map.insert("style", vec!["🎨"]);
    map.insert("format", vec!["🎨"]);
    map.insert("formatting", vec!["🎨"]);
    map.insert("lint", vec!["🎨", "🚨"]);
    map.insert("perf", vec!["⚡"]);
    map.insert("performance", vec!["⚡"]);
    map.insert("optimize", vec!["⚡"]);
    map.insert("optimization", vec!["⚡"]);
    map.insert("remove", vec!["🔥"]);
    map.insert("delete", vec!["🔥"]);
    map.insert("prune", vec!["🔥"]);
    map.insert("hotfix", vec!["🚑"]);
    map.insert("feat", vec!["✨"]);
    map.insert("feature", vec!["✨"]);
    map.insert("features", vec!["✨"]);
    map.insert("docs", vec!["📝"]);
    map.insert("documentation", vec!["📝"]);
    map.insert("doc", vec!["📝"]);
    map.insert("ship", vec!["🚀"]);
    map.insert("launch", vec!["🚀"]);
    map.insert("ui", vec!["💄"]);
    map.insert("theme", vec!["💄"]);
    map.insert("init", vec!["🎉"]);
    map.insert("initialcommit", vec!["🎉"]);
    map.insert("beer", vec!["🍻"]);
    map.insert("drunk", vec!["🍻"]);
    map.insert("celebration", vec!["🍻"]);
    map.insert("test", vec!["✅"]);
    map.insert("tests", vec!["✅"]);
    map.insert("testing", vec!["✅"]);
    map.insert("security", vec!["🔒"]);
    map.insert("secure", vec!["🔒"]);
    map.insert("vulnerability", vec!["🔒"]);
    map.insert("secrets", vec!["🔐"]);
    map.insert("credentials", vec!["🔐"]);
    map.insert("envvars", vec!["🔐"]);
    map.insert("tag", vec!["🔖"]);
    map.insert("version", vec!["🔖"]);
    map.insert("versioning", vec!["🔖"]);
    map.insert("release", vec!["🔖"]);
    map.insert("warning", vec!["🚨"]);
    map.insert("lintfix", vec!["🚨"]);
    map.insert("wip", vec!["🚧"]);
    map.insert("draft", vec!["🚧"]);
    map.insert("workinprogress", vec!["🚧"]);
    map.insert("cifix", vec!["💚"]);
    map.insert("buildfix", vec!["💚"]);
    map.insert("downgrade", vec!["⬇️"]);
    map.insert("upgrade", vec!["⬆️"]);
    map.insert("dependencypin", vec!["📌"]);
    map.insert("pindeps", vec!["📌"]);
    map.insert("ci-setup", vec!["👷"]);
    map.insert("pipeline", vec!["👷"]);
    map.insert("workflow", vec!["👷"]);
    map.insert("analytics", vec!["📈"]);
    map.insert("tracking", vec!["📈"]);
    map.insert("refactor", vec!["♻️"]);
    map.insert("cleanup", vec!["♻️", "🗑️"]);
    map.insert("badcode", vec!["💩"]);
    map.insert("hacky", vec!["💩"]);
    map.insert("tempfix", vec!["💩"]);
    map.insert("add-dependency", vec!["➕"]);
    map.insert("add-dep", vec!["➕"]);
    map.insert("remove-dependency", vec!["➖"]);
    map.insert("remove-dep", vec!["➖"]);
    map.insert("config", vec!["🔧"]);
    map.insert("configuration", vec!["🔧"]);
    map.insert("settings", vec!["🔧"]);
    map.insert("chore", vec!["🔧"]);
    map.insert("maintenance", vec!["🔧"]);
    map.insert("build", vec!["🔨"]);
    map.insert("tooling", vec!["🔨"]);
    map.insert("scripts", vec!["🔨"]);
    map.insert("i18n", vec!["🌐"]);
    map.insert("l10n", vec!["🌐"]);
    map.insert("localization", vec!["🌐"]);
    map.insert("internationalization", vec!["🌐"]);
    map.insert("typo", vec!["✏️"]);
    map.insert("spelling", vec!["✏️"]);
    map.insert("revert", vec!["⏪"]);
    map.insert("rollback", vec!["⏪"]);
    map.insert("bundle", vec!["📦"]);
    map.insert("vendor", vec!["📦"]);
    map.insert("move", vec!["🚚"]);
    map.insert("rename", vec!["🚚"]);
    map.insert("license", vec!["📄"]);
    map.insert("licence", vec!["📄"]);
    map.insert("assets", vec!["🍱"]);
    map.insert("staticassets", vec!["🍱"]);
    map.insert("review", vec!["👌"]);
    map.insert("code-review", vec!["👌"]);
    map.insert("a11y", vec!["♿"]);
    map.insert("accessibility", vec!["♿"]);
    map.insert("comment", vec!["💡"]);
    map.insert("comments", vec!["💡"]);
    map.insert("copy", vec!["💬"]);
    map.insert("text", vec!["💬"]);
    map.insert("strings", vec!["💬"]);
    map.insert("db", vec!["🗃️"]);
    map.insert("database-migration", vec!["🗃️"]);
    map.insert("logging", vec!["🔊"]);
    map.insert("logs", vec!["🔊"]);
    map.insert("remove-logs", vec!["🔇"]);
    map.insert("silence", vec!["🔇"]);
    map.insert("contributors", vec!["👥"]);
    map.insert("authors", vec!["👥"]);
    map.insert("ux", vec!["🚸"]);
    map.insert("usability", vec!["🚸"]);
    map.insert("architecture", vec!["🏗️"]);
    map.insert("arch", vec!["🏗️"]);
    map.insert("responsive", vec!["📱"]);
    map.insert("mobile", vec!["📱"]);
    map.insert("mock", vec!["🤡"]);
    map.insert("mocks", vec!["🤡"]);
    map.insert("easteregg", vec!["🥚"]);
    map.insert("ignore", vec!["🙈"]);
    map.insert("gitignore", vec!["🙈"]);
    map.insert("snapshot", vec!["📸"]);
    map.insert("snapshots", vec!["📸"]);
    map.insert("experiment", vec!["⚗️"]);
    map.insert("experiments", vec!["⚗️"]);
    map.insert("seo", vec!["🔍"]);
    map.insert("search", vec!["🔍"]);
    map.insert("types", vec!["🏷️"]);
    map.insert("typing", vec!["🏷️"]);
    map.insert("seed", vec!["🌱"]);
    map.insert("seeding", vec!["🌱"]);
    map.insert("fixtures", vec!["🌱"]);
    map.insert("featureflag", vec!["🚩"]);
    map.insert("featureflags", vec!["🚩"]);
    map.insert("flag", vec!["🚩"]);
    map.insert("errors", vec!["🥅"]);
    map.insert("errorhandling", vec!["🥅"]);
    map.insert("catching", vec!["🥅"]);
    map.insert("animation", vec!["💫"]);
    map.insert("animations", vec!["💫"]);
    map.insert("transition", vec!["💫"]);
    map.insert("deprecate", vec!["🗑️"]);
    map.insert("deprecated", vec!["🗑️"]);
    map.insert("auth", vec!["🛂", "🔒"]);
    map.insert("authentication", vec!["🛂"]);
    map.insert("authorization", vec!["🛂"]);
    map.insert("login", vec!["🛂"]);
    map.insert("patch", vec!["🩹"]);
    map.insert("quickfix", vec!["🩹"]);
    map.insert("investigate", vec!["🧐"]);
    map.insert("analysis", vec!["🧐"]);
    map.insert("deadcode", vec!["⚰️"]);
    map.insert("cleanup-deadcode", vec!["⚰️"]);
    map.insert("failingtest", vec!["🧪"]);
    map.insert("regression-test", vec!["🧪"]);
    map.insert("business", vec!["👔"]);
    map.insert("domain", vec!["👔"]);
    map.insert("healthcheck", vec!["🩺"]);
    map.insert("monitoring", vec!["🩺"]);
    map.insert("infra", vec!["🧱"]);
    map.insert("infrastructure", vec!["🧱"]);
    map.insert("dx", vec!["🧑‍💻"]);
    map.insert("developer-experience", vec!["🧑‍💻"]);
    map.insert("billing", vec!["💸"]);
    map.insert("payments", vec!["💸"]);
    map.insert("costs", vec!["💸"]);
    map.insert("threading", vec!["🧵"]);
    map.insert("concurrency", vec!["🧵"]);
    map.insert("validation", vec!["🦺"]);
    map.insert("guardrails", vec!["🦺"]);
    map.insert("safety", vec!["🦺"]);
    map
});

/// Search for emojis matching the query using the comprehensive Unicode emoji database
fn search_emojis(query: &str, limit: usize) -> Vec<(String, &'static emojis::Emoji)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut seen: HashMap<String, bool> = HashMap::new();

    // 1. Check custom slang mappings first - exact match
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
                    if let Some(dummy_emoji) = emojis::iter().next() {
                        results.push((
                            format!("__raw__:{}|{}", emoji_str, query_lower),
                            dummy_emoji,
                        ));
                        seen.insert(key, true);
                    }
                }
            }
        }

        if results.len() >= limit {
            return results;
        }
    }

    // 2. Exact matches on standard emoji names
    for emoji in emojis::iter() {
        if results.len() >= limit {
            break;
        }

        let key = emoji.as_str().to_string();
        if seen.contains_key(&key) {
            continue;
        }

        // Check name (e.g., "smiling face")
        if emoji.name().to_lowercase() == query_lower {
            results.push((emoji.name().to_lowercase().replace(' ', ""), emoji));
            seen.insert(key, true);
            continue;
        }

        // Check shortcodes (e.g., ":smile:")
        for shortcode in emoji.shortcodes() {
            if shortcode.trim_matches(':').to_lowercase() == query_lower {
                results.push((shortcode.trim_matches(':').to_string(), emoji));
                seen.insert(key, true);
                break;
            }
        }
    }

    // 3. Check custom slang mappings - prefix match
    if results.len() < limit {
        for (slang_term, slang_emojis) in SLANG_MAP.iter() {
            // Skip exact matches (already handled)
            if *slang_term == query_lower.as_str() {
                continue;
            }

            // Check if slang term starts with query
            if slang_term.starts_with(&query_lower) {
                for emoji_str in slang_emojis {
                    if results.len() >= limit {
                        break;
                    }

                    let key = emoji_str.to_string();
                    if seen.contains_key(&key) {
                        continue;
                    }

                    if let Some(emoji_obj) = emojis::get(emoji_str) {
                        results.push((slang_term.to_string(), emoji_obj));
                        seen.insert(key, true);
                    } else {
                        let mut found = false;
                        for emoji in emojis::iter() {
                            if emoji.as_str() == *emoji_str {
                                results.push((slang_term.to_string(), emoji));
                                seen.insert(key.clone(), true);
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            if let Some(dummy_emoji) = emojis::iter().next() {
                                results.push((
                                    format!("__raw__:{}|{}", emoji_str, slang_term),
                                    dummy_emoji,
                                ));
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

    // 4. Prefix matches on standard emoji names
    if results.len() < limit {
        for emoji in emojis::iter() {
            if results.len() >= limit {
                break;
            }

            let key = emoji.as_str().to_string();
            if seen.contains_key(&key) {
                continue;
            }

            let name_normalized = emoji.name().to_lowercase();

            // Check if name starts with query
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

    // 5. Check custom slang mappings - substring match
    if results.len() < limit {
        for (slang_term, slang_emojis) in SLANG_MAP.iter() {
            // Skip exact and prefix matches (already handled)
            if *slang_term == query_lower.as_str() || slang_term.starts_with(&query_lower) {
                continue;
            }

            // Check if slang term contains query
            if slang_term.contains(&query_lower) {
                for emoji_str in slang_emojis {
                    if results.len() >= limit {
                        break;
                    }

                    let key = emoji_str.to_string();
                    if seen.contains_key(&key) {
                        continue;
                    }

                    if let Some(emoji_obj) = emojis::get(emoji_str) {
                        results.push((slang_term.to_string(), emoji_obj));
                        seen.insert(key, true);
                    } else {
                        let mut found = false;
                        for emoji in emojis::iter() {
                            if emoji.as_str() == *emoji_str {
                                results.push((slang_term.to_string(), emoji));
                                seen.insert(key.clone(), true);
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            if let Some(dummy_emoji) = emojis::iter().next() {
                                results.push((
                                    format!("__raw__:{}|{}", emoji_str, slang_term),
                                    dummy_emoji,
                                ));
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

    // 6. Substring matches on standard emoji names
    if results.len() < limit {
        for emoji in emojis::iter() {
            if results.len() >= limit {
                break;
            }

            let key = emoji.as_str().to_string();
            if seen.contains_key(&key) {
                continue;
            }

            let name_normalized = emoji.name().to_lowercase();

            // Check if name contains query
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
    let mut query_joined = query.join(" ");

    // Check if we should combine results (query ends with +)
    let combine_results = query_joined.trim().ends_with('+');
    if combine_results {
        query_joined = query_joined.trim().trim_end_matches('+').trim().to_string();
    }

    if query_joined.is_empty() {
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

    let processed_results: Vec<(String, String)> = results
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
        .collect();

    if combine_results && !processed_results.is_empty() {
        let combined_emojis: String = processed_results
            .iter()
            .map(|(_, emoji)| emoji.as_str())
            .collect();
        return vec![(query_joined, combined_emojis)];
    }

    processed_results
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
        } else if !args.query.is_empty() && !args.query.join("").trim().is_empty() {
            eprintln!("No emojis found for: {}", args.query.join(" "));
            std::process::exit(1);
        }
        return;
    }

    if args.alfred {
        let alfred_items: Vec<AlfredItem> = results
            .into_iter()
            .map(|(keyword, emoji)| {
                if keyword.starts_with("__raw__:") {
                    let parts: Vec<&str> = keyword[8..].splitn(2, '|').collect();
                    let raw_emoji = parts[0];
                    let actual_keyword = if parts.len() > 1 { parts[1] } else { "" };
                    AlfredItem {
                        uid: actual_keyword.to_string(),
                        title: raw_emoji.to_string(),
                        subtitle: actual_keyword.to_string(),
                        arg: format!("{} ", raw_emoji),
                        text: AlfredItemText {
                            copy: format!("{} ", raw_emoji),
                            largetype: raw_emoji.to_string(),
                        },
                        valid: true,
                    }
                } else {
                    AlfredItem {
                        uid: keyword.clone(),
                        title: emoji.clone(),
                        subtitle: keyword,
                        arg: format!("{} ", emoji),
                        text: AlfredItemText {
                            copy: format!("{} ", emoji),
                            largetype: emoji,
                        },
                        valid: true,
                    }
                }
            })
            .collect();

        let response = AlfredResponse {
            items: alfred_items,
        };
        println!("{}", serde_json::to_string(&response).unwrap());
    } else {
        for (keyword, emoji) in results {
            if keyword.starts_with("__raw__:") {
                let parts: Vec<&str> = keyword[8..].splitn(2, '|').collect();
                let raw_emoji = parts[0];
                let actual_keyword = if parts.len() > 1 { parts[1] } else { "" };
                println!("{} ({})", raw_emoji, actual_keyword);
            } else {
                println!("{} ({})", emoji, keyword);
            }
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

    #[test]
    fn test_combined_results_suffix() {
        // Test with "fire+"
        let query = vec!["fire+".to_string()];
        let results = find_emojis(&query, 10, &None, &None);

        assert_eq!(results.len(), 1);
        let (keyword, emoji) = &results[0];
        assert_eq!(keyword, "fire");
        // Should contain multiple fire emojis concatenated
        assert!(emoji.contains("🔥"));
        assert!(emoji.chars().count() > 1);
    }

    #[test]
    fn test_combined_results_separate_arg() {
        // Test with "fire +"
        let query = vec!["fire".to_string(), "+".to_string()];
        let results = find_emojis(&query, 10, &None, &None);

        assert_eq!(results.len(), 1);
        let (keyword, emoji) = &results[0];
        assert_eq!(keyword, "fire");
        assert!(emoji.contains("🔥"));
        assert!(emoji.chars().count() > 1);
    }

    #[test]
    fn test_combined_results_no_plus() {
        // Test with "fire" (normal behavior)
        let query = vec!["fire".to_string()];
        let results = find_emojis(&query, 10, &None, &None);

        assert!(results.len() > 1); // Should return multiple individual results
    }

    #[test]
    fn test_combined_results_empty_plus() {
        // Test with "+"
        let query = vec!["+".to_string()];
        let results = find_emojis(&query, 10, &None, &None);

        assert!(results.is_empty());
    }
}
