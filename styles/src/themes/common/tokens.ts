import { ColorScheme } from "./colorScheme"

export interface Tokens {
    workspace: {
        background: string,
    }
    title_bar: {
        background: string,
    }
    panel: {
        background: string
    }
    tab_bar: {
        background: string,
        tab: {
            active: {
                label: string,
                background: string
            },
            inactive: {
                label: string
                background: string
            },
        }
    },
    editor: {
        background: string,
        gutter: {
            background: string
        },
        line: {
            active: string,
            inactive: string
        }
    }
}

const buildTokens = (colorScheme: ColorScheme): Tokens => {
    const colors = {
        surface: [
            colorScheme.ramps.neutral(0).hex(),
            colorScheme.ramps.neutral(0.1).hex(),
            colorScheme.ramps.neutral(0.2).hex(),
        ],
        text: {
            primary: colorScheme.ramps.neutral(1).hex(),
            secondary: colorScheme.ramps.neutral(0.7).hex(),
        }
    }

    return {
        workspace: {
            background: colors.surface[2]
        },
        title_bar: {
            background: colors.surface[2]
        },
        panel: {
            background: colors.surface[1],
        },
        tab_bar: {
            background: colors.surface[1],
            tab: {
                active: {
                    label: colors.text.primary,
                    background: colors.surface[0],
                },
                inactive: {
                    label: colors.text.secondary,
                    background: colors.surface[1],
                },
            }
        },
        editor: {
            background: colors.surface[0],
            gutter: {
                background: colors.surface[0]
            },
            line: {
                active: colors.text.primary,
                inactive: colors.text.secondary,
            }
        }
    }
}

export default buildTokens