# Almoji 🎉

A blazingly fast emoji search CLI for macOS that helps you find emojis using keywords.

## Features

- **Extremely Fast**: Uses compile-time perfect hash maps (PHF) for O(1) emoji lookups
- **No External Dependencies**: Emojis are stored as Unicode characters in the binary itself
- **System Font Integration**: Emojis render using macOS's native Apple Color Emoji font
- **Smart Search**: Supports exact matches, prefix matching, and substring matching
- **Customizable**: Support for skin tone modifiers and gender variants
- **Multi-word Search**: Search for emojis using multiple keywords
- **Alfred Integration**: Included Alfred workflow for quick emoji access
- **500+ Emojis**: Comprehensive database covering smileys, animals, food, activities, travel, objects, and symbols

## Installation

### Quick Install

```bash
# Clone the repository
git clone https://github.com/shaond/almoji.git
cd almoji

# Install to user directory (~/.cargo/bin)
./install.sh

# OR install system-wide (/usr/local/bin)
./install.sh --system
```

### Manual Installation

```bash
# Install to user directory
cargo install --path .

# OR install system-wide
cargo install --path . --root /usr/local
```

### Alfred Workflow

An Alfred workflow is included for quick emoji searching directly from Alfred! See [ALFRED_WORKFLOW.md](ALFRED_WORKFLOW.md) for installation and usage instructions.

**Quick start:**
1. Install Almoji CLI (see above)
2. Double-click `Almoji-Workflow.alfredworkflow` to install
3. Type `;` in Alfred followed by your search term

The workflow includes a custom icon and supports skin tone/gender configuration.

## Usage

Basic usage:

```bash
# Search for an emoji
almoji fire
# Output: 🔥 (fire)

almoji heart
# Output: ❤️ (heart)
#        💕 (hearts)

# Limit the number of results
almoji --limit 3 sun
# Output: ☀️ (sun)
#        😎 (sunglasses)
#        🌻 (sunflower)
```

### Command Line Options

```
almoji [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Search query for emojis (can be multiple words)

Options:
  -l, --limit <LIMIT>            Maximum number of results to return [default: 10]
  -g, --gender <GENDER>          Gender variant [possible values: male, female, neutral]
  -s, --skin-tone <SKIN_TONE>    Skin tone variant [possible values: light, medium-light,
                                 medium, medium-dark, dark]
  -h, --help                     Print help
  -V, --version                  Print version
```

### Advanced Usage

**Skin Tone Modifiers:**
```bash
almoji --skin-tone medium wave
# Output: 👋🏽 (wave)

almoji -s dark thumbsup
# Output: 👍🏿 (thumbsup)
```

**Gender Variants:**
```bash
almoji --gender female shrug
# Output: 🤷‍♀️ (shrug)

almoji -g male facepalm
# Output: 🤦‍♂️ (facepalm)
```

**Combining Options:**
```bash
almoji --skin-tone medium-dark --gender female --limit 5 person
# Search with both modifiers and limit results
```

## Examples

```bash
# Smileys & Emotions
almoji happy     # 😊
almoji laugh     # 😂
almoji love      # ❤️

# Animals
almoji dog       # 🐕
almoji cat       # 🐈
almoji unicorn   # 🦄

# Food
almoji pizza     # 🍕
almoji coffee    # ☕
almoji taco      # 🌮

# Activities
almoji fire      # 🔥
almoji music     # 🎵
almoji party     # 🎉

# Travel
almoji plane     # ✈️
almoji rocket    # 🚀
almoji car       # 🚗

# Objects
almoji phone     # 📱
almoji laptop    # 💻
almoji light     # 💡
```

## How It Works

### Performance Architecture

1. **Compile-time Perfect Hash Map**: Uses the `phf` crate to generate a perfect hash function at compile time, enabling O(1) lookups with zero runtime overhead

2. **Static Data**: All emoji mappings are embedded directly in the binary, eliminating file I/O and external dependencies

3. **Smart Search Algorithm**:
   - First, tries exact keyword match
   - Then, performs prefix matching (e.g., "fir" matches "fire")
   - Finally, performs substring matching for broader results

4. **Zero-copy**: Uses static string references throughout, avoiding heap allocations

### Why It's Fast

- **No external files**: Everything is compiled into the binary
- **No runtime initialization**: PHF maps are initialized at compile time
- **Minimal allocations**: Most operations use static references
- **Optimized release builds**: Built with `--release` flag for maximum performance

## Categories

The emoji database includes:

- Smileys & Emotions (happy, sad, angry, love, etc.)
- Gestures (wave, thumbsup, pray, etc.)
- People & Family
- Animals & Nature (dog, cat, tree, flower, etc.)
- Food & Drink (pizza, coffee, beer, etc.)
- Activities & Sports (soccer, music, art, etc.)
- Travel & Places (plane, car, hotel, etc.)
- Objects (phone, computer, book, etc.)
- Symbols (check, warning, arrows, etc.)
- Flags (common country flags)

## Testing

Run the test suite:

```bash
cargo test
```

## Technical Details

- **Language**: Rust
- **Key Dependencies**:
  - `clap`: Command-line argument parsing
  - `phf`: Perfect hash function for compile-time hash maps
- **Platform**: Optimized for macOS but works on any system with Unicode emoji support

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! To add more emojis:

1. Edit `src/main.rs`
2. Add new keyword → emoji mappings to the `EMOJI_MAP`
3. Ensure no duplicate keys
4. Run `cargo test` to verify
5. Submit a pull request

## Acknowledgments

- Emoji characters are from the Unicode Standard
- Rendered using the system's default emoji font (Apple Color Emoji on macOS)
