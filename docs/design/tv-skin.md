# Turbo Vision Skin

Author: [peso](mailto:peer.sommerlund@gmail.com)

## Summary

A short summary of your project/re-design/component and what problems it
addresses in about 3-10 sentences.

Lazyjj has a vi-feel to it, which is familiar to some but not to all. This change adds a menu and a status bar that makes the actions discoverable and usable.

## State of the Feature as of `$VERSION` (optional)

The state of the feature you want to improve and where it currently falls
short. If there's nothing to compare to, leave it out.

## Prior work (optional)

Does this feature exist somewhere else and which tradeoffs it made.

> If there's no prior work, then use the related work section below.

## Goals and non-goals

Give lazyjj a UI that is more mainstream.

It is not a goal to implement all Turbo Vision features.

## Overview

A detailed overview of the project and the improvements it brings.

At the top a single line is used for the menu.

The menu presents most lazyjj actions in a hierarcy. As the user navigates the hierarchy he can learn actions as well as keyboard shortcuts.

At the bottom a single line is used for the status bar. 

This has several different usages
- when the menu is open, it displays a short text describing the menu item.
- when focusing on a tab, it displays the major actions available.

The status bar gives the user context relevant help.

### Event handling (lazyjj)

Mouse events are handled directly by the menu, but key events are more indirect.
First, the application should define menu navigation keys. This means that the
menu must get a map from key-events to menu-actions. The old design exposed 
functions corresponding to menu actions, but this is more verbose in the
application code and less flexible than having
a handle_action function which can trigger all actions.

### Event handling (move to tui-menu)

Since the mapping is quite generic, it ought to be implemented in a separate
crate, but at least it is isolated to module so it will be easy to extract.

The menu has function
    on_event(self, Event) -> Option<MenuAction>
which uses the event-action map provided by the application if the event is a key,
and uses 
    self.on_mouse_event(Event) -> Option<MenuAction>
if it is a mouse event.

Application should do the following

fn handle_event(event)
    let Some(action) = menu.on_event(event) 
    else { return; }
    // Handle menu command
    menu.handle_action(action);
    // Check for selection event
    for e in menu.drain_events() {
    match e {
        MenuEvent::Selected(item) => {
            // handle selection
        }
    }

### Detailed Design

The place to describe all new interfaces and interactions and how it plays into
the existing code and behavior. This is the place for all nitty-gritty details
which interact with the system.

## Alternatives considered (optional)

Another option to make the actions more discoverable is to implement the gitui command bar. This shows commands at the bottom of the screen, filtered to those relevant at the current moment. This would have a more vim-like feeling to it.

The menu bar design was chosen because it is used by a wider range of more popular applications, eg VSCodium.

## Issues addressed (optional)

A list of issues which are addressed by this design.

## Related Work (optional)

If there's a feature in another VCS which shares some similarities to your
proposed work, it belongs here. An example would be Jujutsu sparse workspaces
and Perforce client workspaces.

## Future Possibilities

Dynamic keyboard bindings that automatic show up on menu items.