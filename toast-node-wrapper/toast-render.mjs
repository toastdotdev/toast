import path from "path";
import { existsSync, promises as fs } from "fs";
import "./src/module-aliases.mjs";
import { render } from "./src/page-renderer-pre.mjs";

// loader doesn't show up in argv
const [_node, _binStr, srcDir, outputDir, ...args] = process.argv;

main();

async function main() {
  // require pageWrapper
  let pageWrapper;
  const pageWrapperPath = path.resolve(srcDir, "src/page-wrapper.js");
  try {
    const wrapper = await import(pageWrapperPath);
    pageWrapper = wrapper.default;
  } catch (e) {
    console.log("no user pagewrapper supplied");
  }

  // render html
  return Promise.all(
    args.map(async (file) => {
      const nodeComponent = await import(path.resolve(srcDir, file));
      let data;
      try {
        data = await fs.readFile(
          `${path.resolve(outputDir, file.replace("src/pages/", ""))}on`
        );
        data = JSON.parse(data);
      } catch (e) {
        // TODO: figure out what errors are important here
      }
      return render({
        component: nodeComponent.default,
        pageWrapper,
        data,
        browserPageWrapperPath: "/src/page-wrapper.js",
        browserComponentPath: path.resolve("/", file),
        // .js(on)
        browserDataPath: path.resolve(
          "/",
          `${file.replace("src/pages/", "")}on`
        ),
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
