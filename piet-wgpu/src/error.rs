use thiserror::Error;

pub type Result<T> = std::result::Result<T, PietWgpuError>;

#[derive(Debug, Error)]
pub enum PietWgpuError {
    #[error("Error in wgpu pipeline")]
    Pipeline(#[from] wgpu::Error),
}
