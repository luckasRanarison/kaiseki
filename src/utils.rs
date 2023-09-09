use bincode::config::{self, Configuration};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BINCODE_CONFIG: Configuration = config::standard();
}
