import got from "got";

// --loader doesn't show up in argv
const [_node, _binPath, socketPath, toastFilePath, ...args] = process.argv;

main();

async function main() {
  let toast = await import(toastFilePath);
  const res = await got(`http://unix:${socketPath}:/`);
  if (res.body === "ready" && toast.sourceData) {
    await toast.sourceData({ setDataForSlug });
  } else if (res.body !== "ready") {
    throw new Error("Unable to get ready to run toast.sourceData");
  }
}

// pageArgs is `{module: JSModuleAsString, slug: String, data: {}}`
const setDataForSlug = async (slug, pageArgs) => {
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
    await got.post(`http://unix:${socketPath}:/set-data-for-slug`, {
      json: { slug, ...pageArgs },
    });
  } catch (error) {
    if (error.response.statusCode === 422) {
      // unprocessable entity, something is wrong with the payload
      throw new Error(
        `for slug \`${slug}\`, payload keys [${Object.keys(
          pageArgs
        )}] were malformatted`
      );
    } else {
      console.error(error);
    }
  }
  return { ok: true };
};
