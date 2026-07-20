/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        civic: {
          50: '#eef8f5',
          100: '#d9eee8',
          500: '#2b7c70',
          600: '#24695f',
          700: '#1d554d',
          900: '#123a36',
        },
        ink: {
          50: '#f6f8fb',
          100: '#e8edf3',
          500: '#5f6f86',
          700: '#334155',
          800: '#223044',
          900: '#111827',
        },
        service: {
          blue: '#2563eb',
          gold: '#b7791f',
          red: '#b42318',
        },
        surface: {
          DEFAULT: '#f7fafc',
          raised: '#ffffff',
          muted: '#edf4f2',
        },
      },
      fontFamily: {
        sans: [
          'Inter',
          'ui-sans-serif',
          'system-ui',
          '-apple-system',
          'BlinkMacSystemFont',
          'Segoe UI',
          'sans-serif',
        ],
      },
      boxShadow: {
        focus: '0 0 0 3px rgba(43, 124, 112, 0.22)',
      },
      borderRadius: {
        control: '6px',
      },
    },
  },
  plugins: [],
};
