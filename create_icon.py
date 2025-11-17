#!/usr/bin/env python3
"""Create an icon for the Almoji Alfred Workflow."""

from PIL import Image, ImageDraw, ImageFont

# Create a 256x256 image with a gradient background
size = 256
img = Image.new('RGB', (size, size), color='white')
draw = ImageDraw.Draw(img)

# Create a circular gradient background
for i in range(size):
    for j in range(size):
        # Calculate distance from center
        dx = i - size/2
        dy = j - size/2
        distance = (dx*dx + dy*dy) ** 0.5
        max_distance = size / 2

        # Create gradient from purple to blue
        if distance < max_distance:
            ratio = distance / max_distance
            r = int(138 + (100 - 138) * ratio)  # Purple to blue
            g = int(43 + (149 - 43) * ratio)
            b = int(226 + (237 - 226) * ratio)
            img.putpixel((i, j), (r, g, b))

# Draw a circle border
circle_bbox = [20, 20, size-20, size-20]
draw.ellipse(circle_bbox, outline='white', width=8)

# Try to use a nice font for the emoji, or fall back to default
try:
    # Try to load a system font that supports emojis
    font_size = 120
    font = ImageFont.truetype("/System/Library/Fonts/Apple Color Emoji.ttc", font_size)
except:
    try:
        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 100)
    except:
        font = ImageFont.load_default()

# Draw emoji in the center (use a simple smiley as fallback)
emoji_text = "😊"
try:
    # Get text size
    bbox = draw.textbbox((0, 0), emoji_text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]

    # Center the text
    x = (size - text_width) / 2 - bbox[0]
    y = (size - text_height) / 2 - bbox[1]

    draw.text((x, y), emoji_text, font=font, embedded_color=True)
except:
    # Fallback: draw a simple smiley face
    # Face circle
    face_bbox = [40, 40, size-40, size-40]
    draw.ellipse(face_bbox, fill='#FFD700', outline='white', width=4)

    # Eyes
    eye_size = 20
    left_eye_bbox = [80, 90, 80+eye_size, 90+eye_size]
    right_eye_bbox = [156, 90, 156+eye_size, 90+eye_size]
    draw.ellipse(left_eye_bbox, fill='black')
    draw.ellipse(right_eye_bbox, fill='black')

    # Smile
    smile_bbox = [70, 70, size-70, size-70]
    draw.arc(smile_bbox, start=0, end=180, fill='black', width=8)

# Add a small magnifying glass in corner to indicate "search"
glass_x, glass_y = 185, 185
glass_size = 40
draw.ellipse([glass_x, glass_y, glass_x+glass_size, glass_y+glass_size],
             outline='white', width=6)
draw.line([glass_x+glass_size-8, glass_y+glass_size-8,
           glass_x+glass_size+10, glass_y+glass_size+10],
          fill='white', width=6)

# Save the image
img.save('/home/user/almoji/Almoji.alfredworkflow/icon.png', 'PNG')
print("Icon created successfully!")
