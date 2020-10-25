#!/usr/bin/env node

import { run, meta } from "./binary.js";

if (process.env.TOAST_BINARY) {
  console.log(
    `Running ${meta.name} with overridden binary path: ${process.env.TOAST_BINARY}`
  );
  run({ binaryPath: process.env.TOAST_BINARY });
} else {
  run({});
}
