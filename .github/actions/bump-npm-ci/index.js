const core = require("@actions/core");
const { exec } = require("child_process");
const { promises: fs } = require("fs");
const { join } = require("path");

async function run() {
  console.log(core.getInput("binaryHash"));
  console.log(core.getInput("toastVersion"));
  try {
    const package = await fs.readFile(
      process.env.GITHUB_WORKSPACE + "toast-node-wrapper/package.json"
    );
    const json = JSON.parse(package);
    console.log(json);
    exec(`npm show toast version`);
  } catch (error) {
    console.log(error.message);
  }
}

run();
