use std::borrow::Cow;
use std::ops::Deref;

use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> &str;
    fn xmlns(&self) -> &str;
    fn child_elements(&self) -> Element;
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
}

/// Icon component Props
#[derive(PartialEq, Props, Clone)]
pub struct IconProps<T: IconShape + Clone + PartialEq + 'static> {
    /// The icon shape to use.
    pub icon: T,

    /// The height of the `<svg>` element
    #[props(into)]
    pub height: Option<Cow<'static, str>>,

    /// The width of the `<svg>` element
    #[props(into)]
    pub width: Option<Cow<'static, str>>,

    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor".into())]
    pub fill: Cow<'static, str>,

    /// An class for the `<svg>` element.
    #[props(into)]
    pub class: Option<Cow<'static, str>>,

    /// The style of the `<svg>` element.
    #[props(into)]
    pub style: Option<Cow<'static, str>>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    let IconProps {
        icon,
        height,
        width,
        fill,
        class,
        style,
    } = props;

    let (fill, stroke, stroke_width) = icon.fill_and_stroke(&fill);

    rsx!(
        svg {
            class: if let Some(class) = &class {
                class.deref()
            },
            style: if let Some(style) = &style {
                style.deref()
            },
            height: if let Some(height) = &height {
                height.deref()
            },
            width: if let Some(width) = &width {
                width.deref()
            },
            view_box: "{icon.view_box()}",
            xmlns: "{icon.xmlns()}",
            fill,
            stroke,
            stroke_width,
            stroke_linecap: "{icon.stroke_linecap()}",
            stroke_linejoin: "{icon.stroke_linejoin()}",
            {icon.child_elements()}
        }
    )
}
