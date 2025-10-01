use crate::{Command, Error, Register, ADDR};

impl_common!(Max17043);
impl_common!(Max17044);

#[cfg(not(feature = "async"))]
macro_rules! impl_common_4x {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Get state of charge of the cell as calculated by the ModelGauge
            /// algorithm as a percentage.
            pub fn soc(&mut self) -> Result<f32, Error<E>> {
                let soc = self.read_register(Register::SOC)?;
                Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
            }

            /// Software reset
            pub fn reset(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::COMMAND, Command::POR_43_44)
            }
        }
    };
}

#[cfg(feature = "async")]
macro_rules! impl_common_4x {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: embedded_hal_async::i2c::I2c<Error = E>,
        {
            /// Get state of charge of the cell as calculated by the ModelGauge
            /// algorithm as a percentage.
            pub async fn soc(&mut self) -> Result<f32, Error<E>> {
                let soc = self.read_register(Register::SOC).await?;
                Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
            }

            /// Software reset
            pub async fn reset(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::COMMAND, Command::POR_43_44)
                    .await
            }
        }
    };
}

impl_common_4x!(Max17043);
impl_common_4x!(Max17044);

#[cfg(not(feature = "async"))]
impl<I2C, E> Max17043<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Get battery voltage
    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL)?;
        Ok(f32::from(vcell >> 4) / 800.0)
    }
}

#[cfg(feature = "async")]
impl<I2C, E> Max17043<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    /// Get battery voltage
    pub async fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL).await?;
        Ok(f32::from(vcell >> 4) / 800.0)
    }
}

#[cfg(not(feature = "async"))]
impl<I2C, E> Max17044<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Get battery voltage
    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL)?;
        Ok(f32::from(vcell >> 4) / 400.0)
    }
}

#[cfg(feature = "async")]
impl<I2C, E> Max17044<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    /// Get battery voltage
    pub async fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL).await?;
        Ok(f32::from(vcell >> 4) / 400.0)
    }
}
