export const findReleaseAsset = (assets, { arch, platform }) => {
  const assetName = `rust-analyzer-${arch}-apple-darwin.gz`;
  return assets.find((asset) => asset.name === assetName);
};
