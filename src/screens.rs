use crate::prelude::*;

pub fn game_over_display(ctx: &mut BTerm) {
    ctx.set_active_console(2);
    ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
    ctx.print_color_centered(
        4,
        WHITE,
        BLACK,
        "Slain by a monster, your hero's journey has come to a premature end.",
    );
    ctx.print_color_centered(
        5,
        WHITE,
        BLACK,
        "The Amulet of Yala remains unclaimed, and your home town is not saved.",
    );
    ctx.print_color_centered(
        8,
        YELLOW,
        BLACK,
        "Don't worry, you can always try again with a new hero.",
    );
    ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");
}

pub fn game_won_display(ctx: &mut BTerm) {
    ctx.set_active_console(2);
    ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
    ctx.print_color_centered(
        4,
        WHITE,
        BLACK,
        "You put on the Amulet of Yala and feel its power course through your veins.",
    );
    ctx.print_color_centered(
        5,
        WHITE,
        BLACK,
        "Your town is saved, and you can return to your normal life.",
    );
    ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");
}
