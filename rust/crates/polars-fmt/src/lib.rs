use std::env;

use strum::AsRefStr;
#[derive(Debug, Clone, Copy, AsRefStr)]
pub enum TableFormatting {
    #[strum(serialize = "ASCII_FULL")]
    AsciiFull,
    #[strum(serialize = "ASCII_FULL_CONDENSED")]
    AsciiFullCondensed,
    #[strum(serialize = "ASCII_NO_BORDERS")]
    AsciiNoBorders,
    #[strum(serialize = "ASCII_BORDERS_ONLY")]
    AsciiBordersOnly,
    #[strum(serialize = "ASCII_BORDERS_ONLY_CONDENSED")]
    AsciiBordersOnlyCondensed,
    #[strum(serialize = "ASCII_HORIZONTAL_ONLY")]
    AsciiHorizontalOnly,
    #[strum(serialize = "ASCII_MARKDOWN")]
    AsciiMarkdown,
    #[strum(serialize = "MARKDOWN")]
    Markdown,
    #[strum(serialize = "UTF8_FULL")]
    Utf8Full,
    #[strum(serialize = "UTF8_FULL_CONDENSED")]
    Utf8FullCondensed,
    #[strum(serialize = "UTF8_NO_BORDERS")]
    Utf8NoBorders,
    #[strum(serialize = "UTF8_BORDERS_ONLY")]
    Utf8BordersOnly,
    #[strum(serialize = "UTF8_HORIZONTAL_ONLY")]
    Utf8HorizontalOnly,
    #[strum(serialize = "NOTHING")]
    NOTHING,
}
#[derive(Debug, Clone, Copy, AsRefStr)]
pub enum CellAlignment {
    #[strum(serialize = "LEFT")]
    Left,
    #[strum(serialize = "CENTER")]
    Center,
    #[strum(serialize = "RIGHT")]
    Right,
}

#[derive(Debug)]
pub struct PolarsFmt;
impl Default for PolarsFmt {
    fn default() -> Self { Self::new() }
}

impl PolarsFmt {
    pub fn new() -> Self { Self }

    /// define styling of tables using any of the following options (default =
    /// UTF8_FULL_CONDENSED).  These options are defined by comfy-table
    /// which provides examples for each at _<https://github.com/Nukesor/comfy-table/blob/main/src/style/presets.rs>
    pub fn table_formatting(self, value: TableFormatting) -> Self {
        unsafe { env::set_var("POLARS_FMT_TABLE_FORMATTING", value.as_ref()) };
        self
    }

    ///Set table cell alignment.
    pub fn cell_alignment(self, value: CellAlignment) -> Self {
        unsafe { env::set_var("POLARS_FMT_TABLE_CELL_ALIGNMENT", value.as_ref()) };
        self
    }

    ////Print the DataFrame shape information below the data when displaying tables.
    pub fn dataframe_shape_below(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_DATAFRAME_SHAPE_BELOW",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    /// Hide table column names.
    pub fn hide_column_names(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_HIDE_COLUMN_NAMES",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    /// Hide the DataFrame shape information when displaying tables.
    pub fn hide_column_data_types(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    /// Hide the '---' separator displayed between the column names and column
    /// types.
    pub fn hide_column_separator(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_HIDE_COLUMN_SEPARATOR",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    /// Hide the DataFrame shape information when displaying tables.
    pub fn hide_dataframe_shape_information(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    /// Display the data type next to the column name (to the right, in
    /// parentheses).
    pub fn inline_column_data_type(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_INLINE_COLUMN_DATA_TYPE",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    pub fn rounded_corners(self, enabled: bool) -> Self {
        unsafe {
            env::set_var(
                "POLARS_FMT_TABLE_ROUNDED_CORNERS",
                (enabled as u8).to_string(),
            )
        };
        self
    }

    ///Set the number of columns that are visible when displaying tables.
    ///
    /// If value < 0 (eg: -1), display all columns.
    pub fn max_cols(self, value: i32) -> Self {
        unsafe { env::set_var("POLARS_FMT_MAX_COLS", value.to_string()) };
        self
    }

    ///Set the max number of rows used to draw the table (both Dataframe and
    /// Series).
    ///
    /// If value < 0 (eg: -1), display all rows (DataFrame) and all elements
    /// (Series).
    pub fn max_rows(self, value: i32) -> Self {
        unsafe { env::set_var("POLARS_FMT_MAX_ROWS", value.to_string()) };
        self
    }

    ///Set the number of characters used to display string values.
    pub fn str_length(self, value: usize) -> Self {
        unsafe { env::set_var("POLARS_FMT_STR_LEN", value.to_string()) };
        self
    }
    /// Set the number of elements to display for List values.
    ///
    /// Empty lists will always print "[]". Negative values will result in all
    /// values being printed. A value of 0 will always "[...]" for lists with
    /// contents. A value of 1 will print only the final item in the list.
    pub fn table_cell_list_len(self, value: i32) -> Self {
        unsafe { env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", value.to_string()) };
        self
    }

    /// Set the maximum width of a table in characters
    ///
    /// if value < 0 (eg: -1), display full width..
    pub fn table_width(self, value: i32) -> Self {
        unsafe { env::set_var("POLARS_TABLE_WIDTH", value.to_string()) };
        self
    }

    /// Convenience method to finalize the settings
    pub fn finish(self) {
        // nothing to do, env vars are already set
    }
    pub fn preset_insta() -> Self {
        Self::new()
            .table_formatting(TableFormatting::AsciiFullCondensed)
            .max_cols(-1)
            .max_rows(-1)
            .table_width(-1)
            .table_cell_list_len(-1)
    }
}
