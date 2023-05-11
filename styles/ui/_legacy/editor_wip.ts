// import { Theme } from "@theme"
// import { borderStyle } from "@theme/properties/border"
// import { containedText } from "@theme/container"
// import { containerStyle } from "@theme/container/containerStyle"
// import { padding } from "@theme/properties"
// import * as text from "@theme/text/text"
// import { textStyle } from "@theme/text/text"

// export default function editor(theme: Theme) {
//   const TEXT_SCALE_FACTOR: Readonly<number> = 0.857

//   const autocompeteItem = containerStyle({
//     borderRadius: 6,
//     padding: padding(6, 2),
//   })

//   function diagnosticStyle(theme: Theme) {
//     const header = containerStyle({
//       border: borderStyle({
//         theme,
//         options: {
//           position: "top",
//         },
//       }),
//     })

//     const message = {
//       text: textStyle(theme, {
//         size: text.size.sm,
//       }),
//       highlightText: textStyle(theme, {
//         size: text.size.sm,
//         weight: "bold",
//       }),
//     }

//     return {
//       header,
//       message,
//     }
//   }

//   const legacy_properties = {}

//   return {
//     ...legacy_properties,
//   }
// }
