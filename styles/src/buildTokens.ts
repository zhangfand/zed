import fs from "fs"
import {
    ColorScheme,
    Layer,
    Style,
    StyleSet,
} from "./themes/common/colorScheme"
import { oneDark } from "./themes/index"

type TokenType = "color" | "sizing" | "spacing" | "other"

/**
 * Interface representing a Design Token.
 */
interface Token {
    /**
     * The value of the token.
     */
    $value: string | number | object

    /**
     * The type of the token, such as "color", "dimension", "number", etc.
     */
    $type: TokenType

    /**
     * Optional plain text description explaining the token's purpose.
     */
    "$description?"?: string

    /**
     * Optional object for storing proprietary, user-, team-, or vendor-specific data.
     */
    "$extensions?"?: {
        [key: string]: any
    }
}

function token(
    name: string,
    value: string,
    tokenType: TokenType,
    description?: string
): Token {
    return {
        $value: value,
        $type: tokenType,
        "$description?": description,
    }
}

function createStyleTokens(style: Style): { [key in keyof Style]: Token } {
    const styleTokens: { [key in keyof Style]: Token } = {
        background: token("background", style.background, "color"),
        border: token("border", style.border, "color"),
        foreground: token("foreground", style.foreground, "color"),
    }
    return styleTokens
}

function createStyleSetTokens(styleSet: StyleSet): {
    [key in keyof StyleSet]: Record<keyof Style, Token>
} {
    const stateTokens: { [key in keyof StyleSet]: Record<keyof Style, Token> } =
        {
            default: createStyleTokens(styleSet.default),
            active: createStyleTokens(styleSet.active),
            disabled: createStyleTokens(styleSet.disabled),
            hovered: createStyleTokens(styleSet.hovered),
            pressed: createStyleTokens(styleSet.pressed),
            inverted: createStyleTokens(styleSet.inverted),
        }
    return stateTokens
}

function createColorTokensFromScheme(colorSchemeData: Layer): {
    [key in keyof Layer]: Record<keyof StyleSet, Record<keyof Style, Token>>
} {
    const colorTokens: {
        [key in keyof Layer]: Record<keyof StyleSet, Record<keyof Style, Token>>
    } = {
        base: createStyleSetTokens(colorSchemeData.base),
        variant: createStyleSetTokens(colorSchemeData.variant),
        on: createStyleSetTokens(colorSchemeData.on),
        accent: createStyleSetTokens(colorSchemeData.accent),
        positive: createStyleSetTokens(colorSchemeData.positive),
        warning: createStyleSetTokens(colorSchemeData.warning),
        negative: createStyleSetTokens(colorSchemeData.negative),
    }
    return colorTokens
}

export default function exportDesignTokens(colorScheme: ColorScheme): void {
    const lowest = createColorTokensFromScheme(colorScheme.lowest)

    const tokens = {
        theme: {
            name: token("theme", colorScheme.name, "other"),
            appearance: token(
                "appearance",
                colorScheme.isLight ? "light" : "dark",
                "other"
            ),
        },
        lowest: lowest,
    }

    fs.writeFileSync("design_tokens.json", JSON.stringify(tokens, null, 2))
}

exportDesignTokens(oneDark)
