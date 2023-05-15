import fs from "fs"

export function writeToDisk(
  filePath: string,
  json: any,
  spaces?: number
): void {
  const jsonString = JSON.stringify(json, null, spaces)
  fs.writeFile(filePath, jsonString, (err) => {
    if (err) {
      console.error(err)
      return
    }
    console.log(`Wrote to disk: ${filePath}`)
  })
}
