import { EXPORT_PATH, writeToDisk } from "@/lib/export"
import { tokens } from "./tokens"
import { slugify } from "@/lib/slugify"

export function writeTokens(themeName: string): void {
    const tokensName = slugify(themeName)
    const filePath = `${EXPORT_PATH}/tokens/${tokensName}_tokens.json`
    const json = JSON.stringify(tokens.values, null, 2)

    writeToDisk(filePath, json)
}
