/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution');

module.exports = {
	root: true,
	extends: [
		'eslint:recommended',
		'plugin:vue/vue3-essential',
		'@vue/eslint-config-typescript',
		'@vue/eslint-config-prettier/skip-formatting',
	],
	parserOptions: {
		ecmaVersion: 'latest',
	},
	rules: {
		'no-var': 'error',
		'prefer-const': 'error',
		'no-unused-vars': 'error',
		'no-unused-expressions': 'error',
		indent: [1, 'tab', { SwitchCase: 1 }],
		'@typescript-eslint/no-unused-vars': 1,
		'@typescript-eslint/consistent-type-imports': 'error',
		'@typescript-eslint/explicit-function-return-type': 'error',
		'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
		'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
	},
	ignorePatterns: ['node_modules/', 'dist/', 'coverage/', 'src/**/*.spec.ts'],
};
