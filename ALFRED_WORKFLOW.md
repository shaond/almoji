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

   Option A - Import directly (easiest):
   ```bash
   # Open the workflow directory in Finder
   open Almoji.alfredworkflow
   ```
   Then double-click the `info.plist` file or import the folder via Alfred preferences.

   Option B - Manual installation:
   ```bash
   # Copy to Alfred's workflow directory
   cp -r Almoji.alfredworkflow ~/Library/Application\ Support/Alfred/Alfred.alfredpreferences/workflows/user.workflow.$(uuidgen)
   ```

3. **Add an Icon (Optional but Recommended)**:
   The workflow includes an `icon.png.txt` file with instructions. To add a proper icon:
   - Create or download a 256x256 PNG image
   - Save it as `icon.png` in the `Almoji.alfredworkflow/` directory
   - Recommended: Use an emoji screenshot (😊 or 🔍)

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

- `info.plist` - Workflow configuration and script
- `README.md` - Detailed documentation
- `icon.png.txt` - Instructions for adding an icon
- `icon.png` - (You need to add this - see instructions)

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

To share this workflow with others, create a `.alfredworkflow` package:

```bash
cd Almoji.alfredworkflow
zip -r ../Almoji.alfredworkflow.zip .
mv ../Almoji.alfredworkflow.zip ../Almoji.alfredworkflow
```

Users can then double-click the `.alfredworkflow` file to install.

## Requirements

- macOS
- Alfred Powerpack (paid version)
- Almoji CLI installed and in PATH
- Rust/Cargo (for building Almoji)

## See Also

- [Almoji README](README.md) - Main Almoji CLI documentation
- [Alfred Workflows Documentation](https://www.alfredapp.com/help/workflows/)
- [Alfred Script Filters](https://www.alfredapp.com/help/workflows/inputs/script-filter/)
