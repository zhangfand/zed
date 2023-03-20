Design Tokens Format Module
Draft Community Group Report 07 February 2023

Latest published version:
https://tr.designtokens.org/format/
Editors:
Daniel Banks
Donna Vitan
James Nash
Kevin Powell
Louis Chenais
Feedback:
GitHub design-tokens/community-group (pull requests, new issue, open issues)
Abstract
This document describes the technical specification for a file format to exchange design tokens between different tools.

Status of This Document
This specification was published by the Design Tokens Community Group. It is not a W3C Standard nor is it on the W3C Standards Track. Please note that under the W3C Community Contributor License Agreement (CLA) there is a limited opt-out and other conditions apply. Learn more about W3C Community and Business Groups.

This is a snapshot of the editors' draft. It is provided for discussion only and may change at any moment. Its publication here does not imply endorsement of its contents by W3C or the Design Tokens Community Group Membership. Don't cite this document other than as work in progress.

This document has been published to facilitate Wide Review.

This document was produced by the Design Tokens Community Group, and contributions to this draft are governed by Community Contributor License Agreement (CLA), as specified by the W3C Community Group Process.

GitHub Issues are preferred for discussion of this specification.

table of contents
Abstract
Status of This Document
1.Conformance
2.Introduction
3.Terminology
3.1(Design) Token
3.2(Design) Token Properties
3.3Design tool
3.4Translation tool
3.5Documentation tool
3.6Type
3.7Group
3.8Alias (Reference)
3.9Composite (Design) Token
4.File format
4.1Media type (MIME type)
4.2File extensions
5.Design token
5.1Name and value
5.1.1Character restrictions
5.2Additional properties
5.3Description
5.4Type
5.5Extensions
5.6More token properties TBC
6.Groups
6.1Additional group properties
6.1.1Description
6.1.2Type
6.2Use-cases
6.2.1File authoring & organization
6.2.2GUI tools
6.2.3Translation tools
7.Aliases / references
8.Types
8.1Color
8.2Dimension
8.3Font family
8.4Font weight
8.5Duration
8.6Cubic Bézier
8.7Number
8.8Additional types
9.Composite types
9.1Groups versus composite tokens
9.2Stroke style
9.2.1String value
9.2.2Object value
9.2.3Fallbacks
9.3Border
9.4Transition
9.5Shadow
9.6Gradient
9.7Typography
A.Issue summary
B.References
B.1Normative references
1. Conformance
As well as sections marked as non-normative, all authoring guidelines, diagrams, examples, and notes in this specification are non-normative. Everything else in this specification is normative.

The key words MAY, MUST, MUST NOT, SHOULD, and SHOULD NOT in this document are to be interpreted as described in BCP 14 [RFC2119] [RFC8174] when, and only when, they appear in all capitals, as shown here.

2. Introduction
This section is non-normative.

Design tokens are a methodology for expressing design decisions in a platform-agnostic way so that they can be shared across different disciplines, tools, and technologies. They help establish a common vocabulary across organisations.

There is a growing ecosystem of tools for design system maintainers and consumers that incorporate design token functionality, or would benefit from doing so:

Design tools have begun allowing designers to label and reference shared values for design properties like colors and sizes.
Translation tools exist that can convert source design token data into platform-specific source code that can directly be used by developers.
Documentation tools can display design token names rather than the raw values in design specs and style guides.
It is often desirable for design system teams to integrate such tools together, so that design token data can flow between design and development tools.

For example:

Extracting design tokens from design files and feeding them into translation tools to then be converted into platform-specific code
Maintaining a "single source of truth" for design tokens and automatically keeping design and development tools in sync with it
While many tools now offer APIs to access design tokens or the ability to export design tokens as a file, these are all tool-specific. The burden is therefore on design system teams to create and maintain their own, bespoke "glue" code or workflows. Furthermore, if teams want to migrate to different tools, they will need to update those integrations.

This specification aims to facilitate better interoperability between tools and thus lower the work design system teams need to do to integrate them by defining a standard file format for expressing design token data.

3. Terminology
These definitions are focused on the technical aspects of the specification, aimed at implementers such as design tool vendors. Definitions for designers and developers are available at designtokens.org.

3.1 (Design) Token
A (Design) Token is an information associated with a name, at minimum a name/value pair.

For example:

color-text-primary: #000000;
font-size-heading-level-1: 44px;
The name may be associated with additional Token Properties.

3.2 (Design) Token Properties
Information associated with a token name.

For example:

Value
Type
Description
Additional metadata may be added by tools and design systems to extend the format as needed.

3.3 Design tool
A design tool is a tool for visual design creation and editing.

For example:

Bitmap image manipulation programs:
Photoshop
Affinity Photo
Paint.net
Vector graphics tools:
Illustrator
Inkscape
UI design, wireframing and prototyping tools:
Adobe XD
UXPin
Sketch
Figma
...
3.4 Translation tool
Design token translation tools translate token data from one format to another.

For example:

Theo
Style Dictionary
Diez
Specify
...
3.5 Documentation tool
A documentation tool is a tool for documenting design tokens usage.

For example:

Storybook
Zeroheight
Backlight
Specify
Supernova
Knapsack
...
3.6 Type
A token's type is a predefined categorization applied to the token's value.

For example:

Color
Size
Duration
Token tools can use Types to infer the purpose of a token.

For example:

A translation tool might reference a token's type to convert the source value into the correct platform-specific format.
A visual design tool might reference type to present tokens in the appropriate part of their UI - as in, color tokens are listed in the color picker, font tokens in the text styling UI's fonts list, and so on.
3.7 Group
A group is a set of tokens belonging to a specific category.

For example:

Brand
Alert
Layout
Groups are arbitrary and tools SHOULD NOT use them to infer the type or purpose of design tokens.

3.8 Alias (Reference)
A design token's value can be a reference to another token. The same value can have multiple names or aliases.

The following Sass example illustrates this concept:

$color-palette-black: #000000;
$color-text-primary: $color-palette-black;
The value of $color-text-primary is #000000, because $color-text-primary references $color-palette-black. We can also say $color-text-primary is an alias for $color-palette-black.

3.9 Composite (Design) Token
A design token whose value is made up of multiple, named child values. Composite tokens are useful for closely related style properties that are always applied together. For example, a typography style might be made up of a font name, font size, line height, and color.

Here's an example of a composite shadow token:

{
  "shadow-token": {
    "$type": "shadow",
    "$value": {
      "color": "#00000080",
      "offsetX": "0.5rem",
      "offsetY": "0.5rem",
      "blur": "1.5rem",
      "spread": "0rem"
    }
  }
}
4. File format
Design token files are JSON (https://www.json.org/) files that adhere to the structure described in this specification.

JSON was chosen as an interchange format on the basis of:

Broad support in many programming languages' standard libraries. This is expected to lower barriers to entry for developers writing software that supports design token files.
Current popularity and widespread use. This is expected to lower the learning curve as many people will already be familiar with JSON.
Being text-based (rather than binary) allows hand-editing design token files without needing specialized software other than a basic text editor. It also means the files are somewhat human-readable.
4.1 Media type (MIME type)
When serving design token files via HTTP / HTTPS or in any other scenario where a media type (formerly known as MIME type) needs to be specified, the following MIME type SHOULD be used for design token files:

application/design-tokens+json
However, since every design token file is a valid JSON file, they MAY be served using the JSON media type: application/json. The above, more specific media type is preferred and SHOULD be used wherever possible.

Tools that can open design token files MUST support both media types.

4.2 File extensions
When saving design token files on a local file system, it can be useful to have a distinct file extension as this makes them easier to spot in file browsers. It may also help to associate a file icon and a preferred application for opening those files. The following file extensions are recommended by this spec:

.tokens
.tokens.json
The former is more succinct. However, until this format is widely adopted and supported, the latter might be useful to make design token files open in users' preferred JSON editors.

Tools that can open design token files MAY filter available files (e.g. in an open file dialog) to only show ones using those extensions. It is recommended to also provide users with a way of opening files that do not use those extensions (e.g. a "show all files" option or similar).

Tools that can save design token files SHOULD append one of the recommended file extensions to the filename when saving.

EDITOR'S NOTE: JSON schema
The group is currently exploring the addition of a JSON Schema to support the spec.

EDITOR'S NOTE: JSON file size limitations
A concern about file size limitations of JSON files was raised by one of the vendors. The working group continues to gather feedback about any limitations the JSON format imposes.

5. Design token
5.1 Name and value
EXAMPLE 1: Minimal file with single design token
{
  "token name": {
    "$value": "#fff000",
    "$type": "color"
  }
}
Note: The $type property has been added to ensure this example is valid. Please refer to the design token type chapter for more details.

An object with a $value property is a token. Thus, $value is a reserved word in our spec, meaning you can't have a token whose name is "$value". The parent object's key is the token name.

The example above therefore defines 1 design token with the following properties:

Name: "token name"
Value: "#fff000"
Type: "color"
Name and value are both required.

Token names are case-sensitive, so the following example with 2 tokens in the same group whose names only differ in case is valid:

EXAMPLE 2
{
  "font-size": {
    "$value": "3rem",
    "$type": "dimension"
  },

  "FONT-SIZE": {
    "$value": "16px",
    "$type": "dimension"
  }
}
However, some tools MAY need to transform names when exporting to other languages or displaying names to the user, so having token names that differ only in case is likely to cause identical and undesirable duplicates in the output. For example, a translation tool that converts these tokens to Sass code would generate problematic output like this:

EXAMPLE 3
$font-size: 3rem;
$font-size: 16px;

// The 2nd $font-size overrides the 1st one, so the 1st token
// has essentially been lost.
Tools MAY display a warning when token names differ only by case.

5.1.1 Character restrictions
All properties defined by this format are prefixed with the dollar sign ($). This convention will also be used for any new properties introduced by future versions of this spec. Therefore, token and group names MUST NOT begin with the $ character.

Furthermore, due to the syntax used for token aliases the following characters MUST NOT be used anywhere in a token or group name:

{ (left curly bracket)
} (right curly bracket)
. (period)
EDITOR'S NOTE: '$' Prefix Rationale
Because of the decision to prefix group properties with a dollar sign ($), token properties will also use a dollar sign prefix. This provides a consistent syntax across the spec.

5.2 Additional properties
While $value is the only required property for a token, a number of additional properties MAY be added:

5.3 Description
A plain text description explaining the token's purpose can be provided via the optional $description property. Tools MAY use the description in various ways.

For example:

Style guide generators MAY display the description text alongside a visual preview of the token
IDEs MAY display the description as a tooltip for auto-completion (similar to how API docs are displayed)
Design tools MAY display the description as a tooltip or alongside tokens wherever they can be selected
Translation tools MAY render the description to a source code comment alongside the variable or constant they export.
The value of the $description property MUST be a plain JSON string, for example:

EXAMPLE 4
{
  "Button background": {
    "$value": "#777777",
    "$type": "color",
    "$description": "The background color for buttons in their normal state."
  }
}
5.4 Type
Design tokens always have an unambiguous type, so that tools can reliably interpret their value.

A token's type can be specified by the optional $type property. If the $type property is not set on a token, then the token's type MUST be determined as follows:

If the token's value is a reference, then its type is the resolved type of the token being referenced.
Otherwise, if any of the token's parent groups have a $type property, then the token's type is inherited from the closest parent group with a $type property.
Otherwise, if none of the parent groups have a $type property, the token's type cannot be determined and the token MUST be considered invalid.
Tools MUST NOT attempt to guess the type of a token by inspecting the contents of its value.

The $type property can be set on different levels:

at the group level
at the token level
The $type property MUST be a plain JSON string, whose value is one of the values specified in this specification's respective type definitions. The value of $type is case-sensitive.

For example:

EXAMPLE 5
{
  "Button background": {
    "$value": "#777777",
    "$type": "color"
  }
}
5.5 Extensions
The optional $extensions property is an object where tools MAY add proprietary, user-, team- or vendor-specific data to a design token. When doing so, each tool MUST use a vendor-specific key whose value MAY be any valid JSON data.

The keys SHOULD be chosen such that they avoid the likelihood of a naming clash with another vendor's data. The reverse domain name notation is recommended for this purpose.
Tools that process design token files MUST preserve any extension data they do not themselves understand. For example, if a design token contains extension data from tool A and the file containing that data is opened by tool B, then tool B MUST include the original tool A extension data whenever it saves a new design token file containing that token.
EXAMPLE 6
{
  "Button background": {
    "$value": "#777777",
    "$type": "color",
    "$extensions": {
      "org.example.tool-a": 42,
      "org.example.tool-b": {
        "turn-up-to-11": true
      }
    }
  }
}
In order to maintain interoperability between tools that support this format, teams and tools SHOULD restrict their usage of extension data to optional meta-data that is not crucial to understanding that token's value.

Tool vendors are encouraged to publicly share specifications of their extension data wherever possible. That way other tools can add support for them without needing to reverse engineer the extension data. Popular extensions could also be incorporated as standardized features in future revisions of this specification.

EDITOR'S NOTE: Extensions section
The extensions section is not limited to vendors. All token users can add additional data in this section for their own purposes.

5.6 More token properties TBC
6. Groups
A file MAY contain many tokens and they MAY be nested arbitrarily in groups like so:

EXAMPLE 7
{
  "token uno": {
    "$value": "#111111",
    "$type": "color"
  },
  "token group": {
    "token dos": {
      "$value": "2rem",
      "$type": "dimension"
    },
    "nested token group": {
      "token tres": {
        "$value": 33,
        "$type": "number"
      },
      "Token cuatro": {
        "$value": 444,
        "$type": "fontWeight"
      }
    }
  }
}
The names of the groups leading to a given token (including that token's name) are that token's path, which is a computed property. It is not specified in the file, but parsers that conform to this spec MUST be able to expose the path of a token. The above example, therefore, defines 4 design tokens with the following properties:

Token #1
Name: "token uno"
Path: "token uno"
Value: "#111111"
Type: "color"
Token #2
Name: "token dos"
Path: "token group" / "token dos"
Value: "2rem"
Type: "dimension"
Token #3
Name: "token tres"
Path: "token group" / "nested token group" / "token tres"
Value: 33
Type: "number"
Token #4
Name: "token cuatro"
Path: "token group" / "nested token group" / "token cuatro"
Value: 444
Type: "fontWeight"
Because groupings are arbitrary, tools MUST NOT use them to infer the type or purpose of design tokens.

Groups items (i.e. the tokens and/or nested groups) are unordered. In other words, there is no implicit order between items within a group. Therefore, tools that parse or write design token files are not required to preserve the source order of items in a group.

The names of items in a group are case sensitive. As per the guidance in the design token chapter, tools MAY display a warning to users when groups contain items whose names differ only in case and could therefore lead to naming clashes when exported.

EDITOR'S NOTE: Naming practices
The format editors acknowledge existing best-practices for token naming, but place no direct constraints on naming via the specification.

6.1 Additional group properties
EDITOR'S NOTE: Group properties vs. nested group and token names
To prevent collisions with token names, token properties are prefixed with a dollar sign ($). Using this prefix eliminates the need for a reserved words list and helps future-proof the spec.

Group keys without a dollar sign ($) prefix denote:

A token name: distinguishable by containing a $value property

{
  "Group of tokens": {
    "$description": "This is an example of a group containing a single token",
    "Token name": {
      "$value": "#000000"
    }
  }
}
A nested group name: distinguishable by not having a $value property

{
  "Group of tokens": {
    "$description": "This is an example of a group containing a nested group",
    "Subgroup of tokens": {
      "Token 1 name": {
        "$value": "#aabbcc"
      },
      "Token 2 name": {
        "$value": "#ddeeff"
      }
    }
  }
}
6.1.1 Description
Groups MAY include an optional $description property, whose value MUST be a plain JSON string. Its purpose is to describe the group itself.

For example:

EXAMPLE 8
{
  "brand": {
    "$description": "Design tokens from our brand guidelines",
    "color": {
      "$description": "Our brand's primary color palette",
      "acid green": {
        "$value": "#00ff66"
      },
      "hot pink": {
        "$value": "#dd22cc"
      }
    }
  }
}
Suggested ways tools MAY use this property are:

A style guide generator could render a section for each group and use the description as an introductory paragraph
A GUI tool that lets users browse or select tokens could display this info alongside the corresponding group or as a tooltip
Translation tools could output this as a source code comment
ISSUE 72: Group & file level properties dtcg-format
Groups may support additional properties like type and description. Should other properties be supported at the group level?

6.1.2 Type
Groups MAY include an optional $type property so a type property does not need to be manually added to every token. See supported "Types" for more information.

If a group has a $type property it acts as a default type for any tokens within the group, including ones in nested groups, that do not explicitly declare a type via their own $type property. For the full set of rules by which a design token's type is determined, please refer to the design token type property chapter.

For example:

EXAMPLE 9
{
  "brand": {
    "$type": "color",
    "color": {
      "acid green": {
        "$value": "#00ff66"
      },
      "hot pink": {
        "$value": "#dd22cc"
      }
    }
  }
}
6.2 Use-cases
6.2.1 File authoring & organization
Groups let token file authors better organize their token files. Related tokens can be nested into groups to align with the team's naming conventions and/or mental model. When manually authoring files, using groups is also less verbose than a flat list of tokens with repeating prefixes.

For example:

EXAMPLE 10
{
  "brand": {
    "color": {
      "$type": "color",
      "acid green": {
        "$value": "#00ff66"
      },
      "hot pink": {
        "$value": "#dd22cc"
      }
    },
    "typeface": {
      "$type": "fontFamily",
      "primary": {
        "$value": "Comic Sans MS"
      },
      "secondary": {
        "$value": "Times New Roman"
      }
    }
  }
}
...is likely to be more convenient to type and, arguably, easier to read, than:

EXAMPLE 11
{
  "brand-color-acid-green": {
    "$value": "#00ff66",
    "$type": "color"
  },
  "brand-color-hot-pink": {
    "$value": "#dd22cc",
    "$type": "color"
  },
  "brand-typeface-primary": {
    "$value": "Comic Sans MS",
    "$type": "fontFamily"
  },
  "brand-typeface-secondary": {
    "$value": "Times New Roman",
    "$type": "fontFamily"
  }
}
6.2.2 GUI tools
Tools that let users pick or edit tokens via a GUI MAY use the grouping structure to display a suitable form of progressive disclosure, such as a collapsible tree view.


Figure 1 Progressive disclosure groups
6.2.3 Translation tools
Token names are not guaranteed to be unique within the same file. The same name can be used in different groups. Also, translation tools MAY need to export design tokens in a uniquely identifiable way, such as variables in code. Translation tools SHOULD therefore use design tokens' paths as these are unique within a file.

For example, a translation tool like Style Dictionary might use the following design token file:

EXAMPLE 12
{
  "brand": {
    "color": {
      "$type": "color",
      "acid green": {
        "$value": "#00ff66"
      },
      "hot pink": {
        "$value": "#dd22cc"
      }
    },
    "typeface": {
      "$type": "fontFamily",
      "primary": {
        "$value": "Comic Sans MS"
      },
      "secondary": {
        "$value": "Times New Roman"
      }
    }
  }
}
...and output it as Sass variables like so by concatenating the path to create variable names:

EXAMPLE 13
$brand-color-acid-green: #00ff66;
$brand-color-hot-pink: #dd22cc;
$brand-typeface-primary: 'Comic Sans MS';
$brand-typeface-secondary: 'Times New Roman';
7. Aliases / references
Instead of having explicit values, tokens can reference the value of another token. To put it another way, a token can be an alias for another token. This spec considers the terms "alias" and "reference" to be synonyms and uses them interchangeably.

Aliases are useful for:

Expressing design choices
Eliminating repetition of values in token files (DRYing up the code)
For a design token to reference another, its value MUST be a string containing the period-separated (.) path to the token it's referencing enclosed in curly brackets.

For example:

EXAMPLE 14
{
  "group name": {
    "token name": {
      "$value": 1234,
      "$type": "number"
    }
  },
  "alias name": {
    "$value": "{group name.token name}"
  }
}
When a tool needs the actual value of a token it MUST resolve the reference - i.e. lookup the token being referenced and fetch its value. In the above example, the "alias name" token's value would resolve to 1234 because it references the token whose path is {group name.token name} which has the value 1234.

Tools SHOULD preserve references and therefore only resolve them whenever the actual value needs to be retrieved. For instance, in a design tool, changes to the value of a token being referenced by aliases SHOULD be reflected wherever those aliases are being used.

Aliases MAY reference other aliases. In this case, tools MUST follow each reference until they find a token with an explicit value. Circular references are not allowed. If a design token file contains circular references, then the value of all tokens in that chain is unknown and an appropriate error or warning message SHOULD be displayed to the user.

EDITOR'S NOTE: JSON Pointer syntax
The format editors are currently researching JSON Pointer syntax to inform the exact syntax for aliases in tokens. https://datatracker.ietf.org/doc/html/rfc6901#section-5

8. Types
Many tools need to know what kind of value a given token represents to process it sensibly. Translation tools MAY need to convert or format tokens differently depending on their type. Design tools MAY present the user with different kinds of input when editing tokens of a certain type (such as color picker, slider, text input, etc.). Style guide generators MAY use different kinds of previews for different types of tokens.

This spec defines a number of design-focused types and every design token MUST use one of these types. Furthermore, that token's value MUST then follow rules and syntax for the chosen type as defined by this spec.

A token's type can be set directly by giving it a $type property specifying the chosen type. Alternatively, it can inherit a type from one of its parent groups, or be an alias of a token that has the desired type.

If no explicit type has been set for a token, tools MUST consider the token invalid and not attempt to infer any other type from the value.

If an explicit type is set, but the value does not match the expected syntax then that token is invalid and an appropriate error SHOULD be displayed to the user. To put it another way, the $type property is a declaration of what kind of values are permissible for the token. (This is similar to typing in programming languages like Java or TypeScript, where a value not compatible with the declared type causes a compilation error).

8.1 Color
Represents a 24bit RGB or 24+8bit RGBA color in the sRGB color space. The $type property MUST be set to the string color. The value MUST be a string containing a hex triplet/quartet including the preceding # character. To support other color spaces, such as HSL, translation tools SHOULD convert color tokens to the equivalent value as needed.

For example, initially the color tokens MAY be defined as such:

EXAMPLE 15
{
  "Majestic magenta": {
    "$value": "#ff00ff",
    "$type": "color"
  },
  "Translucent shadow": {
    "$value": "#00000080",
    "$type": "color"
  }
}
Then, the output from a tool's conversion to HSL(A) MAY look something like:

EXAMPLE 16
// colors-hex.scss
$majestic-magenta: #ff00ff;
$translucent-shadow: #00000080;

// colors-hsl.scss
$majestic-magenta: hsl(300, 100%, 50%);
$translucent-shadow: hsla(300, 100%, 50%, 0.5);
8.2 Dimension
Represents an amount of distance in a single dimension in the UI, such as a position, width, height, radius, or thickness. The $type property MUST be set to the string dimension. The value must be a string containing a number (either integer or floating-point) followed by either a "px" or "rem" unit (future spec iterations may add support for additional units). This includes 0 which also MUST be followed by either a "px" or "rem" unit.

For example:

EXAMPLE 17
{
  "spacing-stack-0": {
    "$value": "0rem",
    "$type": "dimension"
  },
  "spacing-stack-1": {
    "$value": "0.25rem",
    "$type": "dimension"
  }
}
The "px" and "rem" units are to be interpreted the same way they are in CSS:

px: Represents an idealized pixel on the viewport. The equivalent in Android is dp and iOS is pt. Translation tools SHOULD therefore convert to these or other equivalent units as needed.
rem: Represents a multiple of the system's default font size (which MAY be configurable by the user). 1rem is 100% of the default font size. The equivalent of 1rem on Android is 16sp. Not all platforms have an equivalent to rem, so translation tools MAY need to do a lossy conversion to a fixed px size by assuming a default font size (usually 16px) for such platforms.
8.3 Font family
ISSUE 53: Type: font family Typography Type Enhancements
A naive approach like the one below may be appropriate for the first stage of the specification, but this could be more complicated than it seems due to platform/OS/browser restrictions.

Represents a font name or an array of font names (ordered from most to least preferred). The $type property MUST be set to the string fontFamily. The value MUST either be a string value containing a single font name or an array of strings, each being a single font name.

For example:

EXAMPLE 18
{
  "Primary font": {
    "$value": "Comic Sans MS",
    "$type": "fontFamily"
  },
  "Body font": {
    "$value": ["Helvetica", "Arial", "sans-serif"],
    "$type": "fontFamily"
  }
}
8.4 Font weight
Represents a font weight. The $type property MUST be set to the string fontWeight. The value must either be a number value in the range [1, 1000] or one of the pre-defined string values defined in the table below.

Lower numbers represent lighter weights, and higher numbers represent thicker weights, as per the OpenType wght tag specification. The pre-defined string values are aliases for specific numeric values. For example 100, "thin" and "hairline" are all the exact same value.

numeric value	string value aliases
100	thin, hairline
200	extra-light, ultra-light
300	light
400	normal, regular, book
500	medium
600	semi-bold, demi-bold
700	bold
800	extra-bold, ultra-bold
900	black, heavy
950	extra-black, ultra-black
Number values outside of the [1, 1000] range and any other string values, including ones that differ only in case, are invalid and MUST be rejected by tools.

Example:

EXAMPLE 19
{
  "font-weight-default": {
    "$value": 350,
    "$type": "fontWeight"
  },
  "font-weight-thick": {
    "$value": "extra-bold",
    "$type": "fontWeight"
  }
}
8.5 Duration
Represents the length of time in milliseconds an animation or animation cycle takes to complete, such as 200 milliseconds. The $type property MUST be set to the string duration. The value MUST be a string containing a number (either integer or floating-point) followed by an "ms" unit. A millisecond is a unit of time equal to one thousandth of a second.

For example:

EXAMPLE 20
{
  "Duration-100": {
    "$value": "100ms",
    "$type": "duration"
  },
  "Duration-200": {
    "$value": "200ms",
    "$type": "duration"
  }
}
8.6 Cubic Bézier
Represents how the value of an animated property progresses towards completion over the duration of an animation, effectively creating visual effects such as acceleration, deceleration, and bounce. The $type property MUST be set to the string cubicBezier. The value MUST be an array containing four numbers. These numbers represent two points (P1, P2) with one x coordinate and one y coordinate each [P1x, P1y, P2x, P2y]. The y coordinates of P1 and P2 can be any real number in the range [-∞, ∞], but the x coordinates are restricted to the range [0, 1].

For example:

EXAMPLE 21
{
  "Accelerate": {
    "$value": [0.5, 0, 1, 1],
    "$type": "cubicBezier"
  },
  "Decelerate": {
    "$value": [0, 0, 0.5, 1],
    "$type": "cubicBezier"
  }
}
8.7 Number
Represents a number. Numbers can be positive, negative and have fractions. Example uses for number tokens are gradient stop positions or unitless line heights. The $type property MUST be set to the string number. The value MUST be a JSON number value.

EXAMPLE 22
{
  "line-height-large": {
    "$value": 2.3,
    "$type": "number"
  }
}
8.8 Additional types
This section is non-normative.

Types still to be documented here are likely to include:

Font style: might be an enum of allowed values like ("normal", "italic"...)
Percentage/ratio: e.g. for opacity values, relative dimensions, aspect ratios, etc.
Not 100% sure about this since these are really "just" numbers. An alternative might be that we expand the permitted syntax for the "number" type, so for example "1:2", "50%" and 0.5 are all equivalent. People can then use whichever syntax they like best for a given token.
File: for assets - might just be a relative file path / URL (or should we let people also express the mime-type?)
9. Composite types
The types defined in the previous chapters such as color and dimension all have singular values. For example, the value of a color token is one color. However, there are other aspects of UI designs that are a combination of multiple values. For instance, a shadow style is a combination of a color, X & Y offsets, a blur radius and a spread radius.

Every shadow style has the exact same parts (color, X & Y offsets, etc.), but their respective values will differ. Furthermore, each part's value (which is also known as a "sub-value") is always of the same type. A shadow's color must always be a color value, its X offset must always be a dimension value, and so on. Shadow styles are therefore combinations of values that follow a pre-defined structure. In other words, shadow styles are themselves a type. Types like this are called composite types.

Specifically, a composite type has the following characteristics:

Its value is an object or array, potentially containing nested objects or arrays, following a pre-defined structure where the properties of the (nested) object(s) or the elements of the (nested) arrays are sub-values.
Sub-values may be explicit values (e.g. "#ff0000") or references to other design tokens that have sub-value's type (e.g. "{some.other.token}").
A design token whose type happens to be a composite type is sometimes also called a composite (design) token. Besides their type, there is nothing special about composite tokens. They can have all the other additional properties like $description or $extensions. They can also be referenced by other design tokens.

EXAMPLE 23: Composite token example
{
  "shadow-token": {
    "$type": "shadow",
    "$value": {
      "color": "#00000080",
      "offsetX": "0.5rem",
      "offsetY": "0.5rem",
      "blur": "1.5rem",
      "spread": "0rem"
    }
  }
}
EXAMPLE 24: Advanced composite token example
{
  "space": {
    "small": {
      "$type": "dimension",
      "$value": "0.5rem"
    }
  },

  "color": {
    "shadow-050": {
      "$type": "color",
      "$value": "#00000080"
    }
  },

  "shadow": {
    "medium": {
      "$type": "shadow",
      "$description": "A composite token where some sub-values are references to tokens that have the correct type and others are explicit values",
      "$value": {
        "color": "{color.shadow-050}",
        "offsetX": "{space.small}",
        "offsetY": "{space.small}",
        "blur": "1.5rem",
        "spread": "0rem"
      }
    }
  },

  "component": {
    "card": {
      "box-shadow": {
        "$description": "This token is an alias for the composite token {shadow.medium}",
        "$value": "{shadow.medium}"
      }
    }
  }
}
9.1 Groups versus composite tokens
At first glance, groups and composite tokens might look very similar. However, they are intended to solve different problems and therefore have some important differences:

Groups are for arbitrarily grouping tokens for the purposes of naming and/or organization.
They impose no rules or restrictions on how many tokens or nested groups you put within them, what they are called, or what the types of the tokens within should be. As such, tools MUST NOT try to infer any special meaning or typing of tokens based on a group they happen to be in.
Different design systems are likely to group their tokens differently.
You can think of groups as containers that exist "outside" of design tokens.
Composite tokens are individual design tokens whose values are made up of several sub-values.
Since they are design tokens, they can be referenced by other design tokens. This is not true for groups.
Their type must be one of the composite types defined in this specification. Therefore their names and types of their sub-values are pre-defined. Adding additional sub-values or setting values that don't have the correct type make the composite token invalid.
Tools MAY provide specialised functionality for composite tokens. For example, a design tool may let the user pick from a list of all available shadow tokens when applying a drop shadow effect to an element.
9.2 Stroke style
Represents the style applied to lines or borders. The $type property MUST be set to the string strokeStyle. The value MUST be either:

a string value as defined in the corresponding section below, or
an object value as defined in the corresponding section below
ISSUE 98: Stroke style type feedback Composite Type Feedback
Is the current specification for stroke styles fit for purpose? Does it need more sub-values (e.g. equivalents to SVG's stroke-linejoin, stroke-miterlimit and stroke-dashoffset attributes)?
9.2.1 String value
String stroke style values MUST be set to one of the following, pre-defined values:

solid
dashed
dotted
double
groove
ridge
outset
inset
These values have the same meaning as the equivalent "line style" values in CSS. As per the CSS spec, their exact rendering is therefore implementation specific. For example, the length of dashes and gaps in the dashed style may vary between different tools.

EXAMPLE 25: String stroke style example
{
  "focus-ring-style": {
    "$type": "strokeStyle",
    "$value": "dashed"
  }
}
9.2.2 Object value
Object stroke style values MUST have the following properties:

dashArray: An array of dimension values and/or references to dimension tokens, which specify the lengths of alternating dashes and gaps. If an odd number of values is provided, then the list of values is repeated to yield an even number of values.
lineCap: One of the following pre-defined string values: "round", "butt" or "square". These values have the same meaning as those of the stroke-linecap attribute in SVG.
EXAMPLE 26: Object stroke style example
{
  "alert-border-style": {
    "$type": "strokeStyle",
    "$value": {
      "dashArray": ["0.5rem", "0.25rem"],
      "lineCap": "round"
    }
  }
}
EXAMPLE 27: Object stroke style example using references
{
  "notification-border-style": {
    "$type": "strokeStyle",
    "$value": {
      "dashArray": ["{dash-length-medium}", "0.25rem"],
      "lineCap": "butt"
    }
  },

  "dash-length-medium": {
    "$type": "dimension",
    "$value": "10px"
  }
}
9.2.3 Fallbacks
The string and object values are mutually exclusive means of expressing stroke styles. For example, some of the string values like inset or groove cannot be expressed in terms of a dashArray and lineCap as they require some implementation-specific means of lightening or darkening the color for portions of a border or outline. Conversely, a precisely defined combination of dashArray and lineCap sub-values is not guaranteed to produce the same visual result as the dashed or dotted keywords as they are implementation-specific.

Furthermore, some tools and platforms may not support the full range of stroke styles that design tokens of this type can represent. When displaying or exporting a strokeStyle token whose value they don't natively support, they should therefore fallback to the closest approximation that they do support.

The specifics of how a "closest approximation" is chosen are implementation-specific. However, the following examples illustrate what fallbacks tools MAY use in some scenarios.

EXAMPLE 28: Fallback for object stroke style
CSS does not allow detailed control of the dash pattern or line caps on dashed borders. So, a tool exporting the "notification-border-style" design token from the example in the previous section, might use the CSS dashed line style as a fallback:

:root {
  --notification-border-style: dashed;
}
EXAMPLE 29: Fallback for string stroke style
Some design tools like Figma don't support inset, outset or double style lines. When a user applies a stroke-style token with those values, such tools might therefore fallback to displaying a solid line instead.

9.3 Border
Represents a border style. The $type property MUST be set to the string border. The value MUST be an object with the following properties:

color: The color of the border. The value of this property MUST be a valid color value or a reference to a color token.
width: The width or thickness of the border. The value of this property MUST be a valid dimension value or a reference to a dimension token.
style: The border's style. The value of this property MUST be a valid stroke style value or a reference to a stroke style token.
EXAMPLE 30: Border composite token examples
{
  "border": {
    "heavy": {
      "$type": "border",
      "$value": {
        "color": "#36363600",
        "width": "3px",
        "style": "solid"
      }
    },
    "focusring": {
      "$type": "border",
      "$value": {
        "color": "{color.focusring}",
        "width": "1px",
        "style": {
          "dashArray": ["0.5rem", "0.25rem"],
          "lineCap": "round"
        }
      }
    }
  }
}
ISSUE 99: Border type feedback Composite Type Feedback
Is the current specification for borders fit for purpose? Does it need more sub-values to account for features like outset, border images, multiple borders, etc. that some platforms and design tools have?
9.4 Transition
Represents a animated transition between two states. The $type property MUST be set to the string transition. The value MUST be an object with the following properties:

duration: The duration of the transition. The value of this property MUST be a valid duration value or a reference to a duration token.
delay: The time to wait before the transition begins. The value of this property MUST be a valid duration value or a reference to a duration token.
timingFunction: The timing function of the transition. The value of this property MUST be a valid cubic bézier value or a reference to a cubic bézier token.
EXAMPLE 31: Transition composite token examples
{
  "transition": {
    "emphasis": {
      "$type": "transition",
      "$value": {
        "duration": "200ms",
        "delay": "0ms",
        "timingFunction": [0.5, 0, 1, 1]
      }
    }
  }
}
ISSUE 103: Transition type feedback Composite Type Feedback
Is the current specification for transitions fit for purpose? Are these transitions parameters by themselves useful considering that they don't let you specify what aspect of a UI is being transitioned and what the start and end states are?
9.5 Shadow
Represents a shadow style. The $type property MUST be set to the string shadow. The value must be an object with the following properties:

color: The color of the shadow. The value of this property MUST be a valid color value or a reference to a color token.
offsetX: The horizontal offset that shadow has from the element it is applied to. The value of this property MUST be a valid dimension value or a reference to a dimension token.
offsetY: The vertical offset that shadow has from the element it is applied to. The value of this property MUST be a valid dimension value or a reference to a dimension token.
blur: The blur radius that is applied to the shadow. The value of this property MUST be a valid dimension value or a reference to a dimension token.
spread: The amount by which to expand or contract the shadow. The value of this property MUST be a valid dimension value or a reference to a dimension token.
EXAMPLE 32: Shadow token example
{
  "shadow-token": {
    "$type": "shadow",
    "$value": {
      "color": "#00000080",
      "offsetX": "0.5rem",
      "offsetY": "0.5rem",
      "blur": "1.5rem",
      "spread": "0rem"
    }
  }
}
ISSUE 100: Shadow type feedback Composite Type Feedback
Is the current specification for shadows fit for purpose? Does it need to support multiple shadows, as some tools and platforms do?
9.6 Gradient
Represents a color gradient. The $type property MUST be set to the string gradient. The value MUST be an array of objects representing gradient stops that have the following structure:

color: The color value at the stop's position on the gradient. The value of this property MUST be a valid color value or a reference to a color token.
position: The position of the stop along the gradient's axis. The value of this property MUST be a valid number value or reference to a number token. The number values must be in the range [0, 1], where 0 represents the start position of the gradient's axis and 1 the end position. If a number value outside of that range is given, it MUST be considered as if it were clamped to the range [0, 1]. For example, a value of 42 should be treated as if it were 1, i.e. the end position of the gradient axis. Similarly, a value of -99 should be treated as if it were 0, i.e. the start position of the gradient axis.
If there are no stops at the very beginning or end of the gradient axis (i.e. with position 0 or 1, respectively), then the color from the stop closest to each end should be extended to that end of the axis.

EXAMPLE 33: Gradient token example
{
  "blue-to-red": {
    "$type": "gradient",
    "$value": [
      {
        "color": "#0000ff",
        "position": 0
      },
      {
        "color": "#ff0000",
        "position": 1
      }
    ]
  }
}
Describes a gradient that goes from blue to red:

EXAMPLE 34: Gradient token with omitted start stop example
{
  "mostly-yellow": {
    "$type": "gradient",
    "$value": [
      {
        "color": "#ffff00",
        "position": 0.666
      },
      {
        "color": "#ff0000",
        "position": 1
      }
    ]
  }
}
Describes a gradient that is solid yellow for the first 2/3 and then fades to red:

EXAMPLE 35: Gradient token using references example
{
  "brand-primary": {
    "$type": "color",
    "$value": "#99ff66"
  },

  "position-end": {
    "$type": "number",
    "$value": 1
  },

  "brand-in-the-middle": {
    "$type": "gradient",
    "$value": [
      {
        "color": "#000000",
        "position": 0
      },
      {
        "color": "{brand-primary}",
        "position": 0.5
      },
      {
        "color": "#000000",
        "position": "{position-end}"
      }
    ]
  }
}
Describes a color token called "brand-primary", which is referenced as the mid-point of a gradient is black at either end.

ISSUE 101: Gradient type feedback Color Type Enhancements
Is the current specification for gradients fit for purpose? Does it need to also specify the type of gradient (.e.g linear, radial, conical, etc.)?
9.7 Typography
Represents a typographic style. The $type property MUST be set to the string typography. The value MUST be an object with the following properties:

fontFamily: The typography's font. The value of this property MUST be a valid font family value or a reference to a font family token.
fontSize: The size of the typography. The value of this property MUST be a valid dimension value or a reference to a dimension token.
fontWeight: The weight of the typography. The value of this property MUST be a valid font weight or a reference to a font weight token.
letterSpacing: The horizontal spacing between characters. The value of this property MUST be a valid dimension value or a reference to a dimension token.
lineHeight: The vertical spacing between lines of typography. The value of this property MUST be a valid number value or a reference to a number token. The number SHOULD be interpreted as a multiplier of the fontSize.
EXAMPLE 36: Typography composite token examples
{
  "type styles": {
    "heading-level-1": {
      "$type": "typography",
      "$value": {
        "fontFamily": "Roboto",
        "fontSize": "42px",
        "fontWeight": "700",
        "letterSpacing": "0.1px",
        "lineHeight": 1.2
      }
    },
    "microcopy": {
      "$type": "typography",
      "$value": {
        "fontFamily": "{font.serif}",
        "fontSize": "{font.size.smallest}",
        "fontWeight": "{font.weight.normal}",
        "letterSpacing": "0px",
        "lineHeight": 1
      }
    }
  }
}
ISSUE 102: Typography type feedback Typography Type Enhancements
Is the current specification for typography styles fit for purpose? Should the lineHeight sub-value use a number value, dimension or a new lineHeight type?

A. Issue summary
Issue 72: Group & file level propertiesIssue 53: Type: font familyIssue 98: Stroke style type feedbackIssue 99: Border type feedbackIssue 103: Transition type feedbackIssue 100: Shadow type feedbackIssue 101: Gradient type feedbackIssue 102: Typography type feedback
B. References
B.1 Normative references
[RFC2119]
Key words for use in RFCs to Indicate Requirement Levels. S. Bradner. IETF. March 1997. Best Current Practice. URL: https://www.rfc-editor.org/rfc/rfc2119
[RFC8174]
Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words. B. Leiba. IETF. May 2017. Best Current Practice. URL: https://www.rfc-editor.org/rfc/rfc8174
↑