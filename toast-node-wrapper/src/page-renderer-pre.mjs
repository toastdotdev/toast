import { render as prender } from "preact-render-to-string";
import { h } from "preact";
import { Helmet } from "react-helmet";

const htmlTemplate = ({
  componentPath,
  pageWrapperPath,
  dataPath,
  appHtml,
  helmet,
}) => `<!DOCTYPE html>
<script>
window.componentPath = "${componentPath}";
window.wrapperComponentPath = ${pageWrapperPath && `"${pageWrapperPath}"`};
window.dataPath = ${dataPath && `"${dataPath}"`};
</script>
<html ${helmet.htmlAttributes.toString()}>
  <head>
  ${helmet.title.toString()}
  ${helmet.meta.toString()}
  ${helmet.link.toString()}
  ${helmet.script.toString()}
  ${helmet.noscript.toString()}
  </head>
  <body ${helmet.bodyAttributes.toString()}>
    <div id="toast-page-section">${appHtml}</div>
    <script type="module">
    /* @jsx jsx */

async function renderPage() {
  const promises = [
    import(window.componentPath),
    window.wrapperComponentPath
      ? import(window.wrapperComponentPath)
      : undefined,
    window.dataPath
      ? fetch(window.dataPath).then(response => {
          return response.json();
        })
      : {},
    import("/web_modules/preact.js")
  ];

  let pageWrapper = ({ children }) => h("div", null, children);
  const [
    PageModule,
    PageWrapperModule,
    pageData,
    { render, h }
  ] = await Promise.all(promises);
  const Page = PageModule.default;
  if(PageWrapperModule) {
    pageWrapper = PageWrapperModule.default
  }

  render(
    h(pageWrapper, pageData, h(Page, pageData)),
    document.getElementById("toast-page-section")
  );
}

renderPage();

</script>
  </body>
</html>
`;

const windowsLocalDevPathReplacement = /\\/g;

export const render = async ({
  component,
  pageWrapper,
  data = {},
  browserComponentPath,
  browserPageWrapperPath,
  browserDataPath,
}) => {
  browserPageWrapperPath = pageWrapper ? browserPageWrapperPath : undefined;
  pageWrapper = pageWrapper
    ? pageWrapper
    : ({ children }) => h("div", null, children);

  const output = prender(h(pageWrapper, data, h(component, data)));
  //   console.log(output);
  const helmet = Helmet.renderStatic();
  return htmlTemplate({
    componentPath: browserComponentPath.replace(
      windowsLocalDevPathReplacement,
      "/"
    ),
    pageWrapperPath: browserPageWrapperPath,
    dataPath:
      Object.keys(data).length > 0
        ? browserDataPath.replace(windowsLocalDevPathReplacement, "/")
        : undefined,
    appHtml: output,
    helmet,
  });
};
