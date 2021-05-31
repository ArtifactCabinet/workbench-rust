use druid::{AppLauncher, Color, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{Label, Flex, Padding};

// TODO:
// + custom text color
// + Whole flex node text color

fn build_ui() -> impl Widget<()> {
Padding::new(
    1.0,
    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top left").background(Color::rgb(255.0,255.0, 255.0)).center(), 1.0)
                .with_flex_child(Label::new("bottom left").center(), 1.0),
            1.0)
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top right").center(), 1.0)
                .with_flex_child(Label::new("bottom right").center(), 1.0),
            1.0)
        )
}


fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(())?;
    Ok(())
}