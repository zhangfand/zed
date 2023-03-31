import { ColorScheme, Layer } from "../themes/common/colorScheme"
import { ContainedText, Container, InteractiveContainer } from "../types"
import { margin } from "../utils/margin"
import { padding } from "../utils/padding"
import { background, border, text } from "./components"

type PlayerType = "player" | "assistant"

type ProseMessage = ContainedText
type CodeMessage = ContainedText
type Header = ContainedText


interface MessageListItems {
    header: Header
    prose_message: ProseMessage
    code_message: CodeMessage
}

export const assistant = (colorScheme: ColorScheme) => {
    const layer = colorScheme.middle

    function build_list_items(
        layer: Layer,
        playerType: PlayerType,
    ): MessageListItems {
        const listItemCommon = {
            margin: margin(0, 8),
            padding: padding(4, 8),
        }

        const header: Header = {
            ...listItemCommon,
            margin: {
                top: 8,
                bottom: 0,
                left: 0,
                right: 0
            },
            ...text(layer, "sans", "default", { size: "xs" }),
        }

        const prose_message = {
            ...listItemCommon,
            background: playerType === "assistant"
                ? background(colorScheme.lowest)
                : background(colorScheme.lowest, "accent"),
            ...text(layer, "sans", "default", { size: "sm" }),
        }
        const code_message = {
            ...listItemCommon,
            background: background(colorScheme.highest, "default"),
            border: border(layer, "default", { overlay: true }),
            ...text(layer, "mono", "default", { size: "sm" }),
        }

        const listItems = {
            header,
            prose_message,
            code_message
        }

        return listItems
    }

    const composer_input = {
        minWidth: 200,
        maxWidth: 500,
        padding: padding(8),
        cornerRadius: 8,
        border: border(layer, "on"),
        background: background(layer, "on"),
        text: text(layer, "sans", "default", { size: "sm" }),
        placeholderText: text(layer, "mono", "disabled"),
        selection: colorScheme.players[0],
    }

    return {
        surface: {
            background: background(layer),
        },
        composer: {
            container: {
                border: border(layer, "variant", {
                    top: true,
                }),
                padding: padding(8, 8, 2, 8),
            },
            editor: composer_input,
            footer_label: {
                ...text(layer, "sans", "variant", { size: "xs" }),
                padding: padding(4, 8)
            }
        },
        assistant_message: build_list_items(layer, "assistant"),
        player_message: build_list_items(layer, "player"),
    }
}
