use sndv_scalpel::error::ScalpelError;
use sndv_scalpel::run;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        let is_no_match = err
            .downcast_ref::<ScalpelError>()
            .map(|e| matches!(e, ScalpelError::NoMatch { .. } | ScalpelError::NoMatchFound { .. }))
            .unwrap_or(false);
        eprintln!("error: {err}");
        std::process::exit(if is_no_match { 1 } else { 2 });
    }
}
