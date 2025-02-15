use serde::{Deserialize, Serialize};
use strum::Display;

/// The different actions that can be performed by the application.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, Hash,
)]
pub enum Action {
    // input actions
    /// Add a character to the input buffer.
    #[serde(skip)]
    AddInputChar(char),
    /// Delete the character before the cursor from the input buffer.
    #[serde(skip)]
    DeletePrevChar,
    /// Delete the character after the cursor from the input buffer.
    #[serde(skip)]
    DeleteNextChar,
    /// Move the cursor to the character before the current cursor position.
    #[serde(skip)]
    GoToPrevChar,
    /// Move the cursor to the character after the current cursor position.
    #[serde(skip)]
    GoToNextChar,
    /// Move the cursor to the start of the input buffer.
    #[serde(alias = "go_to_input_start")]
    GoToInputStart,
    /// Move the cursor to the end of the input buffer.
    #[serde(alias = "go_to_input_end")]
    GoToInputEnd,
    // rendering actions
    /// Render the terminal user interface screen.
    #[serde(skip)]
    Render,
    /// Resize the terminal user interface screen to the given dimensions.
    #[serde(skip)]
    Resize(u16, u16),
    /// Clear the terminal user interface screen.
    #[serde(skip)]
    ClearScreen,
    // results actions
    /// Select the entry currently under the cursor.
    #[serde(alias = "select_entry")]
    SelectEntry,
    /// Select the entry currently under the cursor and pass the key that was pressed
    /// through to be handled the parent process.
    #[serde(alias = "select_passthrough")]
    SelectPassthrough(String),
    /// Select the entry currently under the cursor and exit the application.
    #[serde(alias = "select_and_exit")]
    SelectAndExit,
    /// Select the next entry in the currently focused list.
    #[serde(alias = "select_next_entry")]
    SelectNextEntry,
    /// Select the previous entry in the currently focused list.
    #[serde(alias = "select_prev_entry")]
    SelectPrevEntry,
    /// Copy the currently selected entry to the clipboard.
    #[serde(alias = "copy_entry_to_clipboard")]
    CopyEntryToClipboard,
    // preview actions
    /// Scroll the preview up by one line.
    #[serde(alias = "scroll_preview_up")]
    ScrollPreviewUp,
    /// Scroll the preview down by one line.
    #[serde(alias = "scroll_preview_down")]
    ScrollPreviewDown,
    /// Scroll the preview up by half a page.
    #[serde(alias = "scroll_preview_half_page_up")]
    ScrollPreviewHalfPageUp,
    /// Scroll the preview down by half a page.
    #[serde(alias = "scroll_preview_half_page_down")]
    ScrollPreviewHalfPageDown,
    /// Open the currently selected entry in the default application.
    #[serde(skip)]
    OpenEntry,
    // application actions
    /// Tick the application state.
    #[serde(skip)]
    Tick,
    /// Suspend the application.
    #[serde(skip)]
    Suspend,
    /// Resume the application.
    #[serde(skip)]
    Resume,
    /// Quit the application.
    #[serde(alias = "quit")]
    Quit,
    /// Toggle the help bar.
    #[serde(alias = "toggle_help")]
    ToggleHelp,
    /// Signal an error with the given message.
    #[serde(skip)]
    Error(String),
    /// No operation.
    #[serde(skip)]
    NoOp,
    // channel actions
    /// Toggle the remote control channel.
    #[serde(alias = "toggle_remote_control")]
    ToggleRemoteControl,
    /// Toggle the remote control in `send to channel` mode.
    #[serde(alias = "toggle_send_to_channel")]
    ToggleSendToChannel,
}
