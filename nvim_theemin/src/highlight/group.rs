use crate::color::{gui, Color, TuiHighlightArg};

#[derive(Debug, Clone)]
pub struct Group<'a> {
    fg: &'a Color<'a>,
    bg: &'a Color<'a>,
    gui: Vec<TuiHighlightArg>,
    cterm: Vec<TuiHighlightArg>,
    guisp: gui::Color<'a>,
}

impl<'a> Group<'a> {
    pub fn new(fg: &'a Color<'a>, bg: &'a Color<'a>) -> Self {
        Self {
            fg,
            bg,
            gui: vec![TuiHighlightArg::None],
            cterm: vec![TuiHighlightArg::None],
            guisp: gui::Color::None,
        }
    }

    pub fn fg(&self) -> &Color {
        self.fg
    }

    pub fn bg(&self) -> &Color {
        self.bg
    }

    // TODO: this should probably take Gui HighlightArgs
    pub fn gui(&self) -> &[TuiHighlightArg] {
        &self.gui
    }

    pub fn cterm(&self) -> &[TuiHighlightArg] {
        &self.cterm
    }

    pub fn guisp(&self) -> gui::Color {
        self.guisp
    }

    pub fn set_gui(&mut self, new_gui: TuiHighlightArg) -> &mut Self {
        self.gui = vec![new_gui];
        self
    }

    pub fn set_guis(&mut self, new_guis: Vec<TuiHighlightArg>) -> &mut Self {
        self.gui = new_guis;
        self
    }

    pub fn set_cterm(&mut self, new_cterm: TuiHighlightArg) -> &mut Self {
        self.cterm = vec![new_cterm];
        self
    }

    pub fn set_cterms(&mut self, new_cterms: Vec<TuiHighlightArg>) -> &mut Self {
        self.cterm = new_cterms;
        self
    }

    pub fn set_guisp(&mut self, new_guisp: gui::Color<'a>) -> &mut Self {
        self.guisp = new_guisp;
        self
    }

    pub fn cmd(&self) -> String {
        let guifg = self.fg.gui_string();
        let guibg = self.bg.gui_string();
        let ctermfg = self.fg.cterm_string();
        let ctermbg = self.bg.cterm_string();
        let gui = self
            .gui
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<_>>()
            .join(",");
        let cterm = self
            .cterm
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<_>>()
            .join(",");
        let guisp = self.guisp();

        // let mut highlight_builder: SetHighlightOptsBuilder = Default::default();
        // let highlights = highlight_builder
        //     .ctermbg(&HARD_DARK1.bg0.rgb().to_string())
        //     .ctermfg(&DARK2.fg.rgb().to_string())
        //     .build();

        // oxi::set_hl(0, "Normal", &highlights)?;

        format!(
            "guifg={guifg} guibg={guibg} ctermfg={ctermfg} ctermbg={ctermbg} gui={gui} cterm={cterm} guisp={}",
            guisp.as_ref()
        )
    }
}
