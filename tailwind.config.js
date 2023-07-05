/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./site/**/*.{html,js}"],
  darkMode: "class",
  theme: {
    screens: {
      sm: { max: "640px" },
      md: "768px",
      lg: "1024px",
      xl: "1280px",
      "2xl": "1536px",
    },
    extend: {
      colors: {
        brand: "#31363b",
      },
      fontSize: {
        xl: "1.5rem",
        "2xl": "2rem",
        "3xl": "3.5rem",
        "4xl": "7rem",
      },
    },
  },
  plugins: [],
};
