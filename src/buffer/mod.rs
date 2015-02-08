pub use self::buffer::new;
pub use self::buffer::from_file;
pub use self::buffer::Buffer;
pub use self::gap_buffer::GapBuffer;
pub use self::position::Position;
pub use self::range::Range;

mod buffer;
pub mod gap_buffer;
mod position;
mod range;
