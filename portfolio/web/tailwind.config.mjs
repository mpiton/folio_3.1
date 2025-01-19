/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: {
			colors: {
				'primary': {
					DEFAULT: '#578E7E',
					dark: '#355E57'
				},
				'secondary': '#F5ECD5',
				'accent': '#FFFAEC',
				'text': {
					DEFAULT: '#3D3D3D',
					light: '#666666'
				},
				'bg': {
					DEFAULT: '#FFFFFF',
					dark: '#13151A'
				}
			},
			fontFamily: {
				'heading': ['Poppins', 'sans-serif'],
				'body': ['Open Sans', 'sans-serif']
			}
		},
	},
}
