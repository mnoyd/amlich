/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        parchment: {
          DEFAULT: '#fcfbf9',
          dark: '#f0efe9',
        },
        ink: {
          DEFAULT: '#1a1a1a',
          light: '#333333',
          border: '#e0dfd8',
        },
        ky: '#d93838',        // Red/Avoid (Kỵ/Kỵ Mạnh)
        hoangdao: '#d4af37',  // Gold/Neutral/Royal (Hoàng Đạo)
        nen: '#2d8a56',       // Jade/Recommend (Nên)
        tranh: '#d97706',     // Amber/Warning (Tránh)
        cothe: '#475569',     // Slate/Secondary (Có thể)
        evidence: '#3b82f6',  // Blue/Computed Evidence
      },
      fontFamily: {
        mono: ['Courier New', 'Courier', 'monospace'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
      }
    },
  },
  plugins: [],
}
