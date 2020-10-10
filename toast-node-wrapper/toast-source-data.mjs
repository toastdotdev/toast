import got from "got";

// --loader doesn't show up in argv
const [_node, _binPath, socketPath, toastFilePath, ...args] = process.argv;

main();

async function main() {
  let toast = await import(toastFilePath);
  const res = await got(`http://unix:${socketPath}:/`);
  if (res.body === "ready" && toast.sourceData) {
    try {
      await toast.sourceData({ setDataForSlug });
    } catch (e) {
      console.error(e);
    }
  } else if (res.body !== "ready") {
    throw new Error("Unable to get ready to run toast.sourceData");
  }
}

// pageArgs is `{module: JSModuleAsString, slug: String, data: {}}`
const setDataForSlug = async (pageArgs) => {
  let args = pagesArgs;
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
  await got.post(`http://unix:${socketPath}:/set-data-for-slug`, {
    json: pageArgs,
  });
  return { ok: true };
};
