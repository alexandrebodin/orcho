// Most of this file is ripped from esbuild
// @see https://github.com/evanw/esbuild/blob/master/lib/npm/node-install.ts
// This file is MIT licensed.

const { pkgAndSubpathForCurrentPlatform } = require("./node-platform");

async function checkAndPreparePackage() {
  const { pkg, subpath } = pkgAndSubpathForCurrentPlatform();

  try {
    // First check for the binary package from our "optionalDependencies". This
    // package should have been installed alongside this package at install time.
    require.resolve(`${pkg}/${subpath}`);
  } catch (e) {
    console.error(`Failed to find package "${pkg}" on the file system

This can happen if you use the "--no-optional" flag. The "optionalDependencies"
package.json feature is used by turbo to install the correct binary executable
for your current platform. This install script will now attempt to work around
this. If that fails, you need to remove the "--no-optional" flag to use turbo.
`);
  }
}

checkAndPreparePackage();
