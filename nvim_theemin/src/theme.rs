use nvim_oxi::api as oxi;

pub trait LoadTheme {
    const NAME: &'static str;

    fn load_theme(&self) -> crate::Result {
        oxi::command("highlight clear")?;
        oxi::command("syntax reset")?;
        oxi::set_var("colors_name", Self::NAME)?;

        self.highlight_common()?;
        self.highlight_syntax()?;
        self.highlight_predefined()?;
        self.highlight_terminal()?;
        self.highlight_plugins()?;

        Ok(())
    }

    fn highlight_common(&self) -> crate::Result {
        Ok(())
    }

    fn highlight_syntax(&self) -> crate::Result {
        Ok(())
    }

    fn highlight_predefined(&self) -> crate::Result {
        Ok(())
    }

    fn highlight_terminal(&self) -> crate::Result {
        Ok(())
    }

    fn highlight_plugins(&self) -> crate::Result {
        Ok(())
    }
}
