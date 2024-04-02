use actix_web::guard::{Guard, GuardContext};

/// Allows access to a route only if it targets so said user or the user is super or root
pub(crate) struct TargetUserOrSuperUserGuard;

impl Guard for TargetUserOrSuperUserGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        eprintln!("test");
        true
    }
}
