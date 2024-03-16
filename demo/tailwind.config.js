/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sw: ['SequelWide90', 'sans-serif'],
        sl: ['SequelBlack45', 'sans-serif'],
        grotesk: ['Grotesk', 'sans-serif']
      },
    },
  },
  plugins: [],
}

