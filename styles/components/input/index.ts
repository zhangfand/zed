import { Theme, ThemeColor, useColors } from "@theme"
import { borderStyle } from "@theme/properties/border"
import { buildIntensitiesForStates } from "@theme/state/buildIntensitiesForStates"
import { ContainerOptions } from "@theme/container/containerStyle"
import { ContainerStyle } from "@theme/container/containerStyle"
import { containerStyle } from "@theme/container/containerStyle"
import {
  IntensitySet,
  intensity,
  resolveIntensitySet,
} from "@theme/intensity/intensity"
import { Margin, padding } from "@theme/properties"
import { SelectionStyle, selectionStyle } from "@theme/properties/selection"
import { TextOptions, TextStyle, textStyle } from "@theme/text/text"
import { Interactive } from "@theme/state"

interface InputOptions {
  intensities?: IntensitySet
  themeColor: ThemeColor
  width: number
  height: number
  margin: Margin
}

interface InputStyle {
  container: ContainerStyle
  text: TextStyle
  placeholder: TextStyle
  selection: SelectionStyle
}

const DEFAULT_INPUT_INTENSITIES: IntensitySet = {
  bg: 5,
  border: 8,
  fg: 100,
}

interface InputProps {
  theme: Theme
  options?: Partial<InputOptions>
}

//* A single line input */
export function inputStyle({
  theme,
  options,
}: InputProps): Interactive<InputStyle> {
  const DEFAULT_INPUT_OPTIONS: Partial<InputOptions> = {
    intensities: DEFAULT_INPUT_INTENSITIES,
    themeColor: "neutral",
  }

  const mergedOptions = {
    ...DEFAULT_INPUT_OPTIONS,
    ...options,
  }

  console.log(JSON.stringify(mergedOptions, null, 2))

  const color = useColors(theme)
  const resolvedIntensities = resolveIntensitySet(
    theme,
    mergedOptions.intensities
  )
  const states = buildIntensitiesForStates(
    theme,
    "input",
    resolvedIntensities
  )

  const textOptions: Partial<TextOptions> = mergedOptions
  const containerOptions: Partial<ContainerOptions> = {
    ...containerStyle({
      theme,
      options: mergedOptions,
    }),
    padding: padding(4),
    borderRadius: 4,
  }
  const placeholderOptions: Partial<TextOptions> = {
    ...mergedOptions,
    intensity: intensity.inactive,
  }

  const text = textStyle(theme, textOptions)
  console.log(JSON.stringify(text, null, 2))
  const placeholder = textStyle(theme, placeholderOptions)
  const container = containerStyle({ theme, options: containerOptions })
  const selection = selectionStyle(theme)

  const buildStates = (intensities: IntensitySet): InputStyle => {
    const updatedContainer = {
      ...container,
      background: color[mergedOptions.themeColor](intensities.bg),
      border: borderStyle({
        theme,
        intensity: intensities.border,
        options: {
          color: mergedOptions.themeColor,
        },
      }),
    }

    const updatedText = {
      ...text,
      color: color[mergedOptions.themeColor](intensities.fg),
    }

    return {
      container: updatedContainer,
      text: updatedText,
      placeholder,
      selection,
    }
  }

  return {
    default: buildStates(states.default),
    hovered: buildStates(states.hovered),
    // TODO: We should have some proper way to do a focused state
    pressed: buildStates(states.pressed),
    disabled: buildStates(states.disabled),
  }
}
