# Almoji Alfred Workflow

This repository includes an Alfred workflow for searching emojis using Almoji.

## Quick Start

The Alfred workflow is located in the `Almoji.alfredworkflow/` directory.

### Installation Steps

1. **Install Almoji CLI** (if not already installed):
   ```bash
   cargo build --release
   sudo cp target/release/almoji /usr/local/bin/
   ```

2. **Install the Workflow in Alfred**:

   **Option A - Double-click to install (recommended)**:
   - Download `Almoji-Workflow.alfredworkflow`
   - Double-click the file
   - Alfred will open and prompt you to import the workflow
   - Click "Import" to add it to Alfred

   **Option B - Manual installation**:
   ```bash
   # Open the workflow file to install
   open Almoji-Workflow.alfredworkflow
   ```

   The workflow includes:
   - ✅ Complete info.plist configuration
   - ✅ Custom 256x256 PNG icon (emoji search themed)
   - ✅ Comprehensive documentation

## Usage

1. Open Alfred (`⌘ + Space`)
2. Type `;` followed by your search term
3. Select an emoji from the results
4. Press `Enter` to copy it to clipboard

### Examples

```
; fire        → 🔥
; heart       → ❤️ 💕 💗
; happy       → 😊 😀 😂
; thumbsup    → 👍
```

## Configuration

Right-click the workflow in Alfred Preferences and select "Configure Workflow..." to set:

- **Skin Tone**: `light`, `medium-light`, `medium`, `medium-dark`, `dark`
- **Gender**: `male`, `female`, `neutral`

Leave empty to use default emojis.

## Workflow Features

- **Keyword**: `;` (customizable)
- **Search Limit**: 20 results (customizable)
- **Copy to Clipboard**: Automatically copies selected emoji
- **Fast Search**: Uses Almoji's O(1) perfect hash map lookups
- **500+ Emojis**: Comprehensive emoji database

## Files

**Workflow Package**: `Almoji-Workflow.alfredworkflow` (ready to install)

**Source Files** (in `Almoji.alfredworkflow/` directory):
- `info.plist` - Workflow configuration and script
- `README.md` - Detailed documentation
- `icon.png` - Custom 256x256 emoji search icon

## Customization

### Change the Keyword Trigger

Edit `info.plist` and find the `<key>keyword</key>` section. Change `;` to your preferred trigger.

### Adjust Search Results Limit

In `info.plist`, find `--limit 20` in the script and change to your desired number.

### Modify Skin Tone/Gender

Use the Alfred workflow configuration UI, or edit the script in `info.plist` to hard-code values.

## Technical Details

The workflow uses:
- **Alfred Script Filter**: For dynamic search results
- **Bash Script**: Calls Almoji CLI and formats JSON output
- **Clipboard Output**: Copies emoji when selected
- **Configuration Variables**: For user-customizable skin tone and gender

## Troubleshooting

**Issue**: "No emojis found" for all searches
- **Fix**: Make sure `almoji` is in your PATH (`which almoji`)

**Issue**: Configuration not working
- **Fix**: Check spelling - use lowercase values like `medium-light`, not `Medium Light`

**Issue**: Workflow not appearing
- **Fix**: Make sure you have Alfred Powerpack (required for workflows)

## Distribution

The workflow is distributed as `Almoji-Workflow.alfredworkflow`, which is ready to install.

To rebuild the workflow package:

```bash
cd Almoji.alfredworkflow
zip -r ../Almoji-Workflow.alfredworkflow . -x "*.DS_Store"
```

Users can then double-click the `.alfredworkflow` file to install.

## Included Icon

The workflow includes a custom 256x256 PNG icon featuring:
- Gradient purple-to-blue circular background
- Emoji/smiley face design
- Magnifying glass overlay (indicating "search")
- Professional appearance in Alfred's UI

## Requirements

- macOS
- Alfred Powerpack (paid version)
- Almoji CLI installed and in PATH
- Rust/Cargo (for building Almoji)

## See Also

- [Almoji README](README.md) - Main Almoji CLI documentation
- [Alfred Workflows Documentation](https://www.alfredapp.com/help/workflows/)
- [Alfred Script Filters](https://www.alfredapp.com/help/workflows/inputs/script-filter/)
