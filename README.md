# Almoji 🎉

A blazingly fast emoji search tool for macOS - available as both a CLI and menubar application!

## Features

### CLI Features
- **Extremely Fast**: Uses compile-time perfect hash maps for O(1) emoji lookups
- **No External Dependencies**: Emojis are stored as Unicode characters in the binary itself
- **System Font Integration**: Emojis render using macOS's native Apple Color Emoji font
- **Smart Search**: Supports exact matches, prefix matching, and substring matching
- **Customizable**: Support for skin tone modifiers and gender variants
- **Multi-word Search**: Search for emojis using multiple keywords
- **Alfred Integration**: Included Alfred workflow for quick emoji access

### Menubar App Features (NEW!)
- **macOS Menubar Integration**: Lives in your menubar for instant access
- **Global Hotkey**: Trigger emoji search from anywhere with a customizable hotkey (default: Cmd+Shift+E)
- **Beautiful GUI**: Clean, keyboard-navigable search interface built with egui
- **Auto-copy**: Selected emojis are automatically copied to clipboard
- **Settings Panel**: Configure your hotkey preferences
- **Pure Rust**: No Swift, Objective-C, or JavaScript - 100% Rust implementation
- **500+ Emojis & Slang**: Comprehensive database including Gen Z slang, tech terms, and cultural references

## Installation

### Quick Install (macOS - Recommended)

On macOS, the install script automatically installs both the CLI and menubar app:

```bash
# Clone the repository
git clone https://github.com/shaond/almoji.git
cd almoji

# Install both CLI and menubar app to ~/.cargo/bin
./install.sh

# OR install system-wide to /usr/local/bin
./install.sh --system
```

After installation, run the menubar app:
```bash
almoji-menubar
```

The menubar app will appear in your macOS menubar with a 😊 icon. Click it to:
- **Search Emoji**: Open the emoji search window
- **Settings**: Configure your global hotkey
- **Quit**: Exit the app

You can also trigger the search window anytime using the global hotkey (default: Cmd+Shift+E).

**Tip:** Add `almoji-menubar` to your Login Items in System Settings to start it automatically on login.

### CLI Installation

```bash
# Clone the repository
git clone https://github.com/shaond/almoji.git
cd almoji

# Install to user directory (~/.cargo/bin)
./install.sh

# OR install system-wide (/usr/local/bin)
./install.sh --system
```

### Manual CLI Installation

```bash
# Install to user directory
cargo install --path . --bin almoji

# OR install system-wide
cargo install --path . --bin almoji --root /usr/local
```

### Alfred Workflow

An Alfred workflow is included for quick emoji searching directly from Alfred! See [ALFRED_WORKFLOW.md](ALFRED_WORKFLOW.md) for installation and usage instructions.

**Quick start:**
1. Install Almoji CLI (see above)
2. Double-click `Almoji-Workflow.alfredworkflow` to install
3. Type `;` in Alfred followed by your search term

The workflow includes a custom icon and supports skin tone/gender configuration.

## Usage

### Menubar App Usage

**Quick Start:**
1. Launch `almoji-menubar`
2. The 😊 icon will appear in your macOS menubar
3. Press Cmd+Shift+E (or your custom hotkey) to open the search window
4. Type to search for emojis
5. Use arrow keys to navigate results
6. Press Enter to copy the selected emoji to clipboard
7. The emoji is now ready to paste anywhere!

**Keyboard Shortcuts:**
- **Cmd+Shift+E** (default): Open emoji search window
- **Arrow Keys**: Navigate search results
- **Enter**: Select and copy emoji
- **Escape**: Close search window

**Customizing the Hotkey:**
1. Click the menubar icon
2. Select "Settings..."
3. Choose your preferred modifiers (Ctrl, Alt, Shift, Cmd)
4. Select your preferred key
5. Click "Save"

**Settings Location:**
Settings are stored in `~/.config/almoji/settings.json`

### CLI Usage

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

### CLI
- **Language**: Rust
- **Key Dependencies**:
  - `clap`: Command-line argument parsing
  - `emojis`: Unicode emoji database
  - `once_cell`: Lazy static initialization
- **Platform**: Optimized for macOS but works on any system with Unicode emoji support

### Menubar App
- **Language**: 100% Rust
- **Key Dependencies**:
  - `eframe` / `egui`: Immediate mode GUI framework
  - `global-hotkey`: Cross-platform global hotkey registration
  - `arboard`: Clipboard access
  - `cocoa` / `objc`: macOS native menubar integration (via Rust bindings)
  - `serde` / `serde_json`: Settings persistence
- **Platform**: macOS only (uses native NSStatusBar APIs)

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome!

### To add more emojis or slang terms:

1. Edit `src/emoji_search.rs`
2. Add new keyword → emoji mappings to the `SLANG_MAP`
3. Ensure no duplicate keys
4. Run `cargo test` to verify
5. Submit a pull request

### Project Structure

```
almoji/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── menubar_main.rs      # Menubar app entry point
│   ├── emoji_search.rs      # Shared emoji search logic
│   └── menubar/             # Menubar app modules
│       ├── mod.rs           # Module exports
│       ├── gui.rs           # Search & settings UI
│       ├── settings.rs      # Settings management
│       └── macos_statusbar.rs # macOS menubar integration
├── Cargo.toml               # Dependencies and build config
└── README.md                # This file
```

## Acknowledgments

- Emoji characters are from the Unicode Standard
- Rendered using the system's default emoji font (Apple Color Emoji on macOS)
