const { core } = Deno;
const { ops } = core;

const zed = {
  latestNpmPackageVersion: ops.op_latest_npm_package_version,
};

globalThis.zed = zed;
