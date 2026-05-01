use cliclack::{Theme, ThemeState, set_theme};
use console::{Style, style};

struct GojiTheme;

impl Theme for GojiTheme {
    fn input_style(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().color256(160).bold(),
            ThemeState::Cancel => Style::new().dim().strikethrough(),
            ThemeState::Submit => Style::new().dim(),
            ThemeState::Error(_) => Style::new(),
        }
    }

    fn radio_symbol(&self, _state: &ThemeState, selected: bool) -> String {
        if selected {
            style("›").color256(160).bold().to_string()
        } else {
            style(" ").to_string()
        }
    }

    fn cursor_with_style(&self, cursor: &cliclack::StringCursor, new_style: &Style) -> String {
        let (left, ch, right) = cursor.split();
        let is_field_empty = left.is_empty() && ch.trim().is_empty() && right.is_empty();

        if is_field_empty {
            return "🔎".to_string();
        } else {
            format!(
                "{} {}{}{}{}",
                "🔎".to_string(),
                new_style.apply_to(left),
                style("|").color256(160).bold(),
                new_style.apply_to(ch),
                new_style.apply_to(right),
            )
        }
    }
}

pub fn init_theme() {
    set_theme(GojiTheme);
}
