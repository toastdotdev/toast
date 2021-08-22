import { existsSync, mkdirSync } from "fs";
import { join } from "path";
import { spawnSync } from "child_process";

import axios from "axios";
// import tar from "tar";
import envPaths from "env-paths";
import rimraf from "rimraf";

import os from "os";
import cTable from "console.table";
import { readFileSync, createWriteStream } from "fs";
import { fileURLToPath } from "url";
import path from "path";

const error = (msg, e) => {
  console.error(msg, e);
  process.exit(1);
};

const packageJSON = path.join(
  path.dirname(path.dirname(fileURLToPath(import.meta.url))),
  "package.json"
);

const { version, name, repository, binaryHash, ...etc } = JSON.parse(
  readFileSync(packageJSON)
);

const supportedPlatforms = [
  {
    TYPE: "Windows_NT",
    ARCHITECTURE: "x64",
    TARGET: "windows",
  },
  {
    TYPE: "Linux",
    ARCHITECTURE: "x64",
    TARGET: "linux",
  },
  {
    TYPE: "Darwin",
    ARCHITECTURE: "x64",
    TARGET: "macos",
  },
  {
    TYPE: "Darwin",
    ARCHITECTURE: "arm64",
    TARGET: "macos-arm",
  },
];

export const getPlatform = () => {
  const type = os.type();
  const architecture = os.arch();

  for (let index in supportedPlatforms) {
    let supportedPlatform = supportedPlatforms[index];
    if (
      type === supportedPlatform.TYPE &&
      architecture === supportedPlatform.ARCHITECTURE
    ) {
      return supportedPlatform.TARGET;
    }
  }

  error(
    `Platform with type "${type}" and architecture "${architecture}" is not supported by ${name}.\nYour system must be one of the following:\n\n${cTable.getTable(
      supportedPlatforms
    )}`
  );
};

///

const _installDirectory = envPaths(name).config;
const _binaryDirectory = join(_installDirectory, version, "bin");
const _binaryPath = join(_binaryDirectory, name);

// static information that doesn't change once the package is run.
export const meta = {
  installDir: _installDirectory,
  binaryDir: _binaryDirectory,
  binaryPath: _binaryPath,
  version,
  name,
  repository,
  binaryHash,
};

// users never really call this because removal isn't usually "nicely done"
// it's usually "rm -rf" or something
export function uninstall(
  { installDirectory = _installDirectory } = {
    installDirectory: _installDirectory,
  }
) {
  if (existsSync(installDirectory)) {
    rimraf.sync(installDirectory);
    console.log(
      `${this.name ? this.name : "Your package"} has been uninstalled`
    );
  }
}

const getBinaryUrlForPlatform = ({ binaryHash: bHash, platform } = {}) => {
  let hash = bHash || binaryHash;
  let plat = platform || getPlatform();
  // the url for this binary is constructed from values in `package.json`
  // https://github.com/toastdotdev/toast/releases/download/v1.0.0/toast-example-v1.0.0-x86_64-apple-darwin.tar.gz
  // const url = `${repository.url}/releases/download/v${version}/${name}-v${version}-${platform}.tar.gz`;
  return `https://github.com/toastdotdev/toast/releases/download/binaries-ci-${hash}/${plat}.tar.gz`;
};

export async function installFromUrl({ url: u, binaryDirectory: bDir } = {}) {
  let binaryDirectory = bDir || _binaryDirectory;
  const url = u || getBinaryUrlForPlatform();

  if (existsSync(binaryDirectory)) {
    rimraf.sync(binaryDirectory);
  }

  mkdirSync(binaryDirectory, { recursive: true });

  console.log(`Downloading release from ${url}`);

  await new Promise((resolve, reject) => {
    const writer = createWriteStream(
      path.join(binaryDirectory, "toast.tar.gz")
    );
    axios({ url, responseType: "stream" })
      .then((res) => {
        res.data.pipe(writer);
      })
      .catch((e) => {
        console.error("Error fetching release", e.message);
      });

    writer.on("finish", resolve);
    writer.on("error", reject);
  });

  console.log(`untarring ${name} from ${binaryDirectory}`);
  const result = spawnSync("tar", ["-xf", "toast.tar.gz", "--strip=1"], {
    cwd: binaryDirectory,
    stdio: "inherit",
  });

  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }

  console.log(
    `${name ? name : "Your package"} has been installed to ${binaryDirectory}!`
  );
  return;
}

export function run(
  { binaryPath = _binaryPath } = { binaryPath: _binaryPath }
) {
  const [, , ...args] = process.argv;

  const options = { cwd: process.cwd(), stdio: "inherit" };

  const result = spawnSync(binaryPath, args, options);

  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }

  process.exit(result.status);
}

export function getCurrentPlatformMeta() {
  const platform = getPlatform();
  const url = getBinaryUrlForPlatform({ platform, binaryHash });
  return {
    ...meta,
    remoteBinaryUrl: url,
  };
}
