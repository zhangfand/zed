import {
    ContainedIcon,
    ContainedText,
    ContainedTextAndIcon,
    ContainedTextProps,
    containedText,
} from "./containedElement"
import { FlexStyle } from "@theme/element/flex"
import {
    ContainerStyle,
    containerStyle,
    ContainerOptions,
    DEFAULT_CONTAINER_OPTIONS,
} from "./containerStyle"
export {
    ContainedText,
    ContainedTextProps,
    containedText,
    ContainedIcon,
    ContainedTextAndIcon,
}
export {
    ContainerStyle,
    containerStyle,
    ContainerOptions,
    DEFAULT_CONTAINER_OPTIONS,
}

const blankContainer: ContainerStyle = {}

export const container: Record<string, ContainerStyle> = {
    blank: blankContainer,
}

export type Element =
    | FlexStyle
    | ContainerStyle
    | ContainedIcon
    | ContainedText
    | ContainedTextAndIcon
