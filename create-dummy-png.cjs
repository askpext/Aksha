const fs = require('fs');
const buffer = Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKwITQAAAABJRU5ErkJggg==', 'base64');
fs.writeFileSync('src-tauri/icons/icon.png', buffer);
console.log('Created dummy icon.png');
