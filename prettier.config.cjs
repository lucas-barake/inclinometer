/** @type {import("prettier").Config} */
const config = {
  plugins: [require.resolve("prettier-plugin-toml")],
};

module.exports = config;
