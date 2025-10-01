//! Common methods

#[cfg(not(feature = "async"))]
macro_rules! impl_common {
    ($ic:ident) => {
        /// Device driver
        #[derive(Debug)]
        pub struct $ic<I2C> {
            i2c: I2C,
        }

        impl<I2C, E> $ic<I2C>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Create new instance of the device.
            pub fn new(i2c: I2C) -> Self {
                $ic { i2c }
            }

            /// Destroy driver instance, return I2C bus.
            pub fn destroy(self) -> I2C {
                self.i2c
            }

            /// Quick start
            ///
            /// Restarts fuel-gauge calculations in the same manner as initial power-up
            /// of the IC. This is useful if an application's power-up sequence
            /// is exceedingly noisy
            pub fn quickstart(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::MODE, Command::QSTRT)
            }

            /// Get IC version
            pub fn version(&mut self) -> Result<u16, Error<E>> {
                self.read_register(Register::VERSION)
            }
        }
        impl_register_access!($ic);
    };
}

#[cfg(feature = "async")]
macro_rules! impl_common {
    ($ic:ident) => {
        /// Device driver
        #[derive(Debug)]
        pub struct $ic<I2C> {
            i2c: I2C,
        }

        impl<I2C, E> $ic<I2C>
        where
            I2C: embedded_hal_async::i2c::I2c<Error = E>,
        {
            /// Create new instance of the device.
            pub fn new(i2c: I2C) -> Self {
                $ic { i2c }
            }

            /// Destroy driver instance, return I2C bus.
            pub fn destroy(self) -> I2C {
                self.i2c
            }

            /// Quick start
            ///
            /// Restarts fuel-gauge calculations in the same manner as initial power-up
            /// of the IC. This is useful if an application's power-up sequence
            /// is exceedingly noisy
            pub async fn quickstart(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::MODE, Command::QSTRT).await
            }

            /// Get IC version
            pub async fn version(&mut self) -> Result<u16, Error<E>> {
                self.read_register(Register::VERSION).await
            }
        }
        impl_register_access!($ic);
    };
}
