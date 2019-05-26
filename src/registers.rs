use bitflags::bitflags;

bitflags!(
    pub struct Flags : u32 {
        const CARRY = 0x0000_0001;
        const RESERVED = 0x0000_0002;
        const PARITY = 0x0000_0004;
        const ADJUST = 0x0000_0010;
        const ZERO = 0x0000_0040;
        const SIGN = 0x0000_0080;
        const TRAP = 0x0000_0100;
        const INTERRUPT = 0x0000_0200;
        const DIRECTION = 0x0000_0400;
        const OVERFLOW = 0x0000_0800;
        const IOPL = 0x0000_3000;
        const NESTED_TASK = 0x0000_4000;
        const RESUME = 0x0001_0000;
        const VM = 0x0002_0000;
        const ALIGN_CHECK = 0x0004_0000;
        const VIRT_INTR_FLAG = 0x0008_0000;
        const VIRT_INTR_PEND = 0x0010_0000;
        const CPUID = 0x0020_0000;
    }
);

bitflags!(
    pub struct SegmentFlags : u16 {
        const TYPE = 0x001f;
        const DPL = 0x0060;
        const PRESENT = 0x0080;
        const AVAILABLE = 0x1000;
        const TYPE2 = 0x4000;
        const GRANULARITY = 0x8000;
    }
);

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub selector : u16,
    pub base : u32,
    pub limit : u32,
    pub flags : SegmentFlags
}

#[derive(Clone, Copy, Debug)]
pub struct Registers {
    pub gprs : [u32; 8],
    pub eip : u32,
    pub eflags : Flags,
    pub segs : [Segment; 6],
    pub gdtr : Segment,
    pub ldtr : Segment,
    pub idtr : Segment,
    pub task_reg : Segment,
}