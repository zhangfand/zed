import { Theme, useColors } from "@/theme"
import { border } from "@theme/border"
import {
  BorderRadius,
  ContainedIcon,
  ContainedText,
  ContainedTextAndIcon,
  Interactive,
  buildIntensitiesForStates,
  containerStyle,
} from "@theme/container"
import { TextStyle } from "@theme/text"
import { IntensitySet, resolveElementIntensities } from "@theme/intensity"
import { margin, padding } from "@theme/properties"
import { textStyle, Size as TextSize } from "@theme/text"
import { iconStyle, Size as IconSize } from "@theme/icon"
import { ThemeColor } from "@theme/config"
import { background } from "@theme/properties/background"

type ButtonSizes = "small" | "medium" | "large"
type ButtonSize = (typeof buttonSize)[keyof typeof buttonSize]

const buttonSize: Record<ButtonSizes, number> = {
  small: 15,
  medium: 21,
  large: 25,
}

const DEFAULT_BUTTON_INTENSITIES: IntensitySet = {
  bg: 5,
  border: 8,
  fg: 100,
}

type ButtonOptions = {
  intensities?: IntensitySet
  size?: ButtonSize
  color?: ThemeColor
  /** Providing a width disables width auto-sizing */
  width?: number
  iconSize?: IconSize
  textSize?: TextSize
}

type ButtonKind = "icon" | "label" | "labelAndIcon"

interface ButtonProps {
  theme: Theme
  /** A unique name for the button
   *
   *  Used for debugging & contrast validation  */
  name: string
  kind: ButtonKind
  options?: Partial<ButtonOptions>
}

export type Button<T = ContainedIcon | ContainedText | ContainedTextAndIcon> = Interactive<T>

const DEFAULT_BUTTON_OPTIONS: ButtonOptions = {
  intensities: DEFAULT_BUTTON_INTENSITIES,
  size: buttonSize.medium,
  color: "neutral",
  iconSize: "md",
}

export function buildButton({
  theme,
  name,
  kind = "label",
  options
}: ButtonProps): Button {

  const mergedOptions = {
    ...DEFAULT_BUTTON_OPTIONS,
    ...options,
  }

  const color = useColors(theme)
  const resolvedIntensities = resolveElementIntensities(theme, mergedOptions.intensities)

  const container = containerStyle({
    background: background(theme, resolvedIntensities.bg, mergedOptions.color),
    margin: margin(0, 0, 0, 0),
    padding: padding(6, 4),
    borderRadius: BorderRadius.Medium,
    border: border(theme, resolvedIntensities.border, { color: mergedOptions.color }),
    width: "auto",
    height: mergedOptions.size,
  })

  const icon = iconStyle({
    theme,
    intensity: resolvedIntensities.fg,
    size: mergedOptions.iconSize ? mergedOptions.iconSize : "md",
  })

  const text: TextStyle = textStyle(theme, {
    intensity: resolvedIntensities.fg,
  })

  const states = buildIntensitiesForStates(theme, name, resolvedIntensities)

  const buildStates = (intensities: IntensitySet) => {
    const updatedContainer = {
      ...container,
      background: color.neutral(intensities.bg),
      border: border(theme, intensities.border),
    }

    const updatedIcon = {
      ...icon,
      color: color.neutral(intensities.fg),
    }

    const updatedText = {
      ...text,
      color: color.neutral(intensities.fg),
    }

    let stateStyle

    switch (kind) {
      case "icon":
        stateStyle = {
          container: updatedContainer,
          icon: updatedIcon,
        }

        return stateStyle as ContainedIcon
      case "label":
        stateStyle = {
          container: updatedContainer,
          text: updatedText,
        }
        return stateStyle as ContainedText
      default:
        throw new Error("Unhandled button kind")
    }
  }

  const button = {
    default: buildStates(states.default),
    hovered: buildStates(states.hovered),
    pressed: buildStates(states.pressed),
  }

  switch (kind) {
    case "icon":
      return button as Button<ContainedIcon>
    case "label":
      return button as Button<ContainedText>
    case "labelAndIcon":
      return button as Button<ContainedTextAndIcon>
    default:
      throw new Error("Unhandled button kind")
  }
}

interface ButtonProps {
  theme: Theme,
  options?: ButtonOptions
  componentName?: string
}

export const iconButton = ({ theme, options, componentName }: ButtonProps) =>
  buildButton({
    theme,
    name: componentName ? componentName : "iconButton",
    kind: "icon",
    options
  }) as Button<ContainedIcon>

export const labelButton = ({ theme, options, componentName }: ButtonProps) =>
  buildButton({
    theme,
    name: componentName ? componentName : "labelButton",
    kind: "label",
    options
  }) as Button<ContainedText>

export const labelAndIconButton = ({ theme, options, componentName }: ButtonProps) =>
  buildButton({
    theme,
    name: componentName ? componentName : "labelAndIconButton",
    kind: "label",
    options
  }) as Button<ContainedTextAndIcon>
