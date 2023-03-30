import { ColorScheme } from "../themes/common/colorScheme"
import { Container } from "../types"
import { margin } from "../utils/margin"
import { padding } from "../utils/padding"
import { background, border, text } from "./components"

export const assistant = (colorScheme: ColorScheme) => {
    const layer = colorScheme.highest

    const message_container_common: Container = {
        corner_radius: 6,
        padding: padding(6, 10),
        margin: padding(6, 8),
    }

    const player_message_container: Container = {
        ...message_container_common,
        background: background(layer, "accent"),
        ...text(layer, "sans", "default", { size: "sm" }),
    }

    const player_avatar = {
        width: 32,
        height: 32,
        corderRadius: 16,
        border: border(layer, "on", {
            overlay: true,
        })
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
        placeholderText: text(layer, "mono", "disabled"),
        selection: colorScheme.players[0],
    }

    return {
        surface: {
            background: background(layer),
        },
        player_avatar,
        composer: {
            container: {
                border: border(layer, "variant", {
                    top: true,
                }),
                padding: padding(8, 8, 2, 8),
            },
            editor: editor,
            footer_label: {
                ...text(layer, "sans", "variant", { size: "xs" }),
                padding: padding(4, 8)
            }
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
