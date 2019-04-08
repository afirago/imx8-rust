#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CH_DEBUG1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `STATEMACHINE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATEMACHINER {
    #[doc = "This is the idle state of the DMA state machine."]
    IDLE,
    #[doc = "State in which the DMA is waiting to receive the first word of a command."]
    REQ_CMD1,
    #[doc = "State in which the DMA is waiting to receive the third word of a command."]
    REQ_CMD3,
    #[doc = "State in which the DMA is waiting to receive the second word of a command."]
    REQ_CMD2,
    #[doc = "The state machine processes the descriptor command field in this state and branches accordingly."]
    XFER_DECODE,
    #[doc = "The state machine waits in this state for the PIO APB cycles to complete."]
    REQ_WAIT,
    #[doc = "State in which the DMA is waiting to receive the fourth word of a command, or waiting to receive the PIO words when PIO count is greater than 1."]
    REQ_CMD4,
    #[doc = "This state determines whether another PIO cycle needs to occur before starting DMA transfers."]
    PIO_REQ,
    #[doc = "During a read transfers, the state machine enters this state waiting for the last bytes to be pushed out on the APB."]
    READ_FLUSH,
    #[doc = "When an AHB read request occurs, the state machine waits in this state for the AHB transfer to complete."]
    READ_WAIT,
    #[doc = "During DMA Write transfers, the state machine waits in this state until the AHB master arbiter accepts the request from this channel."]
    WRITE,
    #[doc = "During DMA Read transfers, the state machine waits in this state until the AHB master arbiter accepts the request from this channel."]
    READ_REQ,
    #[doc = "Upon completion of the DMA transfers, this state checks the value of the Chain bit and branches accordingly."]
    CHECK_CHAIN,
    #[doc = "The state machine goes to this state after the DMA transfers are complete, and determines what step to take next."]
    XFER_COMPLETE,
    #[doc = "When a terminate signal is set, the state machine enters this state until the current AHB transfer is completed."]
    TERMINATE,
    #[doc = "When the Wait for Command End bit is set, the state machine enters this state until the DMA device indicates that the command is complete."]
    WAIT_END,
    #[doc = "During DMA Write transfers, the state machine waits in this state until the AHB master completes the write to the AHB memory space."]
    WRITE_WAIT,
    #[doc = "If HALTONTERMINATE is set and a terminate signal is set, the state machine enters this state and effectively halts. A channel reset is required to exit this state"]
    HALT_AFTER_TERM,
    #[doc = "If the Chain bit is a 0, the state machine enters this state and effectively halts."]
    CHECK_WAIT,
    #[doc = "When the NAND Wait for Ready bit is set, the state machine enters this state until the GPMI device indicates that the external device is ready."]
    WAIT_READY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATEMACHINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATEMACHINER::IDLE => 0,
            STATEMACHINER::REQ_CMD1 => 1,
            STATEMACHINER::REQ_CMD3 => 2,
            STATEMACHINER::REQ_CMD2 => 3,
            STATEMACHINER::XFER_DECODE => 4,
            STATEMACHINER::REQ_WAIT => 5,
            STATEMACHINER::REQ_CMD4 => 6,
            STATEMACHINER::PIO_REQ => 7,
            STATEMACHINER::READ_FLUSH => 8,
            STATEMACHINER::READ_WAIT => 9,
            STATEMACHINER::WRITE => 12,
            STATEMACHINER::READ_REQ => 13,
            STATEMACHINER::CHECK_CHAIN => 14,
            STATEMACHINER::XFER_COMPLETE => 15,
            STATEMACHINER::TERMINATE => 20,
            STATEMACHINER::WAIT_END => 21,
            STATEMACHINER::WRITE_WAIT => 28,
            STATEMACHINER::HALT_AFTER_TERM => 29,
            STATEMACHINER::CHECK_WAIT => 30,
            STATEMACHINER::WAIT_READY => 31,
            STATEMACHINER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATEMACHINER {
        match value {
            0 => STATEMACHINER::IDLE,
            1 => STATEMACHINER::REQ_CMD1,
            2 => STATEMACHINER::REQ_CMD3,
            3 => STATEMACHINER::REQ_CMD2,
            4 => STATEMACHINER::XFER_DECODE,
            5 => STATEMACHINER::REQ_WAIT,
            6 => STATEMACHINER::REQ_CMD4,
            7 => STATEMACHINER::PIO_REQ,
            8 => STATEMACHINER::READ_FLUSH,
            9 => STATEMACHINER::READ_WAIT,
            12 => STATEMACHINER::WRITE,
            13 => STATEMACHINER::READ_REQ,
            14 => STATEMACHINER::CHECK_CHAIN,
            15 => STATEMACHINER::XFER_COMPLETE,
            20 => STATEMACHINER::TERMINATE,
            21 => STATEMACHINER::WAIT_END,
            28 => STATEMACHINER::WRITE_WAIT,
            29 => STATEMACHINER::HALT_AFTER_TERM,
            30 => STATEMACHINER::CHECK_WAIT,
            31 => STATEMACHINER::WAIT_READY,
            i => STATEMACHINER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == STATEMACHINER::IDLE
    }
    #[doc = "Checks if the value of the field is `REQ_CMD1`"]
    #[inline]
    pub fn is_req_cmd1(&self) -> bool {
        *self == STATEMACHINER::REQ_CMD1
    }
    #[doc = "Checks if the value of the field is `REQ_CMD3`"]
    #[inline]
    pub fn is_req_cmd3(&self) -> bool {
        *self == STATEMACHINER::REQ_CMD3
    }
    #[doc = "Checks if the value of the field is `REQ_CMD2`"]
    #[inline]
    pub fn is_req_cmd2(&self) -> bool {
        *self == STATEMACHINER::REQ_CMD2
    }
    #[doc = "Checks if the value of the field is `XFER_DECODE`"]
    #[inline]
    pub fn is_xfer_decode(&self) -> bool {
        *self == STATEMACHINER::XFER_DECODE
    }
    #[doc = "Checks if the value of the field is `REQ_WAIT`"]
    #[inline]
    pub fn is_req_wait(&self) -> bool {
        *self == STATEMACHINER::REQ_WAIT
    }
    #[doc = "Checks if the value of the field is `REQ_CMD4`"]
    #[inline]
    pub fn is_req_cmd4(&self) -> bool {
        *self == STATEMACHINER::REQ_CMD4
    }
    #[doc = "Checks if the value of the field is `PIO_REQ`"]
    #[inline]
    pub fn is_pio_req(&self) -> bool {
        *self == STATEMACHINER::PIO_REQ
    }
    #[doc = "Checks if the value of the field is `READ_FLUSH`"]
    #[inline]
    pub fn is_read_flush(&self) -> bool {
        *self == STATEMACHINER::READ_FLUSH
    }
    #[doc = "Checks if the value of the field is `READ_WAIT`"]
    #[inline]
    pub fn is_read_wait(&self) -> bool {
        *self == STATEMACHINER::READ_WAIT
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == STATEMACHINER::WRITE
    }
    #[doc = "Checks if the value of the field is `READ_REQ`"]
    #[inline]
    pub fn is_read_req(&self) -> bool {
        *self == STATEMACHINER::READ_REQ
    }
    #[doc = "Checks if the value of the field is `CHECK_CHAIN`"]
    #[inline]
    pub fn is_check_chain(&self) -> bool {
        *self == STATEMACHINER::CHECK_CHAIN
    }
    #[doc = "Checks if the value of the field is `XFER_COMPLETE`"]
    #[inline]
    pub fn is_xfer_complete(&self) -> bool {
        *self == STATEMACHINER::XFER_COMPLETE
    }
    #[doc = "Checks if the value of the field is `TERMINATE`"]
    #[inline]
    pub fn is_terminate(&self) -> bool {
        *self == STATEMACHINER::TERMINATE
    }
    #[doc = "Checks if the value of the field is `WAIT_END`"]
    #[inline]
    pub fn is_wait_end(&self) -> bool {
        *self == STATEMACHINER::WAIT_END
    }
    #[doc = "Checks if the value of the field is `WRITE_WAIT`"]
    #[inline]
    pub fn is_write_wait(&self) -> bool {
        *self == STATEMACHINER::WRITE_WAIT
    }
    #[doc = "Checks if the value of the field is `HALT_AFTER_TERM`"]
    #[inline]
    pub fn is_halt_after_term(&self) -> bool {
        *self == STATEMACHINER::HALT_AFTER_TERM
    }
    #[doc = "Checks if the value of the field is `CHECK_WAIT`"]
    #[inline]
    pub fn is_check_wait(&self) -> bool {
        *self == STATEMACHINER::CHECK_WAIT
    }
    #[doc = "Checks if the value of the field is `WAIT_READY`"]
    #[inline]
    pub fn is_wait_ready(&self) -> bool {
        *self == STATEMACHINER::WAIT_READY
    }
}
#[doc = r" Value of the field"]
pub struct WR_FIFO_FULLR {
    bits: bool,
}
impl WR_FIFO_FULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WR_FIFO_EMPTYR {
    bits: bool,
}
impl WR_FIFO_EMPTYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RD_FIFO_FULLR {
    bits: bool,
}
impl RD_FIFO_FULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RD_FIFO_EMPTYR {
    bits: bool,
}
impl RD_FIFO_EMPTYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NEXTCMDADDRVALIDR {
    bits: bool,
}
impl NEXTCMDADDRVALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct READYR {
    bits: bool,
}
impl READYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ENDR {
    bits: bool,
}
impl ENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct KICKR {
    bits: bool,
}
impl KICKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BURSTR {
    bits: bool,
}
impl BURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct REQR {
    bits: bool,
}
impl REQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - PIO Display of the DMA Channel n state machine state."]
    #[inline]
    pub fn statemachine(&self) -> STATEMACHINER {
        STATEMACHINER::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - This bit reflects the current state of the DMA Channel's Write FIFO Full signal."]
    #[inline]
    pub fn wr_fifo_full(&self) -> WR_FIFO_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR_FIFO_FULLR { bits }
    }
    #[doc = "Bit 21 - This bit reflects the current state of the DMA Channel's Write FIFO Empty signal."]
    #[inline]
    pub fn wr_fifo_empty(&self) -> WR_FIFO_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR_FIFO_EMPTYR { bits }
    }
    #[doc = "Bit 22 - This bit reflects the current state of the DMA Channel's Read FIFO Full signal."]
    #[inline]
    pub fn rd_fifo_full(&self) -> RD_FIFO_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RD_FIFO_FULLR { bits }
    }
    #[doc = "Bit 23 - This bit reflects the current state of the DMA Channel's Read FIFO Empty signal."]
    #[inline]
    pub fn rd_fifo_empty(&self) -> RD_FIFO_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RD_FIFO_EMPTYR { bits }
    }
    #[doc = "Bit 24 - This bit reflects the internal bit which indicates whether the channel's next command address is valid"]
    #[inline]
    pub fn nextcmdaddrvalid(&self) -> NEXTCMDADDRVALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NEXTCMDADDRVALIDR { bits }
    }
    #[doc = "Bit 26 - This bit is reserved for this DMA Channel and always reads 0."]
    #[inline]
    pub fn ready(&self) -> READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READYR { bits }
    }
    #[doc = "Bit 28 - This bit reflects the current state of the DMA End Command Signal sent from the APB Device"]
    #[inline]
    pub fn end(&self) -> ENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENDR { bits }
    }
    #[doc = "Bit 29 - This bit reflects the current state of the DMA Kick Signal sent to the APB Device"]
    #[inline]
    pub fn kick(&self) -> KICKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KICKR { bits }
    }
    #[doc = "Bit 30 - This bit reflects the current state of the DMA Burst Signal from the APB device"]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURSTR { bits }
    }
    #[doc = "Bit 31 - This bit reflects the current state of the DMA Request Signal from the APB device"]
    #[inline]
    pub fn req(&self) -> REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REQR { bits }
    }
}
