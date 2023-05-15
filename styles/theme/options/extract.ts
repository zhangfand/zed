import { ContainerOptions } from "@theme/container"
import { TextOptions } from "@theme/text/text"

export function extractContainerOptions(
    input: unknown
): Partial<ContainerOptions> | null {
    if (typeof input !== "object" || input === null) {
        return null
    }

    const containerStyleKeys: (keyof ContainerOptions)[] = [
        "background",
        "margin",
        "padding",
        "borderRadius",
        "border",
        "width",
        "height",
        "shadow",
        "themeColor",
        "intensitySet",
    ]

    const result: Record<string, any> = {}

    for (const key of containerStyleKeys) {
        if (key in (input as Record<string, any>)) {
            result[key] = (input as Record<string, any>)[key]
        }
    }

    return Object.keys(result).length > 0
        ? (result as Partial<ContainerOptions>)
        : null
}

export function extractTextOptions(
    input: unknown
): Partial<TextOptions> | null {
    if (typeof input !== "object" || input === null) {
        return null
    }

    const textOptionsKeys: (keyof TextOptions)[] = [
        "family",
        "fontSize",
        "weight",
        "lineHeight",
        "underline",
        "baseFontSize",
        "intensity",
        "themeColor",
    ]

    const result: Record<string, any> = {}

    for (const key of textOptionsKeys) {
        if (key in (input as Record<string, any>)) {
            result[key] = (input as Record<string, any>)[key]
        }
    }

    const validatedResult: Partial<TextOptions> = result

    return Object.keys(validatedResult).length > 0
        ? (result as Partial<TextOptions>)
        : null
}
