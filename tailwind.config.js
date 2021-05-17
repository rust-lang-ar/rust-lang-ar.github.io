const colors = require('tailwindcss/colors')
const path = require('path');
const distPath = path.resolve(__dirname, 'src');

module.exports = {
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      colors:{
        header: "#E8C64E"
      },
      fontFamily:{
        playfair: ["Playfair Display"],
        lato: ["Lato"],
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
  corePlugins: {
   backgroundImage: true,
  }
}
