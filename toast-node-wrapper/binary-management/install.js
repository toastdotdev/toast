#!/usr/bin/env node

import { installFromUrl } from "./binary.js";

if (process.env.TOAST_PREVENT_INSTALL) {
  console.log(
    "Toast binary install from url prevented by env var TOAST_PREVENT_INSTALL"
  );
} else {
  installFromUrl({});
}
