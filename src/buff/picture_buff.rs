
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum PictureBuffType {
    //    Goods,
    Industrial,
    Commercial,
    Housing,
    All,
    Online,
    Offline
}

#[derive(Clone)]
pub struct PictureBuff(String, PictureBuffType, f64);