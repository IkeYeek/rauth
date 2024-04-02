use actix_web::guard::{Guard, GuardContext};

/// Allows access to a route only if it targets so said user or the user is super or root
pub(crate) struct TargetUserGuard;

impl Guard for TargetUserGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        eprintln!("TargetUserGuarded");
        true
    }
}

pub(crate) struct SuperUserGuard;

impl Guard for SuperUserGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        eprintln!("SuperUserGuarded");
        true
    }
}