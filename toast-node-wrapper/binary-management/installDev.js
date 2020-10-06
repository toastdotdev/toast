#!/usr/bin/env node

import { setLocalBinaryPath, install } from "./binary.js";

setLocalBinaryPath(process.argv[1]);
install();
