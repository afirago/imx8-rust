#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVT_MIRROR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `EVENTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTSR {
    #[doc = "- DMA request event not pending"]
    EVENTS_0,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl EVENTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            EVENTSR::EVENTS_0 => 0,
            EVENTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> EVENTSR {
        match value {
            0 => EVENTSR::EVENTS_0,
            i => EVENTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EVENTS_0`"]
    #[inline]
    pub fn is_events_0(&self) -> bool {
        *self == EVENTSR::EVENTS_0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - This register reflects the DMA requests received by the SDMA for events 47-32"]
    #[inline]
    pub fn events(&self) -> EVENTSR {
        EVENTSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
