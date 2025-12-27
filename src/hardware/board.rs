use embassy_rp::config::Config;
use embassy_rp::{
    clocks::RoscRng,
    gpio::{Input, Level, Output, Pull},
    spi,
    spi::Spi,
};

const SPI_FREQUENCY: u32 = 40_000_000;

pub struct Board {
    pub spi: Spi<'static, embassy_rp::peripherals::SPI0, spi::Async>,
    pub cs: Output<'static>,
    pub w5500_int: Input<'static>,
    pub w5500_reset: Output<'static>,
    pub rng: RoscRng,
}

pub fn init() -> (Board, Output<'static>) {
    let p = embassy_rp::init(Config::default());

    (
        Board {
            spi: Spi::new(
                p.SPI0,
                p.PIN_18,
                p.PIN_19,
                p.PIN_16,
                p.DMA_CH0,
                p.DMA_CH1,
                spi_config(),
            ),
            cs: Output::new(p.PIN_17, Level::High),
            w5500_int: Input::new(p.PIN_21, Pull::Up),
            w5500_reset: Output::new(p.PIN_20, Level::High),
            rng: RoscRng,
        },
        Output::new(p.PIN_25, Level::Low),
    )
}

fn spi_config() -> spi::Config {
    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = SPI_FREQUENCY;
    spi_cfg
}
