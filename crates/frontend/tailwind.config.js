/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        montserrat: ["'Montserrat'", "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [],
}
