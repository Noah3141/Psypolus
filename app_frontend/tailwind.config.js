/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{html,rs}", "./index.html"], // Look here for Tailwind to unpack!
    theme: {
        extend: {},
    },
    plugins: [],
};

// npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
