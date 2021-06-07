const colors = require('tailwindcss/colors')
const path = require('path');
const distPath = path.resolve(__dirname, 'src');

module.exports = {
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    borderColor: theme => ({
      ...theme('colors'),
      'rust': "#e56034"
    }),
    minWidth: {
      'project-card-mobile': "320px",
      'project-card': "420px"
    },
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
  plugins: [
  ],
  corePlugins: {
   backgroundImage: true,
  }
}
