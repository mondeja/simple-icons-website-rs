import autoprefixer from 'autoprefixer';
import tailwindcss from 'tailwindcss';
import postcssImport from 'postcss-import';

/** @type {import('postcss-load-config').Config} */
export default {
  plugins: [autoprefixer, tailwindcss, postcssImport],
};
