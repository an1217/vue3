module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
        container: {
            padding: {
              DEFAULT: '15px',
            },
          },
    },
    variants: {},
    plugins: [],
};