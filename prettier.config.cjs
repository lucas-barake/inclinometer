/** @type {import("prettier").Config} */
const config = {
  plugins: [
    require.resolve("prettier-plugin-toml"),
    require.resolve("prettier-plugin-rust"),
  ],
};

module.exports = config;
