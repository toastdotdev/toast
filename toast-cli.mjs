#!/usr/bin/env -S NODE_OPTIONS='--experimental-loader="toast/src/loader.mjs"' node
import {
  incremental,
  setDataForSlug,
  doneSourcingData,
} from "@toastdotdev/lib";
import cac from "cac";
import path from "path";
import { performance } from "perf_hooks";
import { renderAll } from "./src/render.mjs";

const PERF = !!process.env.DEBUG_PERF;
const cli = cac();

const startTime = performance.now();
PERF && performance.mark("start");

cli
  .command("incremental <input-dir> [output-dir]", "Build a site")
  .option("-v, --verbose", "Be verbose")
  .action(async (input, output, options) => {
    PERF && performance.mark("start-action");

    const inputPath = path.resolve(input);
    const outputPath = path.resolve(output || "public");
    const esinstallPath = path.resolve(
      outputPath,
      "web_modules",
      "import-map.json"
    );
    // start incremental so that it can process setDataForSlug
    let things = incremental(inputPath, outputPath);

    PERF && performance.mark("importing-toast");

    let toast;
    try {
      toast = await import(path.resolve(inputPath, "toast.js"));
      PERF && performance.mark("done-importing-toast");
    } catch (e) {
      if (e.code === "ERR_MODULE_NOT_FOUND") {
        console.log("no toast.js found, skipping");
      } else {
        console.warn(e);
      }
    }

    if (!!toast) {
      await toast.sourceData({ setDataForSlug: userSetDataForSlug });
      PERF && performance.mark("done-source-data");
    }

    doneSourcingData();
    let urls = await things;
    PERF && performance.mark("awaited-things");
    // setDataForSlug;
    console.log({ urls });
    await renderAll(inputPath, outputPath, urls);
    console.log(`Toast ran in: ${Math.floor(performance.now() - startTime)}ms`);
    PERF &&
      console.log(
        performance.measure("to start action", "start", "start-action")
      );
    PERF &&
      console.log(performance.measure("2", "start-action", "importing-toast"));
    PERF &&
      console.log(
        performance.measure(
          "await import(toast.js)",
          "importing-toast",
          "done-importing-toast"
        )
      );
    PERF &&
      console.log(
        performance.measure("4", "done-importing-toast", "done-source-data")
      );
    PERF &&
      console.log(
        performance.measure("4", "done-source-data", "awaited-things")
      );
  });

cli.help();

cli.parse();

const userSetDataForSlug = async (slug, pageArgs) => {
  // the slug is the key for everything, it can not be undefined or falsey
  if (!slug) {
    throw new Error(
      `setDataForSlug requires a slug as the first argument. second argument is ${JSON.stringify(
        pageArgs
      )}`
    );
  }
  // This `if` is to protect against this faulty input:
  //
  // ```js
  // setDataForSlug('/', {
  //  component: ""
  // })
  // ```
  //
  if (typeof pageArgs.component === "string") {
    throw new Error(`The \`component\` passed to \`setDataForSlug\` was passed as a string for slug \`${slug}\`.
  It should be an object with a mode of "filepath" or "source":
  \`\`\`
  const page = {
    component: {
      mode: "source",
      value: \`import { h } from "preact";
  export default props => <div>
    <h1>Some Code</h1>
  </div>\`
    }
  }
  \`\`\`
  or
  \`\`\`
  const page = {
    component: {
      mode: "filepath",
      value: "src/pages/index.js"
    }
  }
  \`\`\`
  `);
  }

  // This if is to protect against `mode` being at the top level,
  // outside of `component` or `wrapper`
  //
  // ```js
  // setDataForSlug('/', {
  //   mode: "filepath",
  //   data: {}
  // })
  // ```
  //
  if (pageArgs.mode) {
    throw new Error(`\`mode\` was passed as a top-level key to \`setDataForSlug\` for slug '${slug}'.
  Did you mean to put it in \`component\` or \`wrapper\` object?
  ## sample unexpected input:
  \`\`\`
  const page = {
    mode: "filepath",
    data: {},
  }
  \`\`\`
  ## sample successful input:
  \`\`\`
  const page = {
    component: {
      mode: "filepath",
      value: "src/pages/index.js"
    },
    data: {}
  }
  \`\`\`
  `);
  }

  let args = pageArgs;
  if (args.component === null) {
    args.component = {
      mode: "no-module",
    };
  }
  if (args.wrapper === null) {
    args.wrapper = {
      mode: "no-module",
    };
  }
  try {
    await setDataForSlug(JSON.stringify({ slug, ...pageArgs }));
  } catch (error) {}
  return { ok: true };
};
