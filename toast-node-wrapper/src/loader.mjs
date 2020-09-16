import "./module-aliases.mjs";
const moduleAliases = {
  react: "preact/compat",
  "react-dom": "preact/compat",
};

export const resolve = (specifier, parentModuleURL, defaultResolve) => {
  return defaultResolve(moduleAliases[specifier] || specifier, parentModuleURL);
};
