import {
    gray,
    grayA as grayAlpha,
    grayDark,
    grayDarkA as grayDarkAlpha,
} from '@radix-ui/colors'

import chroma from 'chroma-js'

const neutrals = {
    gray,
    grayAlpha,
    grayDark,
    grayDarkAlpha
} as const

type NeutralFamily = keyof typeof neutrals

const objectKeys = <T extends object>(obj: T) => {
    return Object.keys(obj) as Array<keyof T>
}

export const use_neutral = ({ color }: { color: NeutralFamily }) => {
    const neutralArr = objectKeys(neutrals[color])

    return {
        background: chroma(neutrals[color][neutralArr[0]]).hex(),
        background_variant: chroma(neutrals[color][neutralArr[1]]).hex(),
        surface: chroma(neutrals[color][neutralArr[2]]).hex(),
        hover: chroma(neutrals[color][neutralArr[3]]).hex(),
        pressed: chroma(neutrals[color][neutralArr[4]]).hex(),
        selected: chroma(neutrals[color][neutralArr[4]]).hex(),
        border_variant: chroma(neutrals[color][neutralArr[5]]).hex(),
        border: chroma(neutrals[color][neutralArr[6]]).hex(),
        border_hover: chroma(neutrals[color][neutralArr[7]]).hex(),
        background_inverse: chroma(neutrals[color][neutralArr[8]]).hex(),
        background_inverse_hover: chroma(neutrals[color][neutralArr[9]]).hex(),
        foreground_disabled: chroma(neutrals[color][neutralArr[6]]).hex(),
        foreground_hidden: chroma(neutrals[color][neutralArr[7]]).hex(),
        foreground_muted: chroma(neutrals[color][neutralArr[9]]).hex(),
        foreground_variant: chroma(neutrals[color][neutralArr[10]]).hex(),
        foreground: chroma(neutrals[color][neutralArr[11]]).hex(),
    }
}

export const neutral = use_neutral({ color: "grayDark" })
