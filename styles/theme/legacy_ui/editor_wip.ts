import { Theme } from "@theme"
import { border } from "@theme/border"
import { containedText, containerStyle } from "@theme/container"
import { padding } from "@theme/properties"
import * as text from "@theme/text"
import { textStyle } from "@theme/text"

export default function editor(theme: Theme) {
    const TEXT_SCALE_FACTOR: Readonly<number> = 0.857

    const autocompeteItem = containerStyle({
        borderRadius: 6,
        padding: padding(6, 2),
    })

    function diagnosticStyle(theme: Theme) {
        const header = containerStyle({
            border: border({
                theme,
                options: {
                    position: "top",
                },
            }),
        })

        const message = {
            text: textStyle(theme, {
                size: text.size.sm,
            }),
            highlightText: textStyle(theme, {
                size: text.size.sm,
                weight: 700,
            }),
        }

        return {
            header,
            message,
        }
    }

    const legacy_properties = {}

    return {
        ...legacy_properties,
    }
}
