//! Prelude
pub use embedded_hal::prelude::*;

#[cfg(feature = "adc")]
pub use crate::adc::AdcExt as _stm32h7xx_hal_adc_AdcExt;
#[cfg(feature = "can")]
#[cfg_attr(docsrs, doc(cfg(feature = "can")))]
pub use crate::can::CanExt as _stm32h7xx_hal_can_CanExt;
#[cfg(feature = "crc")]
#[cfg_attr(docsrs, doc(cfg(feature = "crc")))]
pub use crate::crc::CrcExt as _stm32h7xx_hal_crc_CrcExt;
#[cfg(feature = "dac")]
pub use crate::dac::DacExt as _stm32h7xx_hal_dac_DacExt;
#[cfg(feature = "delay")]
pub use crate::delay::DelayExt as _stm32h7xx_hal_delay_DelayExt;
#[cfg(feature = "exti")]
pub use crate::exti::ExtiExt as _stm32h7xx_hal_delay_ExtiExt;
//#[cfg(not(feature = "certified_subset"))]
//pub use crate::flash::FlashExt as _stm32h7xx_hal_flash_FlashExt;
#[cfg(feature = "fmc")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmc")))]
pub use crate::fmc::FmcExt as _stm32h7xx_hal_fmc_FmcExt;
#[cfg(feature = "gpio")]
pub use crate::gpio::GpioExt as _stm32h7xx_hal_gpio_GpioExt;
#[cfg(feature = "i2c")]
pub use crate::i2c::I2cExt as _stm32h7xx_hal_i2c_I2cExt;
#[cfg(all(not(feature = "certified_subset"), feature = "pwm"))]
pub use crate::pwm::PwmAdvExt as _stm32_hal_pwm_PwmAdvExt;
#[cfg(all(not(feature = "certified_subset"), feature = "pwm"))]
pub use crate::pwm::PwmExt as _stm32_hal_pwm_PwmExt;
pub use crate::pwr::PwrExt as _stm32h7xx_hal_pwr_PwrExt;
pub use crate::rcc::RccExt as _stm32h7xx_hal_rcc_RccExt;
#[cfg(feature = "rng")]
pub use crate::rng::RngCore as _stm32h7xx_hal_rng_RngCore;
#[cfg(feature = "rng")]
pub use crate::rng::RngExt as _stm32h7xx_hal_rng_RngExt;
#[cfg(all(not(feature = "certified_subset"), feature = "sai"))]
pub use crate::sai::SaiDmaExt as _stm32h7xx_hal_spi_SaiDmaExt;
#[cfg(all(not(feature = "certified_subset"), feature = "sai"))]
pub use crate::sai::SaiI2sExt as _stm32h7xx_hal_spi_SaiI2sExt;
#[cfg(all(not(feature = "certified_subset"), feature = "sai"))]
pub use crate::sai::SaiPdmExt as _stm32h7xx_hal_spi_SaiPdmExt;
#[cfg(feature = "sdmmc")]
#[cfg_attr(docsrs, doc(cfg(feature = "sdmmc")))]
pub use crate::sdmmc::SdmmcExt as _stm32h7xx_hal_sdmmc_SdmmcExt;
#[cfg(feature = "serial")]
pub use crate::serial::SerialExt as _stm32h7xx_hal_serial_SerialExt;
#[cfg(feature = "spi")]
pub use crate::spi::HalDisabledSpi as _stm32h7xx_hal_spi_HalDisabledSpi;
#[cfg(feature = "spi")]
pub use crate::spi::HalEnabledSpi as _stm32h7xx_hal_spi_HalEnabledSpi;
#[cfg(feature = "spi")]
pub use crate::spi::HalSpi as _stm32h7xx_hal_spi_HalSpi;
#[cfg(feature = "spi")]
pub use crate::spi::SpiExt as _stm32h7xx_hal_spi_SpiExt;
pub use crate::time::U32Ext as _stm32h7xx_hal_time_U32Ext;
#[cfg(feature = "timer")]
pub use crate::timer::HalDisabledLpTimer as _stm32h7xx_hal_timer_HalDisabledLpTimer;
#[cfg(feature = "timer")]
pub use crate::timer::HalEnabledLpTimer as _stm32h7xx_hal_timer_HalEnabledLpTimer;
#[cfg(feature = "timer")]
pub use crate::timer::HalLpTimer as _stm32h7xx_hal_timer_HalLpTimer;
#[cfg(feature = "timer")]
pub use crate::timer::TimerExt as _stm32h7xx_hal_timer_TimerExt;
#[cfg(feature = "xspi")]
#[cfg_attr(docsrs, doc(cfg(feature = "xspi")))]
pub use crate::xspi::XspiExt as _stm32h7xx_hal_xspi_XspiExt;
pub use fugit::{ExtU32 as _, RateExtU32 as _};
