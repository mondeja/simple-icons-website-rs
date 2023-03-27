/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["index.html", "../{app,components}/src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
