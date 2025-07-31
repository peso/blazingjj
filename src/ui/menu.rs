/*!
The user can use the menu to discover and execute actions
in lazyjj.

*/
use std::collections::HashSet;
use std::str::FromStr;

use ratatui::crossterm::event::Event;
use tracing::trace;
use tui_menu::{MenuAction, MenuEvent, MenuItem, MenuState};

use crate::keybinds::{Shortcut, keybinds_store::KeybindsStore};

// TODO: Move to ui/actions.rs and include sub-actions as defined by tabs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
/// Application actions
pub enum Action {
    AppPreferences,
    AppExit,

    RepoNew,
    RepoOpen,
    RepoClone,
    RepoRefresh,
    RepoFetch,
    RepoPush,
    RepoSparse,
    RepoWorkspace,

    OperationUndo, // jj undo = jj op undo one change
    OperationRedo, // jj op restore the change just undone
    OperationAbandon,
    OperationInverse, // interactive jj operation undo
    OperationRestore, // interactive restore

    ChangeDescribe,
    ChangeEdit,
    ChangeNew,
    ChangeDuplicate,
    ChangeParallelize,
    ChangeSplit,
    ChangeRebase,
    ChangeResolve,
    ChangeSquash,
    ChangeAbsorb,
    ChangeAbandon,
    ChangeSign,
    ChangeUnsign,

    FileTrack,
    FileUntrack,
    FileRestore, // jj restore

    ViewChanges,
    ViewFiles,
    ViewBookmarks,
    ViewAnnotate,
    ViewChangeEvolution,
    ViewOperations,
    ViewWorkspaces,
    ViewRemotes,
    ViewTags,

    HelpKeybindings,
    HelpCLICommands,
    HelpKeyword,
    HelpReportIssue,
    HelpAbout,
}

/**
  Create the menu and all its menu items
*/
pub fn create() -> MenuState<Action> {
    MenuState::new(vec![
        MenuItem::group(
            "Repo",
            vec![
                MenuItem::item("New", Action::RepoNew),
                MenuItem::item("Open", Action::RepoOpen),
                /*
                MenuItem::group(
                    "Open recent",
                        ["file_1.txt",
                         "file_2.txt"]
                        .iter()
                        .map(|&f| MenuItem::item(f, Action::RepoOpenRecent(f.into())))
                        .collect(),
                ),
                */
                MenuItem::item("Refresh", Action::RepoRefresh),
                MenuItem::line(),
                MenuItem::item("Clone", Action::RepoClone),
                MenuItem::item("Fetch", Action::RepoFetch),
                MenuItem::item("Push", Action::RepoPush),
                MenuItem::line(),
                MenuItem::item("Sparse..", Action::RepoSparse),
                MenuItem::item("Workspace..", Action::RepoWorkspace),
                MenuItem::line(),
                MenuItem::item("Preferences..", Action::AppPreferences),
                MenuItem::item("Exit", Action::AppExit),
            ],
        ),
        MenuItem::group(
            "Operation",
            vec![
                MenuItem::item("Undo", Action::OperationUndo),
                MenuItem::item("Redo", Action::OperationRedo),
                MenuItem::line(),
                MenuItem::item("Diff", Action::OperationAbandon),
                MenuItem::item("Restore", Action::OperationRestore),
                MenuItem::item("Inverse", Action::OperationInverse),
                MenuItem::item("Abandon", Action::OperationAbandon),
            ],
        ),
        MenuItem::group(
            "Change",
            vec![
                MenuItem::item("New", Action::ChangeNew),
                MenuItem::item("Duplicate", Action::ChangeDuplicate),
                MenuItem::item("Parallelize", Action::ChangeParallelize),
                MenuItem::item("Split", Action::ChangeSplit),
                MenuItem::line(),
                MenuItem::item("Edit", Action::ChangeEdit),
                MenuItem::item("Describe..", Action::ChangeDescribe),
                MenuItem::item("Rebase..", Action::ChangeRebase),
                MenuItem::item("Resolve..", Action::ChangeResolve),
                MenuItem::line(),
                MenuItem::item("Revert..", Action::RepoPush),
                MenuItem::item("Squash", Action::ChangeSquash),
                MenuItem::item("Absorb", Action::ChangeAbsorb),
                MenuItem::item("Abandon", Action::ChangeAbandon),
                MenuItem::line(),
                MenuItem::item("Sign", Action::ChangeSign),
                MenuItem::item("Unsign", Action::ChangeUnsign),
                MenuItem::line(),
                MenuItem::item("Find..", Action::HelpAbout),
            ],
        ),
        MenuItem::group(
            "File",
            vec![
                MenuItem::item("Track", Action::FileTrack),
                MenuItem::item("Untrack", Action::FileUntrack),
                MenuItem::item("Restore..", Action::FileRestore),
            ],
        ),
        MenuItem::group(
            "View",
            vec![
                MenuItem::item("Files", Action::ViewFiles),
                MenuItem::item("Annotate file", Action::ViewAnnotate),
                MenuItem::line(),
                MenuItem::item("Changes", Action::ViewChanges),
                MenuItem::item("Change evolution", Action::ViewChangeEvolution),
                MenuItem::item("Operations", Action::ViewOperations),
                MenuItem::line(),
                MenuItem::item("Workspaces", Action::ViewWorkspaces),
                MenuItem::item("Remotes", Action::ViewRemotes),
                MenuItem::item("Bookmarks", Action::ViewBookmarks),
                MenuItem::item("Tags", Action::ViewTags),
            ],
        ),
        MenuItem::group(
            "Help",
            vec![
                MenuItem::item("Keybindings", Action::HelpKeybindings),
                MenuItem::item("CLI Commands", Action::HelpCLICommands),
                MenuItem::item("Keywords", Action::HelpKeyword),
                MenuItem::item("Report issue", Action::HelpReportIssue),
                MenuItem::item("About", Action::HelpAbout),
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

/// Update menu items with actions, so they are enabled iff
/// the action is in the actions set.
pub fn enable_actions(
    menu: &mut MenuState<Action>,
    actions: &HashSet<Action>,
) {
    for mir in menu.iter() {
        let mut menu_item = mir.borrow_mut();
        if let Some(act) = &menu_item.data {
            menu_item.enabled = actions.contains(act);
        }
    }
}