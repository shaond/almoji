use clap::{Parser, ValueEnum};
use std::collections::HashMap;
use once_cell::sync::Lazy;

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
    map.insert("cunt", vec!["🌮"]);
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
    map.insert("oral", vec!["👅", "🍆"]);
    map.insert("blowjob", vec!["👅", "🍆"]);
    map.insert("bj", vec!["👅", "🍆"]);
    map.insert("deepthroat", vec!["🍆", "👅"]);
    map.insert("anal", vec!["🍑", "🍆"]);
    map.insert("pegging", vec!["🍑", "🍆"]);
    map.insert("69", vec!["👅", "💦"]);
    map.insert("sixtynine", vec!["👅", "💦"]);
    map.insert("facial", vec!["💦", "😮"]);
    map.insert("creampie", vec!["💦", "🥧"]);
    map.insert("squirt", vec!["💦"]);
    map.insert("orgasm", vec!["💦", "😩"]);
    map.insert("climax", vec!["💦", "😫"]);
    map.insert("masturbate", vec!["✊", "💦"]);
    map.insert("jerkoff", vec!["✊", "💦"]);
    map.insert("wank", vec!["✊", "💦"]);
    map.insert("fap", vec!["✊", "💦"]);
    map.insert("fingering", vec!["👆", "💦"]);
    map.insert("handjob", vec!["✊", "🍆"]);
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
    map.insert("dead", vec!["💀"]);
    map.insert("dying", vec!["💀"]);
    map.insert("skull", vec!["💀"]);
    map.insert("fire", vec!["🔥"]);
    map.insert("lit", vec!["🔥"]);
    map.insert("dope", vec!["🔥"]);
    map.insert("cap", vec!["🧢"]);
    map.insert("lying", vec!["🧢", "🤥"]);
    map.insert("fake", vec!["🧢", "🐍"]);
    map.insert("nocap", vec!["🚫🧢"]);
    map.insert("clown", vec!["🤡"]);
    map.insert("clowning", vec!["🤡"]);
    map.insert("foolish", vec!["🤡"]);
    map.insert("stupid", vec!["🤡"]);
    map.insert("cringe", vec!["😬"]);
    map.insert("oof", vec!["😬", "😅"]);
    map.insert("yeet", vec!["💨", "🚀"]);
    map.insert("vibes", vec!["✨", "🌊"]);
    map.insert("vibe", vec!["💚", "✨"]);
    map.insert("slay", vec!["💅", "👑", "😭", "🎯"]);
    map.insert("sus", vec!["🤨", "🔍", "💩"]);
    map.insert("suspicious", vec!["🤨", "🔍"]);
    map.insert("shook", vec!["😱", "🤯"]);
    map.insert("mood", vec!["💯"]);
    map.insert("bet", vec!["💯"]);
    map.insert("flex", vec!["💪", "💎", "🏆"]);
    map.insert("drip", vec!["💧", "💎"]);
    map.insert("ghosted", vec!["👻"]);
    map.insert("ghost", vec!["👻"]);
    map.insert("shade", vec!["☕", "😎"]);
    map.insert("tea", vec!["☕", "🫖"]);
    map.insert("gossip", vec!["☕", "👀"]);
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

    // Country/Location slang
    map.insert("aussie", vec!["🇦🇺"]);
    map.insert("oz", vec!["🇦🇺"]);
    map.insert("straya", vec!["🇦🇺"]);
    map.insert("usa", vec!["🇺🇸"]);
    map.insert("america", vec!["🇺🇸"]);
    map.insert("murica", vec!["🇺🇸"]);
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

    // Australian English slang
    map.insert("roo", vec!["🦘"]);
    map.insert("arvo", vec!["🏖️"]);
    map.insert("ripper", vec!["😎"]);
    map.insert("outback", vec!["🦎"]);
    map.insert("huntsman", vec!["🕷️"]);
    map.insert("goon", vec!["🍷"]);
    map.insert("croc", vec!["🐊"]);
    map.insert("tropical", vec!["🌴"]);

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

    map
});

/// Search for emojis matching the query using the comprehensive Unicode emoji database
fn search_emojis(query: &str, limit: usize) -> Vec<(String, &'static emojis::Emoji)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut seen = HashMap::new();

    // Check custom slang mappings first
    if let Some(slang_emojis) = SLANG_MAP.get(query_lower.as_str()) {
        for emoji_str in slang_emojis {
            if results.len() >= limit {
                break;
            }

            // Try to find the emoji in the database
            if let Some(emoji_obj) = emojis::get(emoji_str) {
                let key = emoji_obj.as_str().to_string();
                if !seen.contains_key(&key) {
                    results.push((query_lower.clone(), emoji_obj));
                    seen.insert(key, true);
                }
            } else {
                // For compound emojis not in the database, try to find by iterating
                for emoji in emojis::iter() {
                    if emoji.as_str() == *emoji_str {
                        let key = emoji.as_str().to_string();
                        if !seen.contains_key(&key) {
                            results.push((query_lower.clone(), emoji));
                            seen.insert(key, true);
                            break;
                        }
                    }
                }
            }
        }

        // If we found slang matches and hit the limit, return early
        if results.len() >= limit {
            return results;
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
/// Note: Gender variants are already in the emoji database (e.g., "man firefighter", "woman firefighter")
/// so we don't apply gender modifiers programmatically as they can render incorrectly
fn apply_gender(emoji_str: &str, _gender: &Gender) -> String {
    // Return emoji as-is since gendered variants are already in the database
    emoji_str.to_string()
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
