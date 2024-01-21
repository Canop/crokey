
# Crokey Examples

To run an example, `cd` to its directory then do `cargo run`.

## deser_keybindings

Shows how a set of key-bindings can be read from JSON (might have been TOML, Hjson, YAML, etc.) and the action executed when the user presses the relevant key combination.

## print_key

Shows how a combiner transforms crossterm key events into key combinations.

The `Combiner` is configured to recognize combinations which aren't normally available, when the terminal supports the Kitty Keyboard protocol.

When using a combiner, key combinations involving a modifier (ctrl, alt, shift, space) are detected on key release.

## print_key_no_combiner

Similar to print_key, but simpler, uses no `Combiner`.

Key combinations which are standard on ANSI terminals are handled, but the capabilities of more modern terminals won't be used and you won't get combinations like `ctrl-a-b`, or `space-n`.

When not using a combiner, all combinations are detected on key press.
