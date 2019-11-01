use cursive::theme::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::Color::*;
use cursive::theme::BaseColor::*;

pub fn get_theme() -> Theme {
	let mut p = Palette::default();
	p[Background] = TerminalDefault;
	p[View] = TerminalDefault;
	p[Primary] = TerminalDefault;
	p[Secondary] = TerminalDefault;
	p[TitlePrimary] = Light(Green);
	p[TitleSecondary] = TerminalDefault;
	p[Highlight] = Dark(Blue);
	p[HighlightInactive] = Dark(Magenta);
	Theme {
		shadow: false,
		borders: BorderStyle::Simple,
		palette: p,
	}
}
