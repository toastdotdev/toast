#!/usr/bin/env node --loader toastrs/src/loader.mjs

import got from "got";

const [_node, _bin, socketPath, toastFilePath, ...args] = process.argv;

main();

async function main() {
  let toast = await import(toastFilePath);
  const res = await got(`http://unix:${socketPath}:/`);
  if (res.body === "ready" && toast.sourceData) {
    try {
      await toast.sourceData({ createPage });
    } catch (e) {
      console.error(e);
    }
  } else if (res.body !== "ready") {
    throw new Error("Unable to get ready to run toast.sourceData");
  }
}

// pageArgs is `{module: JSModuleAsString, slug: String, data: {}}`
const createPage = async (pageArgs) => {
  const { body } = await got.post(`http://unix:${socketPath}:/create-page`, {
    json: pageArgs,
  });
  console.log(body);
  return { ok: true };
};
