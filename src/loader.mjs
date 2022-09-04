import "./module-aliases.mjs";
const moduleAliases = {
  react: "preact/compat",
  "react-dom": "preact/compat",
  "react/jsx-runtime": "preact/jsx-runtime",
};

export const resolve = (specifier, parentModuleURL, defaultResolve) => {
  return defaultResolve(moduleAliases[specifier] || specifier, parentModuleURL);
};
