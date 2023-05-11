import { Theme, ThemeColor } from "@theme"
import { ContainedTextAndIcon, Interactive, Toggleable, containerStyle } from "@theme/container"
import { iconStyle } from "@theme/icon"
import { textStyle } from "@theme/text"

export type InteractiveListItem = Interactive<ContainedTextAndIcon>
export type ToggleableInteractiveListItem = Toggleable<Interactive<ContainedTextAndIcon>>

interface InteractiveListItemOptions {
  themeColor: ThemeColor
}
interface InteractiveListItemProps {
  theme: Theme,
  options?: InteractiveListItemOptions
}

function interactiveListItem({
  theme,
  options
}: InteractiveListItemProps) {
  const item: InteractiveListItem = {}

  const text = textStyle(theme)
  const icon = iconStyle({ theme, iconSize: "md" })
  const container = containerStyle({})

  // implement here

  return item
}

function toggleableInteractiveListItem({
  theme,
  options
}: InteractiveListItemProps) {
  const item: InteractiveListItem = {}
  const activeItem: InteractiveListItem = {}

  // implement here

  return {
    inactive: item,
    active: activeItem
  }
}
