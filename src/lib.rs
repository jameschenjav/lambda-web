mod request;
pub use lamedh_runtime::Error as LambdaError;

#[cfg(feature = "actix4")]
mod actix4;
#[cfg(feature = "actix4")]
pub use actix4::run_actix_on_lambda;
#[cfg(feature = "actix4")]
pub use actix_web;

#[cfg(feature = "warp03")]
mod warp03;
#[cfg(feature = "warp03")]
pub use warp;
#[cfg(feature = "warp03")]
pub use warp03::run_warp_on_lambda;

/// Returns true if it is running on AWS Lambda
pub fn is_running_on_lambda() -> bool {
    std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok()
}
