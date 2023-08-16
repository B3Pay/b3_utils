module.exports = {
  content: ["./frontend/public/index.html", "./frontend/src/**/*.svelte"],
  theme: {
    extend: {}
  },
  plugins: [require("daisyui"), require("@tailwindcss/line-clamp")]
}
