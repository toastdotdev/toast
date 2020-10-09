import { existsSync, mkdirSync } from "fs";
import { join } from "path";
import { spawnSync } from "child_process";

import axios from "axios";
import tar from "tar";
import envPaths from "env-paths";
import rimraf from "rimraf";

import os from "os";
import cTable from "console.table";
import { readFileSync, writeFileSync } from "fs";
import { fileURLToPath } from "url";
import path from "path";

const error = (msg, e) => {
  console.error(msg, e);
  process.exit(1);
};

// this was originally a soft fork of binary-install
// https://github.com/cloudflare/binary-install
// we need to add versioning and local fetching
class Binary {
  constructor(url, data) {
    if (typeof url !== "string") {
      errors.push("url must be a string");
    } else {
      try {
        new URL(url);
      } catch (e) {
        errors.push(e);
      }
    }
    let errors = [];
    if (data.name && typeof data.name !== "string") {
      errors.push("name must be a string");
    }
    if (data.installDirectory && typeof data.installDirectory !== "string") {
      errors.push("installDirectory must be a string");
    }
    if (!data.installDirectory && !data.name) {
      errors.push("You must specify either name or installDirectory");
    }
    if (errors.length > 0) {
      let errorMsg = "Your Binary constructor is invalid:";
      errors.forEach((error) => {
        errorMsg += error;
      });
      error(errorMsg);
    }
    this.url = url;
    this.name = data.name || -1;
    this.installDirectory = data.installDirectory || envPaths(this.name).config;
    this.binaryDirectory = -1;
    this.binaryPath = -1;
  }

  _getInstallDirectory() {
    if (!existsSync(this.installDirectory)) {
      mkdirSync(this.installDirectory, { recursive: true });
    }
    return this.installDirectory;
  }

  _getBinaryDirectory() {
    const installDirectory = this._getInstallDirectory();
    const binaryDirectory = join(this.installDirectory, "bin");
    if (existsSync(binaryDirectory)) {
      this.binaryDirectory = binaryDirectory;
    } else {
      error(`You have not installed ${this.name ? this.name : "this package"}`);
    }
    return this.binaryDirectory;
  }

  _getBinaryPath() {
    if (this.binaryPath === -1) {
      const binaryDirectory = this._getBinaryDirectory();
      this.binaryPath = join(binaryDirectory, this.name);
    }

    return this.binaryPath;
  }

  async install() {
    if (binaryHash === "<binaryhash>" && !devBinaryTar) return;

    const dir = this._getInstallDirectory();
    if (!existsSync(dir)) {
      mkdirSync(dir, { recursive: true });
    }

    this.binaryDirectory = join(dir, "bin");

    if (existsSync(this.binaryDirectory)) {
      rimraf.sync(this.binaryDirectory);
    }

    mkdirSync(this.binaryDirectory, { recursive: true });

    if (!this.url.startsWith("http")) {
      console.log(
        `Extracting release from ${this.url} to ${this.binaryDirectory}`
      );
      tar
        .x({ file: this.url, strip: 1, C: this.binaryDirectory })
        .then(() => {
          console.log(
            `${this.name ? this.name : "Your package"} has been installed!`
          );
        })
        .catch((e) => {
          error(`Error extracting release: ${e.message}`, e);
        });
    } else {
      console.log(
        `Downloading release from ${this.url} to ${this.binaryDirectory}`
      );

      return axios({ url: this.url, responseType: "stream" })
        .then((res) => {
          res.data.pipe(tar.x({ strip: 1, C: this.binaryDirectory }));
        })
        .then(() => {
          console.log(
            `${this.name ? this.name : "Your package"} has been installed!`
          );
        })
        .catch((e) => {
          error("Error fetching release", e.message);
        });
    }
  }

  uninstall() {
    if (existsSync(this._getInstallDirectory())) {
      rimraf.sync(this.installDirectory);
      console.log(
        `${this.name ? this.name : "Your package"} has been uninstalled`
      );
    }
  }

  run(extraArgs = []) {
    const binaryPath = this._getBinaryPath();
    const [, , ...args] = process.argv;

    const options = { cwd: process.cwd(), stdio: "inherit" };

    const result = spawnSync(binaryPath, [...args, ...extraArgs], options);

    if (result.error) {
      console.error(result.error);
      process.exit(1);
    }

    process.exit(result.status);
  }
}

const packageJSON = path.join(
  path.dirname(path.dirname(fileURLToPath(import.meta.url))),
  "package.json"
);

const {
  version,
  name,
  repository,
  binaryHash,
  devBinaryTar,
  ...etc
} = JSON.parse(readFileSync(packageJSON));

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
];

const getPlatform = () => {
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

const getBinary = () => {
  const platform = getPlatform();
  // the url for this binary is constructed from values in `package.json`
  // https://github.com/toastdotdev/toast/releases/download/v1.0.0/toast-example-v1.0.0-x86_64-apple-darwin.tar.gz
  // const url = `${repository.url}/releases/download/v${version}/${name}-v${version}-${platform}.tar.gz`;
  const url = devBinaryTar
    ? devBinaryTar
    : `https://github.com/toastdotdev/toast/releases/download/binaries-ci-${binaryHash}/${platform}.tar.gz`;
  return new Binary(url, { name: "toast" });
};

export const run = () => {
  const binary = getBinary();
  const toastModules = path.dirname(
    path.dirname(fileURLToPath(import.meta.url))
  );
  const args = ["--toast-module-path", toastModules];
  binary.run(args);
};

export const setLocalBinaryPath = (localPath) => {
  let packageJSONContent = JSON.parse(readFileSync(packageJSON));
  packageJSONContent.devBinaryTar = path.resolve(localPath);
  writeFileSync(packageJSON, JSON.stringify(packageJSONContent, null, 2));
};

export const install = () => {
  const binary = getBinary();
  binary.install();
};

export const uninstall = () => {
  const binary = getBinary();
  binary.uninstall();
};
