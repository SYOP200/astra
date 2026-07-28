/*
    Astra Shell Translation Manager
    Handles language switching and API-based translations.

    Configure:
    - API_KEY
    - API_URL
*/

const AstraTranslator = {
    API_KEY: "YOUR_API_KEY_HERE",
    API_URL: "YOUR_TRANSLATE_API_ENDPOINT_HERE",

    language: "en",

    cache: {},

    async init() {
        // Detect system language
        const systemLanguage = navigator.language || "en";

        this.language = systemLanguage.split("-")[0];

        console.log(
            "Astra Translator initialized:",
            this.language
        );
    },

    setLanguage(lang) {
        this.language = lang;
        localStorage.setItem(
            "astra-language",
            lang
        );
    },

    getLanguage() {
        return (
            localStorage.getItem("astra-language") ||
            this.language
        );
    },

    async translate(text) {
        const lang = this.getLanguage();

        // No translation needed
        if (lang === "en") {
            return text;
        }

        const cacheKey = `${lang}:${text}`;

        if (this.cache[cacheKey]) {
            return this.cache[cacheKey];
        }

        try {
            const response = await fetch(
                this.API_URL,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                        "Authorization":
                            `Bearer ${this.API_KEY}`
                    },
                    body: JSON.stringify({
                        text: text,
                        target_language: lang
                    })
                }
            );

            const data = await response.json();

            const translated =
                data.translation ||
                text;

            this.cache[cacheKey] =
                translated;

            return translated;

        } catch (error) {
            console.error(
                "Translation failed:",
                error
            );

            return text;
        }
    }
};


// Global helper
async function translateText(text) {
    return await AstraTranslator.translate(text);
}


// Start translator
AstraTranslator.init();
