import got from "got";

// --loader doesn't show up in argv
const [_node, _binPath, socketPath, toastFilePath, ...args] = process.argv;

main();

async function main() {
  let toast = await import(toastFilePath);
  const res = await got(`http://unix:${socketPath}:/`);
  if (res.body === "ready" && toast.sourceData) {
    try {
      await toast.sourceData({ createPage, setData });
    } catch (e) {
      console.error(e);
    }
  } else if (res.body !== "ready") {
    throw new Error("Unable to get ready to run toast.sourceData");
  }
}

// pageArgs is `{module: JSModuleAsString, slug: String, data: {}}`
const createPage = async (pageArgs) => {
  await got.post(`http://unix:${socketPath}:/create-page`, {
    json: pageArgs,
  });
  return { ok: true };
};

// pageArgs is `{slug: String, data: {}}`
const setData = async (pageArgs) => {
  await got.post(`http://unix:${socketPath}:/set-data`, {
    json: pageArgs,
  });
  return { ok: true };
};
