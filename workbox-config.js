module.exports = {
	globDirectory: 'dist/',
	globPatterns: [
		'**/*.{png,wasm,js,html}'
	],
	swDest: 'dist/sw.js',
	ignoreURLParametersMatching: [
		/^utm_/,
		/^fbclid$/
	]
};