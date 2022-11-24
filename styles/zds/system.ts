import chroma from "chroma-js"

// TODO:
// - Explore a content/interface wrapper that goes around an entire module to pass props like "layer"

export enum Appearance {
  Dark,
  Light
}

type SystemColor =
  | "primary"
  | "primary-container"
  | "secondary"
  | "secondary-container"
  | "accent"
  | "accent-container"
  | "surface"
  | "surface-variant"
  | "background"
  | "negative"
  | "negative-container"
  | "warning"
  | "warning-container"
  | "positive"
  | "positive-container"
  | "on-primary-container"
  | "on-secondary"
  | "on-secondary-container"
  | "on-surface"
  | "on-surface-variant"
  | "on-negative"
  | "on-negative-container"
  | "on-warning"
  | "on-warning-container"
  | "on-positive"
  | "on-positive-container"
  | "on-background"
  | "border"
  | "border-variant"
  | "shadow"
  | "surface-tint-color"
  | "inverse-surface"
  | "inverse-on-surface"
  | "inverse-primary"
  | "overlay"

interface SystemColors {
  "primary": string,
  "primary-container": string,
  "secondary": string,
  "secondary-container": string,
  "accent": string,
  "accent-container": string,
  "surface": string,
  "surface-variant": string,
  "background": string,
  "negative": string,
  "negative-container": string,
  "warning": string,
  "warning-container": string,
  "positive": string,
  "positive-container": string,
  "on-primary-container": string,
  "on-secondary": string,
  "on-secondary-container": string,
  "on-surface": string,
  "on-surface-variant": string,
  "on-negative": string,
  "on-negative-container": string,
  "on-warning": string,
  "on-warning-container": string,
  "on-positive": string,
  "on-positive-container": string,
  "on-background": string,
  "border": string,
  "border-variant": string,
  "shadow": string,
  "surface-tint-color": string,
  "inverse-surface": string,
  "inverse-on-surface": string,
  "inverse-primary": string,
  "overlay": string,
  'level1-surface': string,
  'level1-on-surface': string,
  'level1-surface-variant': string,
  'level1-on-surface-variant': string,
  'level1-inverse-surface': string,
  'level1-inverse-on-surface': string,
  'level2-surface': '#3b4152',
  'level2-on-surface': string,
  'level2-surface-variant': string,
  'level2-on-surface-variant': string,
  'level2-inverse-surface': string,
  'level2-inverse-on-surface': string,
  'level3-surface': '#464e63',
  'level3-on-surface': string,
  'level3-surface-variant': string,
  'level3-on-surface-variant': string,
  'level3-inverse-surface': string,
  'level3-inverse-on-surface': string,
  'level4-surface': '#4f5871',
  'level4-on-surface': string,
  'level4-surface-variant': string,
  'level4-on-surface-variant': string,
  'level4-inverse-surface': string,
  'level4-inverse-on-surface': string
}

interface SystemRadii {
  xs: number,
  sm: number,
  md: number,
  lg: number
}

const ref = {
  palette: {
    neutral: chroma.scale(['#fafafa', '#171717']).colors(101),
    red: chroma.scale(['#fef2f2', '#7f1d1d']).colors(101),
    orange: chroma.scale(['#fff7ed', '#7c2d12']).colors(101),
    yellow: chroma.scale(['#fefce8', '#713f12']).colors(101),
    lime: chroma.scale(['#f0fdf4', '#365314']).colors(101),
    green: chroma.scale(['#f0fdf4', '#14532d']).colors(101),
    teal: chroma.scale(['#f0fdfa', '#134e4a']).colors(101),
    blue: chroma.scale(['#eff6ff', '#1e3a8a']).colors(101),
    indigo: chroma.scale(['#eef2ff', '#312e81']).colors(101),
    violet: chroma.scale(['#f5f3ff', '#4c1d95']).colors(101),
    fuchsia: chroma.scale(['#fdf4ff', '#701a75']).colors(101),
    pink: chroma.scale(['#fdf2f8', '#831843']).colors(101),
  },
  radius: {
    xs: 2,
    sm: 4,
    md: 8,
    lg: 16
  }
}

function buildSurface(appearance: Appearance, name: string, level: number) {

  const surfaceTintColor = appearance ? ref.palette.blue[50] : ref.palette.blue[50]
  const increment = 0.08

  const surface = {
    [`level${level}-${name}`]: appearance
      ? chroma.mix(ref.palette.neutral[99], surfaceTintColor, increment * level).hex()
      : chroma.mix(ref.palette.neutral[10], surfaceTintColor, increment * level).hex(),
    [`level${level}-on-${name}`]: "",
    [`level${level}-${name}-variant`]: "",
    [`level${level}-on-${name}-variant`]: "",
    [`level${level}-inverse-${name}`]: "",
    [`level${level}-inverse-on-${name}`]: ""
  }
  return surface
}

interface System {
  color: Partial<SystemColors>
  borderRadius: SystemRadii
}

export function useSystem(appearance: Appearance) {
  const system: System = {
    color: {
      "primary": appearance ? ref.palette.neutral[40] : ref.palette.neutral[80],
      "primary-container": appearance ? ref.palette.neutral[90] : ref.palette.neutral[30],
      "secondary": appearance ? ref.palette.neutral[60] : ref.palette.neutral[60],
      "secondary-container": appearance ? ref.palette.neutral[95] : ref.palette.neutral[35],
      "accent": appearance ? ref.palette.blue[40] : ref.palette.blue[80],
      "accent-container": appearance ? ref.palette.blue[90] : ref.palette.blue[30],
      "surface": appearance ? ref.palette.neutral[99] : ref.palette.neutral[10],
      "surface-variant": appearance ? ref.palette.neutral[90] : ref.palette.neutral[30],
      "background": appearance ? ref.palette.neutral[99] : ref.palette.neutral[10],
      "negative": appearance ? ref.palette.red[40] : ref.palette.red[80],
      "negative-container": appearance ? ref.palette.red[90] : ref.palette.red[30],
      "warning": appearance ? ref.palette.yellow[40] : ref.palette.yellow[80],
      "warning-container": appearance ? ref.palette.yellow[90] : ref.palette.yellow[30],
      "positive": appearance ? ref.palette.green[40] : ref.palette.green[80],
      "positive-container": appearance ? ref.palette.green[90] : ref.palette.green[30],
      "on-primary-container": appearance ? ref.palette.neutral[10] : ref.palette.neutral[90],
      "on-secondary": appearance ? ref.palette.neutral[100] : ref.palette.neutral[20],
      "on-secondary-container": appearance ? ref.palette.neutral[10] : ref.palette.neutral[90],
      "on-surface": appearance ? ref.palette.neutral[10] : ref.palette.neutral[90],
      "on-surface-variant": appearance ? ref.palette.neutral[30] : ref.palette.neutral[80],
      "on-negative": appearance ? ref.palette.red[100] : ref.palette.red[20],
      "on-negative-container": appearance ? ref.palette.neutral[10] : ref.palette.neutral[90],
      "on-warning": appearance ? ref.palette.yellow[100] : ref.palette.yellow[20],
      "on-warning-container": appearance ? ref.palette.yellow[10] : ref.palette.yellow[90],
      "on-positive": appearance ? ref.palette.green[100] : ref.palette.green[20],
      "on-positive-container": appearance ? ref.palette.green[10] : ref.palette.green[90],
      "on-background": appearance ? ref.palette.neutral[10] : ref.palette.neutral[90],
      "border": appearance ? ref.palette.neutral[50] : ref.palette.neutral[60],
      "border-variant": appearance ? ref.palette.neutral[80] : ref.palette.neutral[30],
      "shadow": appearance ? ref.palette.neutral[0] : ref.palette.neutral[0],
      "surface-tint-color": appearance ? ref.palette.blue[50] : ref.palette.blue[50],
      "inverse-surface": appearance ? ref.palette.neutral[20] : ref.palette.neutral[40],
      "inverse-on-surface": appearance ? ref.palette.neutral[95] : ref.palette.neutral[20],
      "inverse-primary": appearance ? ref.palette.neutral[80] : ref.palette.neutral[40],
      "overlay": appearance ? ref.palette.neutral[0] : ref.palette.neutral[0],
      ...buildSurface(appearance, "surface", 1),
      ...buildSurface(appearance, "surface", 2),
      ...buildSurface(appearance, "surface", 3),
      ...buildSurface(appearance, "surface", 4),
    },
    borderRadius: ref.radius
  }

  return system
}

export default console.log(useSystem(Appearance.Light))