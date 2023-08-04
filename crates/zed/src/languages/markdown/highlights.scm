; Not classified by the grammar:
; - Strikethrough
; - Tables
; - Checklists ([x] and [ ]) (gfm)
; - @mentions (gfm)
; - TeX(LaTeX)
; - Sequence Diagrams
; - FlowChart
;
; Not all of these are vanilla Markdown, but they are commonly used in markdown documents.

; Incorrectly classified by the grammar:
; - Frontmatter delimiter (---)

; Supported by the grammer but we haven't adapted:
; - Reference Links
; - HTML Tags

; Emphasis

(emphasis) @emphasis
(strong_emphasis) @emphasis.strong

; Headings

[
  (atx_heading)
  (setext_heading)
] @title

[
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
  (setext_h1_underline)
  (setext_h2_underline)
] @punctuation.title

; Prose

[
    (paragraph)
] @prose

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
] @punctuation.list_marker

(block_quote
  (block_quote_marker) @punctuation.block_quote
  (paragraph) @prose.block_quote
)

(thematic_break) @punctuation

; Code

(code_span) @text.literal

(fenced_code_block
  (info_string
    (language) @text.literal)
)

(image
  (image_description) @image_description
  (link_destination) @image_uri
)

; Links

(shortcut_link
  [
    "["
    "]"
  ] @punctuation.link
)

(uri_autolink) @link_uri
(link_destination) @link_uri
(link_text) @link_text

; Special

(backslash_escape) @string.escape
(entity_reference) @string.special
