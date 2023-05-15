import { clearDirectory } from "./clearDirectory"
import { writeToDisk } from "./writeToDisk"
export { clearDirectory, writeToDisk }

// export const EXPORT_PATH = './target';
export const EXPORT_PATH = `${__dirname}/../../assets/themes/wip`
const DIRECTORIES_TO_KEEP = ["tokens"]

export function exportData(
  data: Array<{ name: string; json: any; path: string }>
): void {
  console.log(`${__dirname}/../../assets/themes/wip`)
  clearDirectory(EXPORT_PATH, DIRECTORIES_TO_KEEP)

  data.forEach(({ name, json, path }) => {
    const slug = name.toLowerCase().replace(/ /g, "_")
    const filePath = `${path}/${slug}.json`
    writeToDisk(filePath, json, 2)
  })

  console.log(`Exported data to: ${EXPORT_PATH}`)
}
