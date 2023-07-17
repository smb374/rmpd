import { variants } from "@catppuccin/palette";

const mocha = variants.mocha;

/** @type {import('tailwindcss').Config}*/
const config = {
    content: ["./index.html", "./src/**/*.{html,js,svelte,ts}"],
    theme: {
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("daisyui"),
        require("@catppuccin/tailwindcss"),
    ],
    darkMode: "class",
    daisyui: {
        themes: [
            {
                "catppuccin-mocha": {
                    primary: mocha.lavender.hex,
                    secondary: mocha.pink.hex,
                    accent: mocha.teal.hex,
                    neutral: mocha.crust.hex,
                    "base-100": mocha.base.hex,
                    info: mocha.blue.hex,
                    success: mocha.green.hex,
                    warning: mocha.yellow.hex,
                    error: mocha.red.hex,
                },
            },
        ],
    },
};

export default config;
