module.exports = {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    colors: {
      'black': '#000000',
      'white': '#FFFFFF',
      'myorange': '#ffa45b',
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
