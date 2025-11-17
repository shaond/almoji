# Almoji Alfred Workflow

A blazingly fast emoji search workflow for Alfred using the Almoji CLI.

## Features

- **Fast Emoji Search**: Search through 500+ emojis using keywords
- **Customizable**: Set default skin tone and gender variants
- **Simple Interface**: Just type `;` followed by your search term
- **Copy to Clipboard**: Press Enter to copy the selected emoji

## Requirements

- **Alfred Powerpack**: This workflow requires the Alfred Powerpack
- **Almoji CLI**: The Almoji command-line tool must be installed and available in your PATH

## Installation

### 1. Install Almoji CLI

First, install the Almoji CLI tool:

```bash
# Clone the repository
git clone https://github.com/shaond/almoji.git
cd almoji

# Build and install
cargo install --path .

# Or copy the binary to your PATH
cargo build --release
cp target/release/almoji /usr/local/bin/
```

Verify installation:
```bash
almoji fire
# Should output: 🔥 (fire)
```

### 2. Install the Workflow

1. Download the `Almoji.alfredworkflow` file
2. Double-click the file to install it in Alfred
3. Alfred will open and import the workflow

Alternatively, you can manually copy the workflow:
```bash
cp -r Almoji.alfredworkflow ~/Library/Application\ Support/Alfred/Alfred.alfredpreferences/workflows/
```

## Usage

### Basic Search

1. Open Alfred (default: `⌘ + Space`)
2. Type `;` followed by your search term
3. Browse the emoji results
4. Press `Enter` to copy the emoji to your clipboard

### Examples

```
; fire        → 🔥 🔥 (fire, hot)
; heart       → ❤️ 💕 💗 (heart, love, hearts)
; happy       → 😊 😀 😂 (happy, smile, laugh)
; dog         → 🐕 🐶 (dog, puppy)
; pizza       → 🍕 (pizza)
```

## Configuration

The workflow supports two configuration options:

### Skin Tone

Set a default skin tone for emojis that support it:

1. Right-click the workflow in Alfred Preferences
2. Select "Configure Workflow..."
3. Set the "Skin Tone" field to one of:
   - `light`
   - `medium-light`
   - `medium`
   - `medium-dark`
   - `dark`

Example: `medium` → 👋🏽

### Gender

Set a default gender variant for emojis that support it:

1. Right-click the workflow in Alfred Preferences
2. Select "Configure Workflow..."
3. Set the "Gender" field to one of:
   - `male`
   - `female`
   - `neutral`

Example: `female` → 🤷‍♀️

**Note**: Leave these fields empty to use the default (neutral/yellow) emojis.

## Customization

### Change the Keyword

If you prefer a different trigger keyword than `;`:

1. Open Alfred Preferences
2. Go to Workflows → Almoji
3. Double-click the Script Filter object
4. Change the "Keyword" field to your preferred trigger

### Modify Search Limit

By default, the workflow shows up to 20 results. To change this:

1. Open Alfred Preferences
2. Go to Workflows → Almoji
3. Double-click the Script Filter object
4. In the script, modify the `--limit 20` value

## Troubleshooting

### "No emojis found" appears for all searches

**Problem**: Almoji CLI is not found in PATH

**Solution**:
1. Verify Almoji is installed: `which almoji`
2. If not found, install Almoji or add its location to your PATH
3. Restart Alfred after installing

### Configuration variables not working

**Problem**: Skin tone or gender settings aren't being applied

**Solution**:
1. Check the exact spelling of the values (must be lowercase)
2. Valid skin tones: `light`, `medium-light`, `medium`, `medium-dark`, `dark`
3. Valid genders: `male`, `female`, `neutral`
4. Restart Alfred after changing configuration

### Workflow doesn't appear in Alfred

**Problem**: Workflow not properly installed

**Solution**:
1. Make sure you have Alfred Powerpack
2. Double-click the `.alfredworkflow` file again
3. Check Alfred Preferences → Workflows to see if it's listed

## How It Works

The workflow uses Alfred's Script Filter feature to:

1. Capture your search query
2. Call the Almoji CLI with the query and configuration options
3. Parse the output and format it as JSON for Alfred
4. Display the results in Alfred's interface
5. Copy the selected emoji to clipboard when you press Enter

## Performance

- **Fast Lookup**: Almoji uses compile-time perfect hash maps for O(1) emoji lookups
- **No Lag**: Results appear instantly as you type
- **Efficient**: All emoji data is compiled into the binary, no file I/O needed

## Categories

Search across these emoji categories:

- Smileys & Emotions (happy, sad, angry, love)
- Gestures (wave, thumbsup, pray)
- People & Family
- Animals & Nature (dog, cat, tree, flower)
- Food & Drink (pizza, coffee, beer)
- Activities & Sports (soccer, music, art)
- Travel & Places (plane, car, hotel)
- Objects (phone, computer, book)
- Symbols (check, warning, arrows)
- Flags (country flags)

## Contributing

Found a bug or want to add a feature? Please open an issue or pull request on the [Almoji GitHub repository](https://github.com/shaond/almoji).

## License

This workflow is provided as-is. The Almoji CLI is licensed under the MIT License.

## Credits

- **Almoji CLI**: Fast emoji search engine
- **Alfred**: Productivity app for macOS by Running with Crayons Ltd
- **Emoji Characters**: Unicode Standard

---

**Note**: Make sure to add an `icon.png` file (256x256 pixels) to make the workflow look polished in Alfred!
