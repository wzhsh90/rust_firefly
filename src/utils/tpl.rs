use handlebars::{
    Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use chrono::prelude::Local;

lazy_static! {
        pub static ref HTML_TPL:Handlebars<'static>=init_tpl();
        static ref LOOK_UP_TIME:i64= Local::now().timestamp_millis();
}
// default format helper
fn lookup_path_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for LookupPath helper."))?;
    let rendered = format!("{}?v={}", param.value().render(), *LOOK_UP_TIME);
    out.write(rendered.as_ref())?;
    Ok(())
}
fn init_tpl() -> Handlebars<'static> {
    let mut h = Handlebars::new();
    h.register_templates_directory(".html", "./templates")
        .unwrap();
    h.register_helper("lookup_path", Box::new(lookup_path_helper));
    return h;
}
