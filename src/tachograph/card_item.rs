use crate::tacho::CardFileID;

#[derive(Debug)]
pub struct CardItem<D> {
    pub card_file_id: CardFileID,
    pub data: D,
}
