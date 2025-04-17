mod book;
mod comments;
mod file_size;
mod histogram;
mod press_intervals;
mod utils;

use book::book;
use comments::comments;
use file_size::file_size;
use press_intervals::press_intervals;
use utils::int_to_base;

pub fn main() {
    press_intervals();
    file_size();
    comments();
    book();
}
