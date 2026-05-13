use clerk::ClerkError;

#[derive(Debug, thiserror::Error)]
pub enum KioyuError {
    #[error(transparent)]
    Clerk(#[from] ClerkError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
