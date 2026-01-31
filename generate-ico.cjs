const png2icons = require('png2icons');
const fs = require('fs');

const input = fs.readFileSync('src-tauri/icons/icon.png');
// Create ICO with bilinear interpolation (high quality)
const output = png2icons.createICO(input, png2icons.BILINEAR, 0, false);

if (output) {
    fs.writeFileSync('src-tauri/icons/icon.ico', output);
    console.log('Success: icon.ico created!');
} else {
    console.error('Failed to create icon');
    process.exit(1);
}
