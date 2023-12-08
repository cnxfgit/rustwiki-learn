pub mod kinds {
    /// RYB颜色模型的三原色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// RYB模型的调和色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
pub mod utils {
    use crate::kinds::*;
    /// 将两种等量的原色混合生成调和色
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
