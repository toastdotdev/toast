import moduleAlias from "module-alias";

moduleAlias.addAliases({
  react: "preact/compat",
  "react-dom": "preact/compat",
  "react/jsx-runtime": "preact/jsx-runtime",
  //   'create-react-class': path.resolve(__dirname, './create-preact-class')
});
