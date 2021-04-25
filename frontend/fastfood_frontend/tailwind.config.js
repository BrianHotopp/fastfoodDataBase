module.exports = {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    fontFamily: {
      'buttonfont': ['"Montserrat"','sans-serif'],
    },
    fontWeight: {
      thin: 100,
       light: 300,
       normal: 400,
       medium: 500,
       semibold: 600,
       bold: 700,
       black: 900,
    },
    fontSize: {
      'xs': '.75rem',
      'sm': '.875rem',
      'tiny': '.875rem',
      'base': '1rem',
      'lg': '1.125rem',
      'xl': '1.25rem',
      '2xl': '1.5rem',
      '3xl': '1.875rem',
      '4xl': '2.25rem',
      '5xl': '3rem',
      '6xl': '4rem',
      '7xl': '5rem',
    },
    colors: {
      'black': '#000000',
      'bgcolor': '#051E38',
      'bgcolor+1': '#072D56',
      'white': '#FFFFFF',
      'myorange': '#FF9900',
      'myorange+1': '#ED8E00',
      'myorange+2': '#FFA824',
      'myorange+3': '#FFBD5B',
      'myorange+4': '#FFB649',
      'myyellow': '#ffda77',
      'mywhite': '#fbf6f0',
      'myblue': '#aee6e6',
    },
        extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
  options: {
    whitelistPatterns: [ 
  /-(leave|enter|appear)(|-(to|from|active))$/, 
  /^(?!(|.*?:)cursor-move).+-move$/,
      /^router-link(|-exact)-active$/
    ],
  }
}
