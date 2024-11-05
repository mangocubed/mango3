/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: [
            "*.html",
            "./mango3-accounts/src/**/*.rs",
            "./mango3-home/src/**/*.rs",
            "./mango3-leptos-utils/src/**/*.rs",
        ],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
        },
    },
    darkMode: ["selector", '[data-theme="dark"]'],
    plugins: [require("daisyui")],
    theme: {
        extend: {},
    },
};
