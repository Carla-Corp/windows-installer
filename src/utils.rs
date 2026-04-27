use druid::*;
use widget::*;

#[cfg(target_os = "windows")]
pub fn beep() {
    use winapi::um::winuser::*;
    unsafe {
        MessageBeep(MB_OK);
    }
}

pub struct ProgressController;
impl<W: Widget<f64>> Controller<f64, W> for ProgressController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        data: &f64,
        old_data: &f64,
        env: &Env,
    ) {
        if old_data != data {
            ctx.request_paint();
        }
        child.update(ctx, old_data, data, env);
    }
}
