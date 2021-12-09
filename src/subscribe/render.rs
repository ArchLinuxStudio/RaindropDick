use super::appsub::AppSub;
use super::appsub::InputMode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppSub) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal | InputMode::Popup | InputMode::PopupEdit | InputMode::SubscriptView => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select trees"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Select => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search bar"));
    f.render_widget(input, chunks[1]);
    if let InputMode::Editing = app.input_mode {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + app.input.width() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1,
        )
    }

    // Bottom two inner blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[2]);

    let subs: Vec<ListItem> = app.subs[app.subsindex]
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let subs = List::new(subs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Subscribe {}", app.subsindex)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    // popup wiget
    f.render_stateful_widget(subs, bottom_chunks[0], &mut app.state[app.subsindex]);
    //let block : Box<dyn Widget> = {
    if let Some(a) = app.state[app.subsindex].selected() {
        let list = app.informations[app.subsindex][a].information_to_list();
        let messages: Vec<ListItem> = list
            .iter()
            .map(|infom| {
                let content = vec![Spans::from(Span::raw(infom))];
                ListItem::new(content)
            })
            .collect();
        let block =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Informations"));
        f.render_widget(block, bottom_chunks[1]);
    } else {
        let block = Block::default()
            .title("Informations None")
            .borders(Borders::ALL);
        f.render_widget(block, bottom_chunks[1]);
    }
    //};
    //f.render_widget(*block, bottom_chunks[0]);

    if app.show_popup {
        //let block = Block::default().title("About port").borders(Borders::ALL);

        //f.render_widget(input, chunks[1]);
        let area = centered_rect(80, 50, f.size());

        let chunk = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(area);

        f.render_widget(Clear, area); //this clears out the background
        let (msg, style) = (
            vec![Span::raw("Settings, e to edit, s to save")],
            Style::default(),
        );
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let title = Paragraph::new(text);
        f.render_widget(title, chunk[0]);
        let inputpop = Paragraph::new(app.settings_input[0].as_ref())
            .style(match (&app.input_mode, app.index_settings) {
                (InputMode::PopupEdit, 0) => Style::default().fg(Color::Yellow),
                (_, _) => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("v2ray-core"));
        f.render_widget(inputpop, chunk[1]);
        let inputpop2 = Paragraph::new(app.settings_input[1].as_ref())
            .style(match (&app.input_mode, app.index_settings) {
                (InputMode::PopupEdit, 1) => Style::default().fg(Color::Yellow),
                (_, _) => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("add domins"));
        f.render_widget(inputpop2, chunk[2]);
        let subscription: Vec<ListItem> = app
            .subscription
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}:{}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let subscription = List::new(subscription)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(subscription, chunk[3], &mut app.index_subscription);
        if let InputMode::PopupEdit = app.input_mode {
            let index = app.index_settings;
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunk[index + 2].x + app.settings_input[index].width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunk[index + 1].y + 1,
            )
            //InputMode::Normal | InputMode::Select | InputMode::Popup =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        }
    }
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
