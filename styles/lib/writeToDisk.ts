import fs from 'fs';

export function writeToDisk(filePath: string, json: string): void {
  fs.writeFile(filePath, json, (err) => {
    if (err) {
      console.error(err);
      return;
    }
    console.log(`Wrote to disk: ${filePath}`);
  });
}
