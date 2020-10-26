#!/usr/bin/env node

import path from "path";
import { fileURLToPath } from "url";
import { run, meta } from "./binary.js";

console.log("WindOWSSS", process.env.TOAST_BINARY);

if (!process.env.TOAST_MODULE_PATH) {
  process.env.TOAST_MODULE_PATH = path.dirname(
    path.dirname(fileURLToPath(import.meta.url))
  );
}

if (process.env.TOAST_BINARY) {
  console.log(
    `Running ${meta.name} with overridden binary path: ${process.env.TOAST_BINARY}`
  );
  run({ binaryPath: process.env.TOAST_BINARY });
} else {
  run({});
}
