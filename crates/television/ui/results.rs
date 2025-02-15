use crate::television::Television;
use crate::ui::layout::Layout;
use crate::ui::BORDER_COLOR;
use color_eyre::eyre::Result;
use ratatui::layout::Alignment;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListDirection, Padding,
};
use ratatui::Frame;
use std::str::FromStr;
use television_channels::channels::OnAir;
use television_channels::entry::Entry;
use television_utils::strings::{
    next_char_boundary, slice_at_char_boundaries,
};

// Styles
const DEFAULT_RESULT_NAME_FG: Color = Color::Blue;
const DEFAULT_RESULT_PREVIEW_FG: Color = Color::Rgb(150, 150, 150);
const DEFAULT_RESULT_LINE_NUMBER_FG: Color = Color::Yellow;
const DEFAULT_RESULT_SELECTED_BG: Color = Color::Rgb(50, 50, 50);

pub struct ResultsListColors {
    pub result_name_fg: Color,
    pub result_preview_fg: Color,
    pub result_line_number_fg: Color,
    pub result_selected_bg: Color,
}

impl Default for ResultsListColors {
    fn default() -> Self {
        Self {
            result_name_fg: DEFAULT_RESULT_NAME_FG,
            result_preview_fg: DEFAULT_RESULT_PREVIEW_FG,
            result_line_number_fg: DEFAULT_RESULT_LINE_NUMBER_FG,
            result_selected_bg: DEFAULT_RESULT_SELECTED_BG,
        }
    }
}

#[allow(dead_code)]
impl ResultsListColors {
    pub fn result_name_fg(mut self, color: Color) -> Self {
        self.result_name_fg = color;
        self
    }

    pub fn result_preview_fg(mut self, color: Color) -> Self {
        self.result_preview_fg = color;
        self
    }

    pub fn result_line_number_fg(mut self, color: Color) -> Self {
        self.result_line_number_fg = color;
        self
    }

    pub fn result_selected_bg(mut self, color: Color) -> Self {
        self.result_selected_bg = color;
        self
    }
}

pub fn build_results_list<'a, 'b>(
    results_block: Block<'b>,
    entries: &'a [Entry],
    list_direction: ListDirection,
    results_list_colors: Option<ResultsListColors>,
    use_icons: bool,
) -> List<'a>
where
    'b: 'a,
{
    let results_list_colors = results_list_colors.unwrap_or_default();
    List::new(entries.iter().map(|entry| {
        let mut spans = Vec::new();
        // optional icon
        if entry.icon.is_some() && use_icons {
            let icon = entry.icon.as_ref().unwrap();
            spans.push(Span::styled(
                icon.to_string(),
                Style::default().fg(Color::from_str(icon.color).unwrap()),
            ));
            spans.push(Span::raw(" "));
        }
        // entry name
        if let Some(name_match_ranges) = &entry.name_match_ranges {
            let mut last_match_end = 0;
            for (start, end) in name_match_ranges
                .iter()
                .map(|(s, e)| (*s as usize, *e as usize))
            {
                spans.push(Span::styled(
                    slice_at_char_boundaries(
                        &entry.name,
                        last_match_end,
                        start,
                    ),
                    Style::default().fg(results_list_colors.result_name_fg),
                ));
                spans.push(Span::styled(
                    slice_at_char_boundaries(&entry.name, start, end),
                    Style::default().fg(Color::Red),
                ));
                last_match_end = end;
            }
            spans.push(Span::styled(
                &entry.name[next_char_boundary(&entry.name, last_match_end)..],
                Style::default().fg(results_list_colors.result_name_fg),
            ));
        } else {
            spans.push(Span::styled(
                entry.display_name(),
                Style::default().fg(results_list_colors.result_name_fg),
            ));
        }
        // optional line number
        if let Some(line_number) = entry.line_number {
            spans.push(Span::styled(
                format!(":{line_number}"),
                Style::default().fg(results_list_colors.result_line_number_fg),
            ));
        }
        // optional preview
        if let Some(preview) = &entry.value {
            spans.push(Span::raw(": "));

            if let Some(preview_match_ranges) = &entry.value_match_ranges {
                if !preview_match_ranges.is_empty() {
                    let mut last_match_end = 0;
                    for (start, end) in preview_match_ranges
                        .iter()
                        .map(|(s, e)| (*s as usize, *e as usize))
                    {
                        spans.push(Span::styled(
                            slice_at_char_boundaries(
                                preview,
                                last_match_end,
                                start,
                            ),
                            Style::default()
                                .fg(results_list_colors.result_preview_fg),
                        ));
                        spans.push(Span::styled(
                            slice_at_char_boundaries(preview, start, end),
                            Style::default().fg(Color::Red),
                        ));
                        last_match_end = end;
                    }
                    spans.push(Span::styled(
                        &preview[next_char_boundary(
                            preview,
                            preview_match_ranges.last().unwrap().1 as usize,
                        )..],
                        Style::default()
                            .fg(results_list_colors.result_preview_fg),
                    ));
                }
            } else {
                spans.push(Span::styled(
                    preview,
                    Style::default().fg(results_list_colors.result_preview_fg),
                ));
            }
        }
        Line::from(spans)
    }))
    .direction(list_direction)
    .highlight_style(
        Style::default().bg(results_list_colors.result_selected_bg),
    )
    .highlight_symbol("> ")
    .block(results_block)
}

impl Television {
    pub(crate) fn draw_results_list(
        &mut self,
        f: &mut Frame,
        layout: &Layout,
    ) -> Result<()> {
        let results_block = Block::default()
            .title_top(Line::from(" Results ").alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(BORDER_COLOR))
            .style(Style::default())
            .padding(Padding::right(1));

        let result_count = self.channel.result_count();
        if result_count > 0 && self.results_picker.selected().is_none() {
            self.results_picker.select(Some(0));
            self.results_picker.relative_select(Some(0));
        }

        let entries = self.channel.results(
            layout.results.height.saturating_sub(2).into(),
            u32::try_from(self.results_picker.view_offset)?,
        );

        let results_list = build_results_list(
            results_block,
            &entries,
            ListDirection::BottomToTop,
            None,
            self.config.ui.use_nerd_font_icons,
        );

        f.render_stateful_widget(
            results_list,
            layout.results,
            &mut self.results_picker.relative_state,
        );
        Ok(())
    }
}
