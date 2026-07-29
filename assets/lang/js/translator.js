// translate.js
// A high-performance translation bridge for a custom macOS Shell written in Rust.
// Handles stdin input, communicates with the Google Translation API, and outputs JSON.

const https = require('https');

// --- CONFIGURATION ---
// Safely load the API key from environment variables passed by the Rust binary.
const API_KEY = process.env.GOOGLE_TRANSLATE_API_KEY;
const TARGET_LANG = process.env.SHELL_TARGET_LANG || 'en';

/**
 * Validates initialization parameters before execution.
 */
function validateEnvironment() {
    if (!API_KEY) {
        console.error(JSON.stringify({
            error: "Missing API Key",
            message: "Please set the GOOGLE_TRANSLATE_API_KEY environment variable in your Rust shell environment."
        }));
        process.exit(1);
    }
}

/**
 * Communicates with the Google Cloud Translation v2 API.
 * @param {string} textToTranslate - The raw string sent from the shell.
 */
function translateText(textToTranslate) {
    if (!textToTranslate.trim()) {
        console.log(JSON.stringify({ translatedText: "" }));
        process.exit(0);
    }

    // Google Translation v2 URL payload structure
    const postData = JSON.stringify({
        q: textToTranslate,
        target: TARGET_LANG,
        format: 'text' 
    });

    const options = {
        hostname: '://googleapis.com',
        port: 443,
        path: `/language/translate/v2?key=${API_KEY}`,
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Content-Length': Buffer.byteLength(postData)
        }
    };

    const req = https.request(options, (res) => {
        let body = '';

        res.on('data', (chunk) => body += chunk);

        res.on('end', () => {
            try {
                const responseData = JSON.parse(body);

                if (res.statusCode !== 200) {
                    throw new Error(responseData.error?.message || `HTTP Error ${res.statusCode}`);
                }

                // Extract string data structurally matching Google API schema
                const translatedText = responseData.data.translations[0].translatedText;
                
                // Return clean structural JSON data back to Rust stdout
                console.log(JSON.stringify({
                    success: true,
                    translatedText: translatedText,
                    detectedSourceLanguage: responseData.data.translations[0].detectedSourceLanguage || null
                }));
                process.exit(0);

            } catch (err) {
                console.error(JSON.stringify({ success: false, error: err.message }));
                process.exit(1);
            }
        });
    });

    req.on('error', (err) => {
        console.error(JSON.stringify({ success: false, error: `Network error: ${err.message}` }));
        process.exit(1);
    });

    req.write(postData);
    req.end();
}

/**
 * Standard Stream consumer capturing stdin pipeline data injected by Rust.
 */
function main() {
    validateEnvironment();

    let inputBuffer = '';
    
    process.stdin.on('data', (chunk) => {
        inputBuffer += chunk;
    });

    process.stdin.on('end', () => {
        translateText(inputBuffer.trim());
    });
}

main();
