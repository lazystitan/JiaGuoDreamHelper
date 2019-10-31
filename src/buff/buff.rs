use super::building_buff::BuildingBuffType;
use super::policy_buff::PolicyBuffType;
use super::picture_buff::PictureBuffType;

pub enum GlobalBuffType {
    Building(BuildingBuffType),
    Policy(PolicyBuffType),
    Picture(PictureBuffType)
}