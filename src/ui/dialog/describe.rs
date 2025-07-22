/*! ui::dialog::DescribeDialog is used to capture the text input that
goes into a change description.
*/

use std::cmp::max;
use std::str::FromStr; // used by crate::set_keybinds macro

use anyhow::Result;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui_textarea::CursorMove;
use ratatui_textarea::TextArea;

use crate::Commander;
use crate::keybinds::Shortcut;
use crate::keybinds::keybinds_store::KeybindsStore;
use crate::set_keybinds;
use crate::ui;
use crate::ui::ComponentAction;
use crate::ui::ComponentInputResult;
use crate::ui::utils::centered_rect_fixed;

#[derive(Clone, Eq, PartialEq)]
enum Action {
    None,
    Cancel,
    Save,
}

fn default_keybinds() -> KeybindsStore<Action> {
    let mut keys = KeybindsStore::<Action>::default();
    set_keybinds!(
        keys,
        Action::Cancel => "esc",
        Action::Save => "ctrl+s",
    );
    keys
}

pub struct DescribeDialog {
    keybinds: KeybindsStore<Action>,
    commander: Commander,
    /// The revset operated on
    // = log_tab.head.commit_id.as_str(),
    revset: String,
    /// Editor text. Note lifetime is set to 'static
    /// which is ok because all data is provided as owned Strings
    describe_textarea: TextArea<'static>,
}

impl DescribeDialog {
    pub fn new(
        commander: &Commander,
        revset: &str,
        text: String)
     -> Self {
        let mut textarea = TextArea::new(
            text
                .split("\n")
                .map(|line| line.to_string())
                .collect(),
        );
        textarea.move_cursor(CursorMove::End);
        Self {
            keybinds: default_keybinds(),
            commander: commander.clone(),
            revset: revset.into(),
            describe_textarea: textarea,
        }
    }

    /// Map an event to a popup action
    fn match_event(&self, event: &Event) -> Action {
        if let Event::Key(key) = event {
            return self.keybinds.match_event(*key)
                .unwrap_or(Action::None);
        }
        Action::None
    }
}

impl ui::Component for DescribeDialog {
    fn input(&mut self, event: Event) -> Result<ComponentInputResult> {
        let commander = &self.commander;
        match self.match_event(&event) {
            Action::Save => {
                // TODO: Handle error
                commander.run_describe(
                    &self.revset,
                    &self.describe_textarea.lines().join("\n"),
                )?;

                //
                /*
                log_tab.head = commander.get_head_latest(&self.head)?;

                // can I make app call LogTab.handle_event(LogTabEvent::Refresh)

                //simulate LogTabEvent::Refresh, or focus()
                log_tab.refresh_log_output();
                log_tab.refresh_head_output(); // called by set_head
                // Maybe
                // CommandAction::SetHead
                // CommandAction::RereshTab .. nah, not good enough
                */

                return Ok(ComponentInputResult::HandledAction(
                    ComponentAction::Multiple(vec![
                        ComponentAction::SetPopup(None),
                        //ComponentAction::ViewLog(head),
                        ComponentAction::RefreshTab(),
                    ]),
                ));
            }
            Action::Cancel => {
                return Ok(ComponentInputResult::HandledAction(
                    ComponentAction::SetPopup(None),
                ));
            }
            _ => (),
        }
        self.describe_textarea.input(event);
        return Ok(ComponentInputResult::Handled);
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::bordered()
            .title(Span::styled(" Describe ", Style::new().bold().cyan()))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green));
        // Text target size
        const MAX_COMMIT_WIDTH: u16 = 72; // git recommended max width
        const MIN_COMMIT_HEIGHT: u16 = 5; // heading + blank + 3 lines
        // Include margin and help text to get size
        let area = centered_rect_fixed(
            area,
            /* width */ MAX_COMMIT_WIDTH + 2,
            /* height */ max(MIN_COMMIT_HEIGHT + 4, area.height / 2),
        );
        f.render_widget(Clear, area);
        f.render_widget(&block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(2)])
            .split(block.inner(area));

        f.render_widget(&self.describe_textarea, popup_chunks[0]);

        let help = Paragraph::new(vec!["Ctrl+s: save | Escape: cancel".into()])
            .fg(Color::DarkGray)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(help, popup_chunks[1]);

        Ok(())
    }
}
