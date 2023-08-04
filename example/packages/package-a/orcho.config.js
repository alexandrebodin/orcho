const { defineConfig } = require("../../../npm/orcho");

module.exports = defineConfig({
  tasks: {
    build: {
      script: 'echo "Building package A"',
    },
  },
});
