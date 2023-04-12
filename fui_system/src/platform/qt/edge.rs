use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Edge: i32 {
    const Top = 0x01;
    const Left = 0x02;
    const Right = 0x04;
    const Bottom = 0x08;
    const TopLeft = Self::Top.bits() | Self::Left.bits();
    const TopRight = Self::Top.bits() | Self::Right.bits();
    const BottomLeft = Self::Bottom.bits() | Self::Left.bits();
    const BottomRight = Self::Bottom.bits() | Self::Right.bits();
    }
}
