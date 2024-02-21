const { core } = Deno;
const { ops } = core;

export const latestNpmPackageVersion =
  ops.op_language_server_latest_npm_package_version;
