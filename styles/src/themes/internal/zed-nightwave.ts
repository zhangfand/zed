import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Zed Nightwave";
const author = "Nate Butler"
const url = "https://github.com/iamnbutler"
const license = {
  type: "MIT",
  url: "https://opensource.org/licenses/MIT",
};

const red = colorRamp(chroma("#ED08C9"))
const orange = colorRamp(chroma("#FB862D"))
const yellow = colorRamp(chroma("#FDF970"))
const green = colorRamp(chroma("#2A643D"))
const blue = colorRamp(chroma("#2F6DB7"))
const cyan = colorRamp(chroma("#76F4FC"))

const pink = chroma.scale([
  "#4E1760",
  "#751E69",
  "#A3227D",
  "#D22791",
  "#EB49D0",
  "#ED79C0",
  "#ED79C0",
  "#F2A7B4",
  "#F8D6A9",
  "#FFFD82",
]).correctLightness()

const purple = chroma.scale([
  "#07081C",
  "#431B67",
  "#5E3292",
  "#6442A6",
  "#6B53BB",
  "#7265D0",
  "#937ED1",
  "#B497D2",
  "#FFFFFF",
])

const ramps = {
  neutral: chroma.scale([
    "#191124",
    "#1E1230",
    "#2A173F",
    "#2F1A47",
    "#402155",
    "#513465",
    "#584271",
    "#70648d",
    "#8886a9",
    "#a0a8c5",
    "#b8cae1",
    "#B9DDFD",
    "#F9FFFF",
  ]).mode("hsv"),
  red: red,
  orange: orange,
  yellow: pink,
  green: blue,
  cyan: cyan,
  blue: yellow,
  violet: purple,
  magenta: green,
};

export const dark = createColorScheme(`${name} Dark`, false, ramps);