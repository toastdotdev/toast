import core from "@actions/core";
import { exec } from "child_process";
import { promises as fs } from "fs";
import { join } from "path";

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
