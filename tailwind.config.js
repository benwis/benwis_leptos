/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: {
        files: ["*.html", "./app/src/**/*.rs"],
    },
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/typography'),
    ],
}
