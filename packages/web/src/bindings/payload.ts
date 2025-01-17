// DO NOT EDIT. CODE GENERATED BY gents.
import {BlockInput} from './block_input'
import {BlockStyleUpdate} from './block_style_update'
import {CellInput} from './cell_input'
import {ColShift} from './col_shift'
import {CreateBlock} from './create_block'
import {LineShiftInBlock} from './line_shift_in_block'
import {MoveBlock} from './move_block'
import {RemoveBlock} from './remove_block'
import {RowShift} from './row_shift'
import {SetColWidth} from './set_col_width'
import {SetRowHeight} from './set_row_height'
import {SetVisible} from './set_visible'
import {SheetRename} from './sheet_rename'
import {SheetShift} from './sheet_shift'
import {StyleUpdate} from './style_update'

// `EditPayload` is the basic update unit of the Workbook. Developers can config their own
// `EditAction` (e.g. setting a button to create a table) to facilitate their users.
export type EditPayload =
    | {blockInput: BlockInput}
    | {blockStyleUpdate: BlockStyleUpdate}
    | {cellInput: CellInput}
    | {colShift: ColShift}
    | {createBlock: CreateBlock}
    | {lineShiftInBlock: LineShiftInBlock}
    | {moveBlock: MoveBlock}
    | {removeBlock: RemoveBlock}
    | {rowShift: RowShift}
    | {setColWidth: SetColWidth}
    | {setRowHeight: SetRowHeight}
    | {setVisible: SetVisible}
    | {sheetRename: SheetRename}
    | {sheetShift: SheetShift}
    | {styleUpdate: StyleUpdate}
