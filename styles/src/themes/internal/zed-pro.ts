import chroma, { Color } from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Zed Pro";
const author = "Nate Butler"
const url = "https://github.com/iamnbutler"
const license = {
  type: "?",
  url: "?",
};

// Look into using LAB over HSL
function buildRamp(steps: any, hue: any, saturation: any, lightness: any) {
  let ramp: Color[] = []

  for (let step = 0; step <= steps; step++) {
    let hueIncrement = (hue.end - hue.start) / steps
    let stepHue = hue.start + (hueIncrement * step)

    let saturationIncrement = saturation.end / steps
    let stepSaturation = saturation.start + (saturationIncrement * step)

    let lightnessIncrement = (lightness.end - lightness.start) / steps
    let stepLightness = lightness.start + (lightnessIncrement * step)

    let rampStep = chroma(stepHue, stepSaturation, stepLightness, "hsl")

    ramp.push(rampStep)
  }

  return chroma.scale(ramp)
}

function buildNeutrals() {
  let steps = 11
  let hue = {
    start: 220,
    end: 240,
    curve: "linear"
  }
  let saturation = {
    start: 0.12,
    end: 0.01,
    curve: "linear"
  }
  let lightness = {
    start: 0.08,
    end: 1,
    curve: "linear"
  }

  return buildRamp(steps, hue, saturation, lightness)
}

const domain = [
  0, 0.4, 0.8, 1
]

const neutral = buildNeutrals().domain(domain)

const red = chroma.scale([
  "#fff5f5",
  "#fbafaf",
  "#f57474",
  "#eb4949",
  "#de2c2c",
  "#cc1818",
  "#b40c0c",
  "#960505",
  "#710202",
  "#490000",
  "#1f0000"
].reverse())

const orange = chroma.scale([
  "#ffe8e1",
  "#facdbd",
  "#f4b69a",
  "#eb9260",
  "#de782f",
  "#d36012",
  "#c44d00",
  "#b04400",
  "#963a00",
  "#752e00",
  "#512000",
  "#2b1100"
].reverse())

const yellow = chroma.scale([
  "#fff5db",
  "#f6e1ad",
  "#ecce81",
  "#e0b750",
  "#dda933",
  "#d89920",
  "#d08713",
  "#c4760b",
  "#b56506",
  "#a25602",
  "#8c4701",
  "#753b00"
].reverse())

const green = chroma.scale([
  "#f7fff4",
  "#e8fcdf",
  "#d9faca",
  "#b5ed9d",
  "#8dd86e",
  "#65b944",
  "#409224",
  "#246a10",
  "#134905",
  "#093101",
  "#042400",
  "#031f00"
].reverse())

const cyan = colorRamp(chroma("#215050"))
const blue = colorRamp(chroma("#2F6DB7"))
const violet = colorRamp(chroma("#5874C1"))
const magenta = colorRamp(chroma("#DE9AB8"))

const ramps = {
  neutral: neutral,
  red: red,
  orange: orange,
  yellow: yellow,
  green: green,
  cyan: cyan,
  blue: blue,
  violet: violet,
  magenta: magenta,
};

export const dark = createColorScheme(`${name} Dark`, false, ramps);
export const light = createColorScheme(`${name} Light`, true, ramps);
