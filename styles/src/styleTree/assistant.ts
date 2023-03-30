import { ColorScheme } from "../themes/common/colorScheme"
import { Container } from "../types"
import { margin } from "../utils/margin"
import { padding } from "../utils/padding"
import { background, border, text } from "./components"

export const assistant = (colorScheme: ColorScheme) => {
    const layer = colorScheme.middle

    const message: Container = {
        margin: margin(8),
    }

    const message_container_common: Container = {
        corner_radius: 6,
        padding: padding(6, 10),
        margin: padding(6, 8),
    }

    const player_message_container: Container = {
        ...message_container_common,
        background: background(layer, "variant"),
        ...text(layer, "sans", "default", { size: "sm" }),
    }

    const assistant_message_container: Container = {
        ...message_container_common,
        ...text(layer, "sans", "default", { size: "sm" }),
        background: background(layer, "on"),
    }

    const messageHeader = {
        image: {
            width: 20,
            height: 20,
            corderRadius: 10,
        },
        name: {
            ...text(layer, "sans", "default", { size: "sm" }),
        },
        time: {
            ...text(layer, "sans", "variant", { size: "sm" }),
        },
    }

    const editor = {
        minWidth: 200,
        maxWidth: 500,
        padding: padding(8),
        cornerRadius: 8,
        border: border(layer, "on"),
        background: background(layer, "on"),
        text: text(layer, "sans", "default", { size: "sm" }),
        // placeholderText: text(layer, "mono", "disabled"),
        selection: colorScheme.players[0],
    }

    return {
        composer: {
            container: {
                padding: padding(8),
                margin: margin(8),
            },
            editor: editor,
        },
        assistant_message: assistant_message_container,
        player_message: player_message_container,
        error_message: {
            ...assistant_message_container,
            background: background(layer, "negative"),
            ...text(layer, "sans", "negative", { size: "sm" }),
        },
    }
}
