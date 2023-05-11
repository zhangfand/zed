import {
  DEFAULT_TEXT_OPTIONS,
  TextOptions,
  TextStyle,
  textStyle,
} from "@theme/text/text"
import { Prettify } from "@theme/types/utility"
import { ContainerOptions, DEFAULT_CONTAINER_OPTIONS } from "./containerStyle"
import { ContainerStyle } from "./containerStyle"
import { containerStyle } from "./containerStyle"
import { Theme } from "@theme"
import { IconStyle } from "@theme/icon/icon"

export interface ContainedText {
  container: ContainerStyle
  text: TextStyle
}

export type ContainedTextOptions = Prettify<
  Partial<TextOptions> & ContainerOptions
>

export interface ContainedTextProps {
  theme: Theme
  options: ContainedTextOptions
}

export const containedText = ({
  theme,
  options
}: ContainedTextProps): ContainedText => {
  const mergedOptions = {
    ...DEFAULT_TEXT_OPTIONS,
    ...DEFAULT_CONTAINER_OPTIONS,
    ...options,
  }

  // const textOptions: Partial<TextOptions> = mergedOptions as Partial<TextOptions>
  // const containerOptions: Partial<ContainerOptions> = mergedOptions as Partial<ContainerOptions>
  const textOptions: Partial<TextOptions> = mergedOptions
  const containerOptions: Partial<ContainerOptions> = mergedOptions

  console.log(`
    textOptions: ${JSON.stringify(textOptions, null, 2)}
    containerOptions: ${JSON.stringify(containerOptions, null, 2)}
    `)

  console.log(textOptions, containerOptions)

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
