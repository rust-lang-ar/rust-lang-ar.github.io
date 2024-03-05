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
    extend: {
      screens: {
        'xsm': '460px',
      },
      colors:{
        header: "#E8C64E"
      },
      fontFamily:{
        heading: ['Playfair Display', 'sans-serif'],
        body: ['Lato', 'sans-serif'],
      },
    },
  },
  variants: {
    extend: {
      textColor: ['visited'],
    },
  },
  plugins: [
  ],
  corePlugins: {
   backgroundImage: true,
  }
}
