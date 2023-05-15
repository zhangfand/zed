import { TextOptions, TextStyle, textStyle } from "@theme/text/text"
import { Prettify } from "@theme/types/utility"
import {
    ContainerOptions,
    ContainerStyle,
    containerStyle,
} from "./containerStyle"
import { Theme } from "@theme"
import { IconStyle } from "@theme/icon/icon"
import {
    extractContainerOptions,
    extractTextOptions,
} from "@theme/options/extract"

export interface ContainedText {
    container: ContainerStyle
    text: TextStyle
}

export type ContainedTextOptions = Partial<TextOptions> & ContainerOptions

export interface ContainedTextProps {
    theme: Theme
    options: ContainedTextOptions
}

export const containedText = ({
    theme,
    options,
}: ContainedTextProps): ContainedText => {
    const textOptions: Partial<TextOptions> = extractTextOptions(options)
    const containerOptions: Partial<ContainerOptions> =
        extractContainerOptions(options)

    const text = textStyle(theme, textOptions)
    const container = containerStyle({ theme, options: containerOptions })

    return {
        text,
        container,
    }
}

export interface ContainedIcon {
    container: ContainerStyle
    icon: IconStyle
}

export type ContainedTextAndIcon = Prettify<ContainedText & ContainedIcon>
