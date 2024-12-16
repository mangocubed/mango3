/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: [
            "*.html",
            "./mango3-accounts/src/**/*.rs",
            "./mango3-home/src/**/*.rs",
            "./mango3-leptos-utils/src/**/*.rs",
            "./mango3-my-account/src/**/*.rs",
            "./mango3-studio/src/**/*.rs",
            "./mango3-websites/src/**/*.rs",
        ],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
        },
    },
    daisyui: {
        themes: [
            "light",
            "dark",
            "cupcake",
            "corporate",
            "bumblebee",
            "emerald",
            "synthwave",
            "retro",
            "cyberpunk",
            "valentine",
            "halloween",
            "garden",
            "forest",
            "aqua",
            "lofi",
            "pastel",
            "fantasy",
            "wireframe",
            "black",
            "luxury",
            "dracula",
            "cmyk",
            "autumn",
            "business",
            "acid",
            "lemonade",
            "night",
            "coffee",
            "winter",
            "dim",
            "nord",
            "sunset",
        ],
    },
    darkMode: ["selector", '[data-theme="dark"]'],
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    theme: {
        extend: {},
    },
};
