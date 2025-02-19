/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: [
            "*.html",
            "./mango3-accounts/src/**/*.rs",
            "./mango3-admin/src/**/*.rs",
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
    safeList: ["opacity-50"],
    theme: {
        extend: {},
    },
    plugins: [],
};
