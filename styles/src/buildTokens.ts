import fs from "fs"
import {
    ColorScheme,
    Layer,
    Player,
    Players,
    Style,
    StyleSet,
} from "./themes/common/colorScheme"
import * as themes from "./themes/index"
import { snakeCase } from "case-anything"

type TokenType = "color" | "sizing" | "spacing" | "other"

/**
 * Interface representing a Design Token.
 */
interface Token {
    /**
     * The value of the token.
     */
    value: string | number | object

    /**
     * The type of the token, such as "color", "dimension", "number", etc.
     */
    type: TokenType

    /**
     * Optional plain text description explaining the token's purpose.
     */
    "description?"?: string

    /**
     * Optional object for storing proprietary, user-, team-, or vendor-specific data.
     */
    "extensions?"?: {
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
        value: value,
        type: tokenType,
        "description?": description,
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

function createPlayerTokens(player: Player): { [key in keyof Player]: Token } {
    const playerTokens: { [key in keyof Player]: Token } = {
        cursor: token("cursor", player.cursor, "color"),
        selection: token("selection", player.selection, "color"),
    }
    return playerTokens
}

function createPlayersTokens(players: Players): { [key: string]: Token } {
    const playersTokens: { [key: string]: Token } = {}

    for (const playerKey in players) {
        const player = players[playerKey as keyof Players]
        const playerTokens = createPlayerTokens(player)

        for (const tokenKey in playerTokens) {
            playersTokens[`player_${playerKey}_${tokenKey}`] =
                playerTokens[tokenKey as keyof Player]
        }
    }

    return playersTokens
}

export default function exportThemeTokens(colorScheme: ColorScheme): void {
    const slug = snakeCase(colorScheme.name.toLowerCase())

    const lowest = createColorTokensFromScheme(colorScheme.lowest)
    const middle = createColorTokensFromScheme(colorScheme.middle)
    const highest = createColorTokensFromScheme(colorScheme.highest)

    const players = createPlayersTokens(colorScheme.players)

    const tokens = {
        theme: {
            name: token("theme", colorScheme.name, "other"),
            appearance: token(
                "appearance",
                colorScheme.isLight ? "light" : "dark",
                "other"
            ),
        },
        color: {
            players,
            surface: {
                lowest: lowest,
                middle: middle,
                highest: highest,
            },
        },
    }

    fs.writeFileSync(`tokens/${slug}.json`, JSON.stringify(tokens, null, 2))
}

// Export all the themes as tokens
Object.values(themes).forEach((theme) => {
    exportThemeTokens(theme)
})
