#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVT_MIRROR {
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
    #[doc = "DMA request event not pending"]
    EVENTS_0,
    #[doc = "DMA request event pending"]
    EVENTS_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl EVENTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            EVENTSR::EVENTS_0 => 0,
            EVENTSR::EVENTS_1 => 1,
            EVENTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> EVENTSR {
        match value {
            0 => EVENTSR::EVENTS_0,
            1 => EVENTSR::EVENTS_1,
            i => EVENTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EVENTS_0`"]
    #[inline]
    pub fn is_events_0(&self) -> bool {
        *self == EVENTSR::EVENTS_0
    }
    #[doc = "Checks if the value of the field is `EVENTS_1`"]
    #[inline]
    pub fn is_events_1(&self) -> bool {
        *self == EVENTSR::EVENTS_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - This register reflects the DMA requests received by the SDMA for events 31-0"]
    #[inline]
    pub fn events(&self) -> EVENTSR {
        EVENTSR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
