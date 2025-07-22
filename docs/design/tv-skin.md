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

The general event flow is that the app captures an event (mouse or keyboard),
and then sends it to all active layers in z order, top-down. As soon
as the event is marked as handled, the propagation stops.

The layers are hardcoded as
1. global shortcuts
2. menu (optional)
3. pop-up (optional)
4. tabs

To start out simple, all optional layers are modal. This means that as soon as
the layer is present, it will handle all events even those it has no action to.

Global shortcut examples: exit, help

Tabs may have their own shortcuts.
Example: Log tab has 'd' for describe, which is also in the menu.



Remember the rendering order of layers is bottom-up.

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

### Main menu

The menu is like the information architecture for a website: It gives
a structure to all the actions the application can take.

This is the menu structure for lazyjj

  Repo   - aka repository
    New     - jj git init
    Open    - change to this directory that holds a jj repo / workspace
              jj git import if needed
    Clone   - jj git clone
    -----
    Fetch   - jj git fetch
    Push    - jj git push
    -----
    Sparse..       - edit sparse patterns in this worksmace
    Workspace..    - manage workspaces
    -----
    Preferences - jj config *
    Exit

  Change    - edit a change
    Undo     Ctrl+Z
    Redo     Ctrl+Y
    Operation..
    ----
    New           N
    Duplicate
    Parallelize
    Split
    ----
    Edit          E
    Describe      D
    Rebase.. Ctrl+R
    Resolve..          -- jj resolve with external merge tool
    ----
    Revert..
    Squash   Ctrl+S
    Absorb
    Abandon     DEL
    ----
    Sign                -- jj sign
    Unsign                -- jj unsign
    ----
    Go to @       @
    Find..     Ctrl+F  -- search for content, filter changes, search for tag
    Annotate
    Evolog
    At Operation       -- jj --at-operation [OPERATION]

  File    - edit a file
    Annotate
    Track
    Untrack
    Restore..        -- jj restore [FILESETS]
    
  View   - change the open display (was tab)
    Files            -- jj file list
    Annotate
    ----
    Revisions        -- jj log
    Evolog           -- jj evolog [FILE]
    Operations       -- jj op log
    ----
    Workspaces
    Remotes

  Help
    Keybindings        -- Open github page for lazyjj keybindings
    jj CLI commands        -- show jj util markdown-help as a paged hypertext
                            you can jump to the help page by pressing F1
                            when highlighing a menu item
                            The status line will show  
                              F1 CLI command help | <command summary>
    jj keyword help..
        -- jj help -k bookmarks: Named pointers to revisions (similar to Git's branches)
        -- jj help -k config:    How and where to set configuration options
        -- jj help -k filesets:  A functional language for selecting a set of files
        -- jj help -k glossary:  Definitions of various terms
        -- jj help -k revsets:   A functional language for selecting a set of revision
        -- jj help -k templates: A functional language to customize command output
        -- jj help -k tutorial: 
    Report issue   -- Open github page for lazyjj issues
    About             -- version of lazyjj, release date, version of jj, OS version


These are the commands that jj version 0.30.0 has
  abandon           Abandon a revision
  absorb            Move changes from a revision into the stack of mutable revisions
  bookmark          Manage bookmarks [default alias: b]
    create   Create a new bookmark [aliases: c]
    delete   Delete an existing bookmark and propagate the deletion to remotes on the next push [aliases: d]
    forget   Forget a bookmark without marking it as a deletion to be pushed [aliases: f]
    list     List bookmarks and their targets [aliases: l]
    move     Move existing bookmarks to target revision [aliases: m]
    rename   Rename `old` bookmark name to `new` bookmark name [aliases: r]
    set      Create or update a bookmark to point to a certain commit [aliases: s]
    track    Start tracking given remote bookmarks [aliases: t]
    untrack  Stop tracking given remote bookmarks
  commit            Update the description and create a new change on top
  config            Manage config options
    edit   Start an editor on a jj config file [aliases: e]
    get    Get the value of a given config option. [aliases: g]
    list   List variables set in config files, along with their values [aliases: l]
    path   Print the paths to the config files [aliases: p]
    set    Update a config file to set the given option to a given value [aliases: s]
    unset  Update a config file to unset the given option [aliases: u]
  describe          Update the change description or other metadata [aliases: desc]
  diff              Compare file contents between two revisions
  diffedit          Touch up the content changes in a revision with a diff editor
  duplicate         Create new changes with the same content as existing ones
  edit              Sets the specified revision as the working-copy revision
  evolog            Show how a change has evolved over time [aliases: evolution-log]
  file              File operations
    annotate  Show the source change for each line of the target file
    chmod     Sets or removes the executable bit for paths in the repo
    list      List files in a revision
    show      Print contents of files in a revision
    track     Start tracking specified paths in the working copy
    untrack   Stop tracking specified paths in the working copy
  fix               Update files with formatting fixes or other changes
  git               Commands for working with Git remotes and the underlying Git repo
    clone   Create a new repo backed by a clone of a Git repo
    export  Update the underlying Git repo with changes made in the repo
    fetch   Fetch from a Git remote
    import  Update repo with changes made in the underlying Git repo
    init    Create a new Git backed repo
    push    Push to a Git remote
    remote  Manage Git remotes
      add      Add a Git remote
      list     List Git remotes
      remove   Remove a Git remote and forget its bookmarks
      rename   Rename a Git remote
      set-url  Set the URL of a Git remote
    root    Show the underlying Git directory of a repository using the Git backend
  help              Print this message or the help of the given subcommand(s)
      -k, --keyword <KEYWORD>
        Show help for keywords instead of commands

        Possible values:
        - bookmarks: Named pointers to revisions (similar to Git's branches)
        - config:    How and where to set configuration options
        - filesets:  A functional language for selecting a set of files
        - glossary:  Definitions of various terms
        - revsets:   A functional language for selecting a set of revision
        - templates: A functional language to customize command output
        - tutorial:  Show a tutorial to get started with jj

  interdiff         Compare the changes of two commits
  log               Show revision history
  new               Create a new, empty change and (by default) edit it in the working copy
  next              Move the working-copy commit to the child revision
    --conflict        Jump to the next conflicted descendant
  operation         Commands for working with the operation log [aliases: op]
    abandon  Abandon operation history
    diff     Compare changes to the repository between two operations
    log      Show the operation log
    restore  Create a new operation that restores the repo to an earlier state
    show     Show changes to the repository in an operation
    undo     Create a new operation that undoes an earlier operation
  parallelize       Parallelize revisions by making them siblings
  prev              Change the working copy revision relative to the parent revision
  rebase            Move revisions to different parent(s)
  resolve           Resolve conflicted files with an external merge tool
  restore           Restore paths from another revision
  revert            Apply the reverse of the given revision(s)
  root              Show the current workspace root directory (shortcut for `jj workspace root`)
  show              Show commit description and changes in a revision
  sign              Cryptographically sign a revision
  simplify-parents  Simplify parent edges for the specified revision(s)
  sparse            Manage which paths from the working-copy commit are present in the working copy
    edit   Start an editor to update the patterns that are present in the working copy
    list   List the patterns that are currently present in the working copy
    reset  Reset the patterns to include all files in the working copy
    set    Update the patterns that are present in the working copy
  split             Split a revision in two
  squash            Move changes from a revision into another revision
  status            Show high-level repo status [aliases: st]
  tag               Manage tags
    list              List tags
  undo              Undo an operation (shortcut for `jj op undo`)
  unsign            Drop a cryptographic signature
  util              Infrequently used commands such as for generating shell completions
    completion         Print a command-line-completion script
    config-schema      Print the JSON schema for the jj TOML config format
    exec               Execute an external command via jj
    gc                 Run backend-dependent garbage collection
    install-man-pages  Install Jujutsu's manpages to the provided path
    markdown-help      Print the CLI help for all subcommands in Markdown
  version           Display version information
  workspace         Commands for working with workspaces
    add           Add a workspace
    forget        Stop tracking a workspace's working-copy commit in the repo
    list          List workspaces
    rename        Renames the current workspace
    root          Show the current workspace root directory
    update-stale  Update a workspace that has become stale




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