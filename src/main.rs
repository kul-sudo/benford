mod audio;
mod book;
mod channels;
mod comments;
mod confucius;
mod countries;
mod file_size;
mod histogram;
mod population;
mod power;
mod press_intervals;
mod two_numbers;
mod utils;

use audio::audio;
use book::book;
use channels::channels;
use comments::comments;
use confucius::confucius;
use countries::countries;
use file_size::file_size;
use population::population;
use power::power;
use press_intervals::press_intervals;
use two_numbers::two_numbers;
use utils::int_to_base;

pub fn main() {
    press_intervals();
    file_size();
    comments();
    book();
    confucius();
    two_numbers();
    power();
    countries();
    population();
    audio();
    channels();
}
