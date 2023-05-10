import {
  DEFAULT_TEXT_OPTIONS,
  TextOptions,
  TextStyle,
  textStyle,
} from "@theme/text"
import { Prettify } from "@theme/types/utility"
import {
  ContainerOptions,
  ContainerStyle,
  DEFAULT_CONTAINER_OPTIONS,
  containerStyle,
} from "."
import { Theme } from "@theme"

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
  options,
}: ContainedTextProps): ContainedText => {
  const mergedOptions = {
    ...DEFAULT_TEXT_OPTIONS,
    ...DEFAULT_CONTAINER_OPTIONS,
    ...options,
  }

  const textOptions: Partial<TextOptions> = mergedOptions
  const containerOptions: Partial<ContainerOptions> = mergedOptions

  const text = textStyle(theme, textOptions)
  const container = containerStyle(containerOptions)

  return {
    text,
    container,
  }
}
