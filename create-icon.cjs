const pngToIco = require('png-to-ico').default || require('png-to-ico');
const fs = require('fs');
const path = require('path');

const inputFile = path.join('src-tauri', 'icons', 'icon.png');
const outputFile = path.join('src-tauri', 'icons', 'icon.ico');

console.log(`Converting ${inputFile} to ${outputFile}...`);

pngToIco(inputFile)
    .then(buf => {
        fs.writeFileSync(outputFile, buf);
        console.log('Success: icon.ico created!');
    })
    .catch(err => {
        console.error('Error creating icon:', err);
        process.exit(1);
    });
