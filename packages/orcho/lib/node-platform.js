"use strict";

const os = require("os");

const PKG = "@alexandrebodin/orcho";
const BIN = "bin/orcho";

const knownWindowsPackages = {
  "win32 arm64 LE": `${PKG}-windows-arm64`,
  "win32 x64 LE": `${PKG}-windows-64`,
};

const knownUnixlikePackages = {
  "darwin arm64 LE": `${PKG}-darwin-arm64`,
  "darwin x64 LE": `${PKG}-darwin-x64`,
  "linux arm64 LE": `${PKG}-linux-arm64`,
  "linux x64 LE": `${PKG}-linux-64`,
};

function pkgAndSubpathForCurrentPlatform() {
  let pkg;
  let subpath;
  let platformKey = `${process.platform} ${os.arch()} ${os.endianness()}`;

  if (platformKey in knownWindowsPackages) {
    pkg = knownWindowsPackages[platformKey];
    subpath = `${BIN}.exe`;
  } else if (platformKey in knownUnixlikePackages) {
    pkg = knownUnixlikePackages[platformKey];
    subpath = BIN;
  } else {
    throw new Error(`Unsupported platform: ${platformKey}`);
  }

  return { pkg, subpath };
}

function generateBinPath() {
  const { pkg, subpath } = pkgAndSubpathForCurrentPlatform();
  let binPath;

  console.log('ici', require.resolve('@alexandrebodin/orcho-darwin-arm64'), subpath);

  try {
    // First check for the binary package from our "optionalDependencies". This
    // package should have been installed alongside this package at install time.
    binPath = require.resolve(`${pkg}/${subpath}`);
  } catch (e) {
    throw e;
  }

  return binPath;
}

module.exports = {
  generateBinPath,
  pkgAndSubpathForCurrentPlatform,
};
