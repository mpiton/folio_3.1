/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: {
			colors: {
				primary: '#578E7E',
				'primary-dark': '#355E57',
				secondary: '#F5ECD5',
				accent: '#FFFAEC',
				'text-main': '#3D3D3D',
				'text-light': '#666666',
			},
			fontFamily: {
				heading: ['Poppins', 'sans-serif'],
				body: ['"Open Sans"', 'sans-serif'],
			}
		},
	},
	plugins: [],
}
