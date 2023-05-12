import feedback from "@ui/_legacy/feedback"
import { Theme } from "@/theme"

export const buildUI = (theme: Theme) => {
    console.log(`Reminder: Single color scales are currently placeholders`)

    const ui = {
        feedback: feedback(theme),
    }

    return ui
}
