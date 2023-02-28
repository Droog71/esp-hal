pub trait RadioExt {
    type Components;

    fn split(self) -> Self::Components;
}

/// WiFi radio
pub struct Wifi {
    _private: (),
}

/// Bluetooth radio
pub struct Bluetooth {
    _private: (),
}

/// IEEE 802.15.4 Low rate wireless personal area radio
pub struct LowRate {
    _private: (),
}

impl Wifi {
    pub const unsafe fn steal() -> Self {
        Self { _private: () }
    }
}

impl Bluetooth {
    pub const unsafe fn steal() -> Self {
        Self { _private: () }
    }
}

impl LowRate {
    pub const unsafe fn steal() -> Self {
        Self { _private: () }
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(esp32, esp32c2, esp32c3, esp32s3))] {
        impl RadioExt for crate::peripherals::RADIO {
            type Components = (Wifi, Bluetooth);

            fn split(self) -> Self::Components {
                unsafe {
                    (Wifi::steal(), Bluetooth::steal())
                }
            }
        }
    } else if #[cfg(esp32c6)] {
        impl RadioExt for crate::peripherals::RADIO {
            type Components = (Wifi, Bluetooth, LowRate);

            fn split(self) -> Self::Components {
                unsafe {
                    (Wifi::steal(), Bluetooth::steal(), LowRate::steal())
                }
            }
        }
    } else if #[cfg(esp32s2)] {
        impl RadioExt for crate::peripherals::RADIO {
            type Components = Wifi;

            fn split(self) -> Self::Components {
                unsafe {
                    Wifi::steal()
                }
            }
        }
    }
}