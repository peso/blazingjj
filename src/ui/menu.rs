/*!
The user can use the menu to discover and execute actions
in lazyjj.

*/
use std::str::FromStr;

use ratatui::crossterm::event::Event;
use tracing::trace;
use tui_menu::{MenuAction, MenuEvent, MenuItem, MenuState};

use crate::keybinds::{Shortcut, keybinds_store::KeybindsStore};

// TODO: Move to ui/actions.rs and include sub-actions as defined by tabs
#[derive(Clone, Eq, PartialEq, Debug)]
/// Application actions
pub enum Action {
    FileNew,
    FileOpen,
    FileRefresh,
    ViewLog,
    ViewFiles,
    ViewBookmarks,
    ViewCommands,
    AppExit,
    AboutAuthor,
    AboutHelp,
}

/**
  Create the menu and all its menu items
*/
pub fn create() -> MenuState<Action> {
    MenuState::new(vec![
        MenuItem::group(
            "File",
            vec![
                MenuItem::item("New", Action::FileNew),
                MenuItem::item("Open", Action::FileOpen),
                /*
                MenuItem::group(
                    "Open recent",
                        ["file_1.txt",
                         "file_2.txt"]
                        .iter()
                        .map(|&f| MenuItem::item(f, Action::FileOpenRecent(f.into())))
                        .collect(),
                ),
                */
                MenuItem::item("Refresh", Action::FileRefresh),
                MenuItem::item("Exit", Action::AppExit),
            ],
        ),
        MenuItem::group(
            "View",
            vec![
                MenuItem::item("Log", Action::ViewLog),
                MenuItem::item("Files", Action::ViewFiles),
                MenuItem::item("Bookmarks", Action::ViewBookmarks),
                MenuItem::item("Commands", Action::ViewCommands),
            ],
        ),
        MenuItem::group(
            "About",
            vec![
                MenuItem::item("Author", Action::AboutAuthor),
                MenuItem::item("Help", Action::AboutHelp),
            ],
        ),
    ])
}

fn keybinds() -> KeybindsStore<MenuAction> {
    let default_binds = vec![
        ("up", MenuAction::Up),
        ("down", MenuAction::Down),
        ("left", MenuAction::Left),
        ("right", MenuAction::Right),
        ("k", MenuAction::Up),
        ("j", MenuAction::Down),
        ("h", MenuAction::Left),
        ("l", MenuAction::Right),
        ("esc", MenuAction::Reset),
        ("enter", MenuAction::Select),
        ("f10", MenuAction::Activate),
        (".", MenuAction::Activate),
    ];
    let mut keys = KeybindsStore::default();
    for (bind_str, bind_act) in default_binds {
        keys.add_action(Shortcut::from_str(bind_str).unwrap(), bind_act);
    }

    return keys;
}

//use ratatui::crossterm::event::MouseEvent;

/// Try to handle an input event. If it caused a selection you can
/// get it from next_action()
pub fn input(menu: &mut MenuState<Action>, event: Event) -> bool {
    let event_name = event.clone();
    let action = match event {
        Event::Key(key_event) => {
            let Some(key_action) = keybinds().match_event(key_event) else {
                return false;
            };
            key_action
        },
        Event::Mouse(mouse_event) => {
            let handled = menu.on_mouse_event(&mouse_event);
            // drain event from mouse action must be done
            // by calling menu::next_action
            return handled;
        },
        _ => {
            trace!("menu passes on event {:?} ::", event_name);
            return false;
        },
    };

    // Check if MenuAction is allowed
    let menu_active = menu.is_active();
    if !menu_active {
        if action != MenuAction::Activate {
            trace!("menu ignores event matching {:?}", action);
            return false;
        }
    }

    // Handle MenuAction
    trace!("menu event {:?}", action);
    menu.handle_action(action);
    return true;
}

/// Return a list of actions selected. This will be empty if input events
/// only caused an internal state change of the menu.
pub fn next_action(menu: &mut MenuState<Action>) -> Vec<Action> {
    let mut result = vec![];
    // If menu selected an action, return it
    for e in menu.drain_events() {
        match e {
            MenuEvent::Selected(item) => {
                trace!("menu selected {:?}", item);
                menu.reset();
                result.push(item);
            }
        }
    }
    result
}
