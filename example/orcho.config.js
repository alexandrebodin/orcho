const { defineConfig } = require("../../../npm/orcho");

module.exports = defineConfig({
  root: true,
  tasks: {
    build: {
      command: "echo 'Building package A'",
      // input: [],
      // output: [],
      // dependsOn: [],
    },
  },
});
