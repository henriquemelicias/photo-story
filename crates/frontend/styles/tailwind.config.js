/** @type {import("tailwindcss").Config} */
module.exports = {

    content: ["./index.html", "./styles/tailwind.css", "./src/**/*.rs"],
    theme: {
        extend: {}
    },
    plugins: [require("daisyui")],
    screens: {
        sm: "576px",
        md: [{min: "668px", max: "767px"}, {min: "868px"}],
        lg: {min: "992px", max: "1199px"},
        xl: {min: "1200px"},
        xxl: {max: "1920px"},
        print: {raw: "print"},
        dark: {raw: "(prefers-color-scheme: dark)"}
    }
};
