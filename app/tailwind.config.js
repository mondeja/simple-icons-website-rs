const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["index.html", "../{app,components}/src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        mono: ["Roboto Mono", ...defaultTheme.fontFamily.mono],
      },
    },
  },
  plugins: [],
};
