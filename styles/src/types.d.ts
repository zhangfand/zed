import { ContainerStyle } from "../../crates/gpui/bindings/ContainerStyle"
import { TextStyle } from "../../crates/gpui/bindings/TextStyle"
import { Margin } from "../../crates/gpui/bindings/Margin"
import { Border } from "../../crates/gpui/bindings/Border"
import { Padding } from "../../crates/gpui/bindings/Padding"
import { ImageStyle } from "../../crates/gpui/bindings/ImageStyle"
import { Shadow } from "../../crates/gpui/bindings/Shadow"
import { Underline } from "../../crates/gpui/bindings/Underline"

type Container = Partial<ContainerStyle>
type Text = Partial<TextStyle>
type Image = Partial<ImageStyle>
interface ContainedText extends Container, Text { }

export { Container, Text, ContainedText, Margin, Border, Padding, Image, Shadow, Underline }
