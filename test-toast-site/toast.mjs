export const sourceData = async ({ createPage }) => {
  // data fetching
  await createPage({
    module: page,
    slug: "/about",
    data: {},
    moduleType: "mdx",
  });
  return;
};

const page = `/** @jsx mdx */
import {mdx} from '@mdx-js/preact';

const layoutProps = {};
const MDXLayout = "wrapper";
export default function MDXContent({ components, ...props }) {
  return (
    <MDXLayout
      {...layoutProps}
      {...props}
      components={components}
      mdxType="MDXLayout"
    >
      <h1>{\`some page\`}</h1>
      <p>{\`with mdx content\`}</p>
      <pre>
        <code parentName="pre" {...{}}>{\`const thing = 1;
\`}</code>
      </pre>
    </MDXLayout>
  );
}

MDXContent.isMDXComponent = true;
`;
