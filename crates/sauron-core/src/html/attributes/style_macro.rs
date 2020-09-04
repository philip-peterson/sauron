/// a utility function for convenient styling of elements
#[macro_export]
macro_rules! style {
    ($($tokens:tt)+) => {
        {
            let json = serde_json::json!({$($tokens)*});
            println!("json: {}", json.to_string());
            let mut styles = vec![];
            if let Some(style_properties) = json.as_object(){
                for (prop,p_value) in style_properties.iter(){
                    //TODO: change the type of style name to &'a str, so we don't
                    //have to lookup the &'static str equivalent
                    let style_name = $crate::html::attributes::HTML_STYLES
                        .iter()
                        .find(|name| name == &prop)
                        .map(|name| *name)
                        .expect("style name must be valid");

                    let value = match p_value{
                        serde_json::Value::String(s) => $crate::html::attributes::Value::String(s.to_string()),
                        serde_json::Value::Number(v) => {
                            if let Some(i64_value) = v.as_i64(){
                                $crate::html::attributes::Value::I64(i64_value)
                            }else if let Some(f64_value) = v.as_f64(){
                                $crate::html::attributes::Value::F64(f64_value)
                            }else{
                                unreachable!("should either be i64 or f64")
                            }
                        }
                        serde_json::Value::Bool(v) => $crate::html::attributes::Value::Bool(*v),
                        _ => panic!("supported values are String, Number or Bool only"),
                    };
                    println!("value: {}", value);
                    styles.push($crate::html::attributes::style::Style::new(style_name, value));

                }
            }
            $crate::mt_dom::attr(
                "style",
                $crate::html::attributes::AttributeValue::from_styles(styles),
            )
        }
    };
}

/// HTML style names
//#[cfg(feature = "with-parser")]
pub const HTML_STYLES: [&'static str; 368] = [
    "align-content",
    "align-items",
    "align-self",
    "animation",
    "animation-delay",
    "animation-direction",
    "animation-duration",
    "animation-fill-mode",
    "animation-iteration-count",
    "animation-name",
    "animation-play-state",
    "animation-timing-function",
    "backdrop-filter",
    "backface-visibility",
    "background",
    "background-attachment",
    "background-blend-mode",
    "background-clip",
    "background-color",
    "background-image",
    "background-origin",
    "background-position",
    "background-repeat",
    "background-size",
    "block-size",
    "border",
    "border-block",
    "border-block-color",
    "border-block-end",
    "border-block-end-color",
    "border-block-end-style",
    "border-block-end-width",
    "border-block-start",
    "border-block-start-color",
    "border-block-start-style",
    "border-block-start-width",
    "border-block-style",
    "border-block-width",
    "border-bottom",
    "border-bottom-color",
    "border-bottom-left-radius",
    "border-bottom-right-radius",
    "border-bottom-style",
    "border-bottom-width",
    "border-collapse",
    "border-color",
    "border-end-end-radius",
    "border-end-start-radius",
    "border-image",
    "border-image-outset",
    "border-image-repeat",
    "border-image-slice",
    "border-image-source",
    "border-image-width",
    "border-inline",
    "border-inline-color",
    "border-inline-end",
    "border-inline-end-color",
    "border-inline-end-style",
    "border-inline-end-width",
    "border-inline-start",
    "border-inline-start-color",
    "border-inline-start-style",
    "border-inline-start-width",
    "border-inline-style",
    "border-inline-width",
    "border-left",
    "border-left-color",
    "border-left-style",
    "border-left-width",
    "border-radius",
    "border-right",
    "border-right-color",
    "border-right-style",
    "border-right-width",
    "border-spacing",
    "border-start-end-radius",
    "border-start-start-radius",
    "border-style",
    "border-top",
    "border-top-color",
    "border-top-left-radius",
    "border-top-right-radius",
    "border-top-style",
    "border-top-width",
    "border-width",
    "bottom",
    "box-decoration-break",
    "box-shadow",
    "box-sizing",
    "break-after",
    "break-before",
    "break-inside",
    "caption-side",
    "caret-color",
    "clear",
    "clip",
    "clip-path",
    "color",
    "color-adjust",
    "column-count",
    "column-fill",
    "column-gap",
    "column-rule",
    "column-rule-color",
    "column-rule-style",
    "column-rule-width",
    "column-span",
    "column-width",
    "columns",
    "contain",
    "content",
    "counter-increment",
    "counter-reset",
    "counter-set",
    "cursor",
    "direction",
    "display",
    "empty-cells",
    "filter",
    "flex",
    "flex-basis",
    "flex-direction",
    "flex-flow",
    "flex-grow",
    "flex-shrink",
    "flex-wrap",
    "float",
    "font",
    "font-family",
    "font-feature-settings",
    "font-kerning",
    "font-language-override",
    "font-optical-sizing",
    "font-size",
    "font-size-adjust",
    "font-stretch",
    "font-style",
    "font-synthesis",
    "font-variant",
    "font-variant-alternates",
    "font-variant-caps",
    "font-variant-east-asian",
    "font-variant-ligatures",
    "font-variant-numeric",
    "font-variant-position",
    "font-variation-settings",
    "font-weight",
    "gap",
    "grad",
    "grid",
    "grid-area",
    "grid-auto-columns",
    "grid-auto-flow",
    "grid-auto-rows",
    "grid-column",
    "grid-column-end",
    "grid-column-start",
    "grid-row",
    "grid-row-end",
    "grid-row-start",
    "grid-template",
    "grid-template-areas",
    "grid-template-columns",
    "grid-template-rows",
    "hanging-punctuation",
    "height",
    "hyphens",
    "image-orientation",
    "image-rendering",
    "inherit",
    "initial",
    "inline-size",
    "inset",
    "inset-block",
    "inset-block-end",
    "inset-block-start",
    "inset-inline",
    "inset-inline-end",
    "inset-inline-start",
    "isolation",
    "justify-content",
    "justify-items",
    "justify-self",
    "left",
    "letter-spacing",
    "line-break",
    "line-height",
    "list-style",
    "list-style-image",
    "list-style-position",
    "list-style-type",
    "margin",
    "margin-block",
    "margin-block-end",
    "margin-block-start",
    "margin-bottom",
    "margin-inline",
    "margin-inline-end",
    "margin-inline-start",
    "margin-left",
    "margin-right",
    "margin-top",
    "mask",
    "mask-border",
    "mask-border-mode",
    "mask-border-outset",
    "mask-border-repeat",
    "mask-border-slice",
    "mask-border-source",
    "mask-border-width",
    "mask-clip",
    "mask-composite",
    "mask-image",
    "mask-mode",
    "mask-origin",
    "mask-position",
    "mask-repeat",
    "mask-size",
    "mask-type",
    "max-block-size",
    "max-height",
    "max-inline-size",
    "max-width",
    "min-block-size",
    "min-height",
    "min-inline-size",
    "min-width",
    "mix-blend-mode",
    "object-fit",
    "object-position",
    "offset",
    "offset-anchor",
    "offset-distance",
    "offset-path",
    "offset-rotate",
    "opacity",
    "order",
    "orphans",
    "outline",
    "outline-color",
    "outline-offset",
    "outline-style",
    "outline-width",
    "overflow",
    "overflow-anchor",
    "overflow-block",
    "overflow-inline",
    "overflow-wrap",
    "overflow-x",
    "overflow-y",
    "overscroll-behavior",
    "overscroll-behavior-block",
    "overscroll-behavior-inline",
    "overscroll-behavior-x",
    "overscroll-behavior-y",
    "padding",
    "padding-block",
    "padding-block-end",
    "padding-block-start",
    "padding-bottom",
    "padding-inline",
    "padding-inline-end",
    "padding-inline-start",
    "padding-left",
    "padding-right",
    "padding-top",
    "page-break-after",
    "page-break-before",
    "page-break-inside",
    "paint-order",
    "perspective",
    "perspective-origin",
    "place-content",
    "place-items",
    "place-self",
    "pointer-events",
    "position",
    "quotes",
    "resize",
    "revert",
    "right",
    "rotate",
    "row-gap",
    "scale",
    "scroll-behavior",
    "scroll-margin",
    "scroll-margin-block",
    "scroll-margin-block-end",
    "scroll-margin-block-start",
    "scroll-margin-bottom",
    "scroll-margin-inline",
    "scroll-margin-inline-end",
    "scroll-margin-inline-start",
    "scroll-margin-left",
    "scroll-margin-right",
    "scroll-margin-top",
    "scroll-padding",
    "scroll-padding-block",
    "scroll-padding-block-end",
    "scroll-padding-block-start",
    "scroll-padding-bottom",
    "scroll-padding-inline",
    "scroll-padding-inline-end",
    "scroll-padding-inline-start",
    "scroll-padding-left",
    "scroll-padding-right",
    "scroll-padding-top",
    "scroll-snap-align",
    "scroll-snap-stop",
    "scroll-snap-type",
    "scrollbar-color",
    "scrollbar-width",
    "shape-image-threshold",
    "shape-margin",
    "shape-outside",
    "tab-size",
    "table-layout",
    "text-align",
    "text-align-last",
    "text-combine-upright",
    "text-decoration",
    "text-decoration-color",
    "text-decoration-line",
    "text-decoration-skip-ink",
    "text-decoration-style",
    "text-decoration-thickness",
    "text-emphasis",
    "text-emphasis-color",
    "text-emphasis-position",
    "text-emphasis-style",
    "text-indent",
    "text-justify",
    "text-orientation",
    "text-overflow",
    "text-rendering",
    "text-shadow",
    "text-transform",
    "text-underline-offset",
    "text-underline-position",
    "top",
    "touch-action",
    "transform",
    "transform-box",
    "transform-origin",
    "transform-style",
    "transition",
    "transition-delay",
    "transition-duration",
    "transition-property",
    "transition-timing-function",
    "translate",
    "turn",
    "unicode-bidi",
    "unset",
    "vertical-align",
    "visibility",
    "vmax",
    "vmin",
    "white-space",
    "widows",
    "width",
    "will-change",
    "word-break",
    "word-spacing",
    "word-wrap",
    "writing-mode",
    "z-index",
];

#[cfg(test)]
mod tests {
    use crate::{
        html::{units::px, *},
        Render,
    };

    #[test]
    fn test_style_json() {
        let mut b1 = String::new();
        let s1: Attribute<()> = style! {
            "background-color": "green",
            "border": "1px solid green",
        };

        s1.render(&mut b1).unwrap();

        assert_eq!(
            b1,
            r#"style="background-color:green;border:1px solid green;""#
        );
    }

    #[test]
    fn test_style_values() {
        let mut b1 = String::new();
        let s1: Attribute<()> = style! {"width": px(10), "height" : px(20)};
        s1.render(&mut b1).unwrap();

        assert_eq!(b1, r#"style="width:10px;height:20px;""#);

        let s2: Attribute<()> = style! {
            "background-color":"red",
            "width": px(10),
            "height": px(20),
        };

        let mut b2 = String::new();
        s2.render(&mut b2).unwrap();
        println!("s2: {:#?}", s2);

        assert_eq!(
            b2,
            r#"style="background-color:red;width:10px;height:20px;""#
        );

        struct Data {
            width: i32,
        }

        let data = Data { width: 101 };
        let padding_width = 200;

        let mut b3 = String::new();

        let s3: Attribute<()> = style! {
            "width": px(data.width),
            "padding-left": px(padding_width),
            "padding-right": px(padding_width),
        };
        s3.render(&mut b3).unwrap();
        assert_eq!(
            b3,
            r#"style="width:101px;padding-left:200px;padding-right:200px;""#
        );
    }
}
