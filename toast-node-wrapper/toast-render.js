#!/usr/bin/env node

const path = require("path");
const { existsSync, promises: fs } = require("fs");
require("./src/module-aliases");
const { render } = require("./src/page-renderer-pre");

const [node, bin, srcDir, outputDir, ...args] = process.argv;

main();

async function main() {
  // require pageWrapper
  let pageWrapper;
  const pageWrapperPath = path.resolve(srcDir, "src/page-wrapper");
  try {
    pageWrapper = require(pageWrapperPath).default;
  } catch (e) {
    // console.log("no user pagewrapper supplied");
  }

  // TODO: no data for now
  const data = {};

  // render html
  return Promise.all(
    args.map(async (file) => {
      const nodeComponent = await import(
        path.resolve(srcDir, file).replace(".js", ".mjs")
      );
      //   console.log(nodeComponent);
      return render({
        component: nodeComponent.default,
        pageWrapper,
        data,
        browserPageWrapperPath: "/src/page-wrapper.js",
        browserComponentPath: path.resolve("/", file),
        // .js(on)
        browserDataPath: path.resolve("/", `${file}on`),
      }).then((html) => {
        // write HTML file out for page
        const htmlFilePath = path.resolve(
          outputDir,
          file.replace("src/pages/", "").replace(".js", ".html")
        );
        return fs.writeFile(htmlFilePath, html);
      });
    })
  );
}
