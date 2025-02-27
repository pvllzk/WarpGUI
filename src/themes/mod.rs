pub struct Theme {
    pub primary: String,
    pub primary_dark: String,
    pub primary_light: String,

    pub secondary: String,
    pub secondary_light: String,

    pub red: String,
    pub light_red: String,
    pub green: String,
    pub light_green: String,

    pub background: String,
    pub background_dark: String,
    pub background_light: String,

    pub foreground: String,
    pub foreground_dark: String,

    pub text: String,
    pub text_muted: String,
    pub text_darker: String,
    pub text_bright: String,

    pub placeholder: String,
    pub borders: String,
    pub highlight: String,

    pub semi_transparent: String,
    pub modal: String,
}

// When dealing with colors and backgrounds we should only use values provided within the Themes
impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: String::from("#205FFA"),
            primary_dark: String::from("#1e59ec"),
            primary_light: String::from("#2563fa"),

            secondary: String::from("#2B2B3B"),
            secondary_light: String::from("#2f2f40"),

            green: String::from("#00B894"),
            light_green: String::from("#00c29c"),
            red: String::from("#F93854"),
            light_red: String::from("#fa4662"),

            background: String::from("#0e0d17"),
            background_dark: String::from("#0e0d17"),
            background_light: String::from("#1A1E2E"),

            foreground: String::from("#232838"),
            foreground_dark: String::from("#1a1e2a"),

            text: String::from("#EEF0F2"),
            text_muted: String::from("#6F748A"),
            text_darker: String::from("#A8AABE"),
            text_bright: String::from("#EEEFFF"),
            placeholder: String::from("#6F748A"),

            borders: String::from("#232838"),
            highlight: String::from("#2b2843"),
            semi_transparent: String::from("rgba(14, 13, 23, 0.2)"),
            modal: String::from("#1A1E2E"),
        }
    }
}

impl Theme {
    pub fn load_or_default() -> Self {
        // TODO: Support loading themes in memory
        Self::default()
    }

    pub fn rosetta(&self) -> String {
        let root = format!(
            ":root {{
                --theme-primary: {};
                --theme-primary-dark: {};
                --theme-primary-light: {};
                --theme-secondary: {};
                --theme-secondary-light: {};
                --theme-green: {};
                --theme-light-green: {};
                --theme-red: {};
                --theme-light-red: {};
                --theme-background: {};
                --theme-background-light: {};
                --theme-text: {};
                --theme-text-muted: {};
                --theme-text-darker: {};
                --theme-text-bright: {};
                --theme-placeholder: {};
                --theme-borders: {};
                --theme-highlight: {};
                --theme-semi-transparent: {};
                --theme-modal: {};
                --theme-foreground: {};
                --theme-foreground-dark: {};
            }}",
            &self.primary,
            &self.primary_dark,
            &self.primary_light,
            &self.secondary,
            &self.secondary_light,
            &self.green,
            &self.light_green,
            &self.red,
            &self.light_red,
            &self.background,
            &self.background_light,
            &self.text,
            &self.text_muted,
            &self.text_darker,
            &self.text_bright,
            &self.placeholder,
            &self.borders,
            &self.highlight,
            &self.semi_transparent,
            &self.modal,
            &self.foreground,
            &self.foreground_dark,
        );
        root
    }
}
