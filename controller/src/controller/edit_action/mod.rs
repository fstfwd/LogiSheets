use controller_base::{async_func::Task, CellId};

/// EditAction indicates the operation generated by users. It should be translated
/// before it is excuted.
use crate::SheetId;

use self::style_payload::StyleUpdate;

mod converter;
pub mod style_payload;

pub type Converter<'a> = converter::Converter<'a>;

#[derive(Debug)]
pub enum EditAction {
    Undo,
    Redo,
    Payloads(Vec<EditPayload>),
}
#[derive(Debug)]
pub enum EditPayload {
    BlockInput(BlockInput),
    BlockStyleUpdate(BlockStyleUpdate),
    CellInput(CellInput),
    ColShift(ColShift),
    CreateBlock(CreateBlock),
    LineShiftInBlock(LineShiftInBlock),
    MoveBlock(MoveBlock),
    RowShift(RowShift),
    SetColWidth(SetColWidth),
    SetRowHeight(SetRowHeight),
    StyleUpdate(StyleUpdate),
}

#[derive(Debug)]
pub struct RowShift {
    pub sheet_idx: usize,
    pub start: usize,
    pub count: usize,
    pub insert: bool,
}

#[derive(Debug)]
pub struct ColShift {
    pub sheet_idx: usize,
    pub start: usize,
    pub count: usize,
    pub insert: bool,
}

#[derive(Debug)]
pub struct CellInput {
    pub sheet_idx: usize,
    pub row: usize,
    pub col: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct CreateBlock {
    pub sheet_idx: usize,
    pub id: usize,
    pub master_row: usize,
    pub master_col: usize,
    pub row_cnt: usize,
    pub col_cnt: usize,
}

#[derive(Debug)]
pub struct SetRowHeight {
    pub sheet_idx: usize,
    pub idx: usize,
    pub height: f64,
}

#[derive(Debug)]
pub struct SetColWidth {
    pub sheet_idx: usize,
    pub idx: usize,
    pub width: f64,
}

#[derive(Debug)]
pub struct MoveBlock {
    pub sheet_idx: usize,
    pub id: usize,
    pub new_master_row: usize,
    pub new_master_col: usize,
}

#[derive(Debug)]
pub struct BlockInput {
    pub sheet_idx: usize,
    pub id: usize,
    pub row: usize,
    pub col: usize,
    pub input: String,
}

#[derive(Debug)]
pub struct LineShiftInBlock {
    pub sheet_idx: usize,
    pub id: usize,
    pub idx: usize,
    pub cnt: usize,
    pub horizontal: bool,
    pub insert: bool,
}

#[derive(Debug)]
pub struct BlockStyleUpdate {}

#[derive(Default, Debug)]
pub struct ActionEffect {
    // sheet indices
    pub sheets: Vec<usize>,
    pub async_tasks: Vec<Task>,
    pub dirtys: Vec<(SheetId, CellId)>,
}