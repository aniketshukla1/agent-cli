use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeKind {
    // High-Contrast Dark with Pure White / Silver Text
    MidnightWhite,
    ObsidianSilver,
    TitaniumDark,
    CarbonSilver,
    OledMonochrome,
    Vesper,

    // Popular Vibrant Dark Themes
    CatppuccinMocha,
    TokyoNight,
    Dracula,
    RosePine,
    Synthwave84,
    OneDark,
    Nord,
    GruvboxDark,
    Cyberpunk,
    Monokai,
    Kanagawa,
    AyuDark,
    Palenight,
    SolarizedDark,
    GitHubDarkDimmed,
    EmeraldMatrix,

    // Elegant Light Themes
    MinimalistLight,
    CatppuccinLatte,
    GitHubLight,
    PaperwhiteMinimal,
    RosePineDawn,
    TokyoNightDay,
    SolarizedLight,
}

impl ThemeKind {
    pub const ALL: &'static [ThemeKind] = &[
        // White & Silver Dark Themes
        ThemeKind::MidnightWhite,
        ThemeKind::ObsidianSilver,
        ThemeKind::TitaniumDark,
        ThemeKind::CarbonSilver,
        ThemeKind::OledMonochrome,
        ThemeKind::Vesper,

        // Iconic Dark Themes
        ThemeKind::CatppuccinMocha,
        ThemeKind::TokyoNight,
        ThemeKind::Dracula,
        ThemeKind::RosePine,
        ThemeKind::Synthwave84,
        ThemeKind::OneDark,
        ThemeKind::Nord,
        ThemeKind::GruvboxDark,
        ThemeKind::Cyberpunk,
        ThemeKind::Monokai,
        ThemeKind::Kanagawa,
        ThemeKind::AyuDark,
        ThemeKind::Palenight,
        ThemeKind::SolarizedDark,
        ThemeKind::GitHubDarkDimmed,
        ThemeKind::EmeraldMatrix,

        // Light Themes
        ThemeKind::MinimalistLight,
        ThemeKind::CatppuccinLatte,
        ThemeKind::GitHubLight,
        ThemeKind::PaperwhiteMinimal,
        ThemeKind::RosePineDawn,
        ThemeKind::TokyoNightDay,
        ThemeKind::SolarizedLight,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ThemeKind::MidnightWhite => "Midnight White",
            ThemeKind::ObsidianSilver => "Obsidian Silver",
            ThemeKind::TitaniumDark => "Titanium Dark",
            ThemeKind::CarbonSilver => "Carbon Silver",
            ThemeKind::OledMonochrome => "OLED Monochrome",
            ThemeKind::Vesper => "Vesper Minimal",

            ThemeKind::CatppuccinMocha => "Catppuccin Mocha",
            ThemeKind::TokyoNight => "Tokyo Night",
            ThemeKind::Dracula => "Dracula",
            ThemeKind::RosePine => "Rosé Pine",
            ThemeKind::Synthwave84 => "Synthwave '84",
            ThemeKind::OneDark => "One Dark",
            ThemeKind::Nord => "Nord",
            ThemeKind::GruvboxDark => "Gruvbox Dark",
            ThemeKind::Cyberpunk => "Cyberpunk Neon",
            ThemeKind::Monokai => "Monokai Pro",
            ThemeKind::Kanagawa => "Kanagawa Wave",
            ThemeKind::AyuDark => "Ayu Dark",
            ThemeKind::Palenight => "Material Palenight",
            ThemeKind::SolarizedDark => "Solarized Dark",
            ThemeKind::GitHubDarkDimmed => "GitHub Dimmed",
            ThemeKind::EmeraldMatrix => "Emerald Matrix",

            ThemeKind::MinimalistLight => "Minimalist Light",
            ThemeKind::CatppuccinLatte => "Catppuccin Latte",
            ThemeKind::GitHubLight => "GitHub Light",
            ThemeKind::PaperwhiteMinimal => "Paperwhite Clean",
            ThemeKind::RosePineDawn => "Rosé Pine Dawn",
            ThemeKind::TokyoNightDay => "Tokyo Night Day",
            ThemeKind::SolarizedLight => "Solarized Light",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            ThemeKind::MidnightWhite
            | ThemeKind::ObsidianSilver
            | ThemeKind::TitaniumDark
            | ThemeKind::CarbonSilver
            | ThemeKind::OledMonochrome
            | ThemeKind::Vesper => "DARK / SILVER-WHITE",

            ThemeKind::CatppuccinMocha
            | ThemeKind::TokyoNight
            | ThemeKind::Dracula
            | ThemeKind::RosePine
            | ThemeKind::Synthwave84
            | ThemeKind::OneDark
            | ThemeKind::Nord
            | ThemeKind::GruvboxDark
            | ThemeKind::Cyberpunk
            | ThemeKind::Monokai
            | ThemeKind::Kanagawa
            | ThemeKind::AyuDark
            | ThemeKind::Palenight
            | ThemeKind::SolarizedDark
            | ThemeKind::GitHubDarkDimmed
            | ThemeKind::EmeraldMatrix => "DARK PALETTES",

            ThemeKind::MinimalistLight
            | ThemeKind::CatppuccinLatte
            | ThemeKind::GitHubLight
            | ThemeKind::PaperwhiteMinimal
            | ThemeKind::RosePineDawn
            | ThemeKind::TokyoNightDay
            | ThemeKind::SolarizedLight => "LIGHT PALETTES",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ThemeKind::MidnightWhite => "Pure OLED black with crisp pure white text & platinum silver accents",
            ThemeKind::ObsidianSilver => "Volcanic obsidian dark with sterling silver typography & chrome border",
            ThemeKind::TitaniumDark => "Gunmetal dark with brushed silver text and titanium white headings",
            ThemeKind::CarbonSilver => "Matte carbon fiber dark with clean silver body and white highlights",
            ThemeKind::OledMonochrome => "Ultra high-contrast deep black with pure white and silver monochrome",
            ThemeKind::Vesper => "Minimalist deep charcoal with warm silver and pure bone white typography",

            ThemeKind::CatppuccinMocha => "Soothing pastel dark theme with soft tones",
            ThemeKind::TokyoNight => "Clean dark theme inspired by Tokyo neon lights",
            ThemeKind::Dracula => "Classic vampire dark palette with vibrant purples & pinks",
            ThemeKind::RosePine => "Warm pine, faux fur, and soft rose pastels",
            ThemeKind::Synthwave84 => "80s retro neon glow with hot magenta & electric blue",
            ThemeKind::OneDark => "Iconic Atom/VSCode balanced modern dark theme",
            ThemeKind::Nord => "Arctic, north-bluish clean and elegant palette",
            ThemeKind::GruvboxDark => "Retro groove warm dark palette with earthy tones",
            ThemeKind::Cyberpunk => "High-energy vibrant neon cyan and hot magenta",
            ThemeKind::Monokai => "High-contrast classic developer theme",
            ThemeKind::Kanagawa => "Japanese ukiyo-e wave aesthetic with sumi ink and soft autumn tones",
            ThemeKind::AyuDark => "Deep slate with warm amber, coral orange, and vibrant cyan",
            ThemeKind::Palenight => "Sleek Material Theme deep purple-indigo with lavender accents",
            ThemeKind::SolarizedDark => "Precision-calibrated dark palette by Ethan Schoonover",
            ThemeKind::GitHubDarkDimmed => "Modern muted charcoal with crisp GitHub accents",
            ThemeKind::EmeraldMatrix => "Obsidian black with luminous cyber emerald green",

            ThemeKind::MinimalistLight => "Crisp, clean high-readability daylight theme",
            ThemeKind::CatppuccinLatte => "Warm, cozy daylight pastel theme with dark blue-slate text",
            ThemeKind::GitHubLight => "Official GitHub pure white theme with navy accents",
            ThemeKind::PaperwhiteMinimal => "Distraction-free warm newsprint cream with crisp charcoal text",
            ThemeKind::RosePineDawn => "Soft morning daylight variant of Rosé Pine",
            ThemeKind::TokyoNightDay => "Pristine daytime variant of Tokyo Night with rich indigo typography",
            ThemeKind::SolarizedLight => "Warm paper-like cream and cyan daytime palette",
        }
    }
}

impl Default for ThemeKind {
    fn default() -> Self {
        ThemeKind::MidnightWhite
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub kind: ThemeKind,
    pub bg: Color,
    pub fg: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub border: Color,
    pub border_focused: Color,
    pub user_msg_bg: Color,
    pub assistant_msg_bg: Color,
    pub tool_msg_bg: Color,
    pub code_bg: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub selected_bg: Color,
    pub selected_fg: Color,
    pub muted: Color,
}

impl Theme {
    pub fn from_kind(kind: ThemeKind) -> Self {
        match kind {
            // ==========================================
            // 1. WHITE & SILVER TEXT DARK THEMES
            // ==========================================
            ThemeKind::MidnightWhite => Theme {
                kind,
                bg: Color::Rgb(0, 0, 0),               // Pure OLED Black
                fg: Color::Rgb(255, 255, 255),         // Pure White
                primary: Color::Rgb(255, 255, 255),    // Pure White
                secondary: Color::Rgb(220, 225, 235),  // Platinum Silver
                accent: Color::Rgb(195, 205, 220),     // Cool Silver
                success: Color::Rgb(160, 240, 180),    // Mint
                warning: Color::Rgb(250, 220, 140),    // Pale Amber
                error: Color::Rgb(255, 120, 120),      // Soft Red
                border: Color::Rgb(65, 70, 80),        // Steel Silver
                border_focused: Color::Rgb(255, 255, 255), // Pure White
                user_msg_bg: Color::Rgb(20, 22, 26),
                assistant_msg_bg: Color::Rgb(10, 11, 14),
                tool_msg_bg: Color::Rgb(16, 18, 22),
                code_bg: Color::Rgb(12, 13, 16),
                status_bar_bg: Color::Rgb(8, 9, 12),
                status_bar_fg: Color::Rgb(230, 235, 245),
                selected_bg: Color::Rgb(45, 50, 60),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(150, 158, 170),      // Silver Muted
            },

            ThemeKind::ObsidianSilver => Theme {
                kind,
                bg: Color::Rgb(18, 18, 20),            // Deep Obsidian
                fg: Color::Rgb(228, 232, 238),         // Sterling Silver
                primary: Color::Rgb(255, 255, 255),    // Crisp White
                secondary: Color::Rgb(198, 204, 216),  // Polished Silver
                accent: Color::Rgb(215, 222, 234),     // Bright Chrome
                success: Color::Rgb(130, 220, 160),
                warning: Color::Rgb(245, 215, 130),
                error: Color::Rgb(245, 115, 115),
                border: Color::Rgb(65, 68, 78),        // Silver Border
                border_focused: Color::Rgb(230, 235, 245),
                user_msg_bg: Color::Rgb(30, 32, 38),
                assistant_msg_bg: Color::Rgb(24, 25, 29),
                tool_msg_bg: Color::Rgb(27, 28, 33),
                code_bg: Color::Rgb(12, 12, 14),
                status_bar_bg: Color::Rgb(12, 12, 14),
                status_bar_fg: Color::Rgb(220, 225, 235),
                selected_bg: Color::Rgb(52, 56, 68),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(140, 146, 160),      // Silver muted
            },

            ThemeKind::TitaniumDark => Theme {
                kind,
                bg: Color::Rgb(24, 26, 30),            // Gunmetal
                fg: Color::Rgb(242, 244, 248),         // Titanium White
                primary: Color::Rgb(255, 255, 255),    // Pure White
                secondary: Color::Rgb(205, 212, 224),  // Brushed Silver
                accent: Color::Rgb(180, 190, 205),     // Steel
                success: Color::Rgb(145, 225, 165),
                warning: Color::Rgb(250, 215, 120),
                error: Color::Rgb(245, 125, 125),
                border: Color::Rgb(72, 78, 90),
                border_focused: Color::Rgb(255, 255, 255),
                user_msg_bg: Color::Rgb(36, 40, 48),
                assistant_msg_bg: Color::Rgb(28, 31, 38),
                tool_msg_bg: Color::Rgb(32, 35, 42),
                code_bg: Color::Rgb(16, 18, 22),
                status_bar_bg: Color::Rgb(16, 18, 22),
                status_bar_fg: Color::Rgb(235, 238, 245),
                selected_bg: Color::Rgb(58, 64, 76),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(152, 160, 175),
            },

            ThemeKind::CarbonSilver => Theme {
                kind,
                bg: Color::Rgb(22, 22, 24),            // Carbon Matte
                fg: Color::Rgb(220, 223, 228),         // Silver Body
                primary: Color::Rgb(255, 255, 255),    // Pure White
                secondary: Color::Rgb(195, 200, 210),  // Sterling
                accent: Color::Rgb(235, 240, 248),     // Bright Silver
                success: Color::Rgb(140, 220, 155),
                warning: Color::Rgb(245, 205, 115),
                error: Color::Rgb(240, 110, 110),
                border: Color::Rgb(68, 70, 76),
                border_focused: Color::Rgb(255, 255, 255),
                user_msg_bg: Color::Rgb(34, 34, 38),
                assistant_msg_bg: Color::Rgb(27, 27, 30),
                tool_msg_bg: Color::Rgb(30, 30, 34),
                code_bg: Color::Rgb(14, 14, 16),
                status_bar_bg: Color::Rgb(14, 14, 16),
                status_bar_fg: Color::Rgb(225, 228, 235),
                selected_bg: Color::Rgb(55, 56, 64),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(138, 142, 152),
            },

            ThemeKind::OledMonochrome => Theme {
                kind,
                bg: Color::Rgb(5, 5, 5),               // True OLED Black
                fg: Color::Rgb(245, 245, 245),         // Off-White
                primary: Color::Rgb(255, 255, 255),    // Pure White
                secondary: Color::Rgb(215, 215, 215),  // Silver
                accent: Color::Rgb(185, 185, 185),     // Medium Silver
                success: Color::Rgb(175, 235, 175),
                warning: Color::Rgb(245, 235, 150),
                error: Color::Rgb(245, 135, 135),
                border: Color::Rgb(60, 60, 60),
                border_focused: Color::Rgb(255, 255, 255),
                user_msg_bg: Color::Rgb(26, 26, 26),
                assistant_msg_bg: Color::Rgb(16, 16, 16),
                tool_msg_bg: Color::Rgb(20, 20, 20),
                code_bg: Color::Rgb(10, 10, 10),
                status_bar_bg: Color::Rgb(8, 8, 8),
                status_bar_fg: Color::Rgb(225, 225, 225),
                selected_bg: Color::Rgb(48, 48, 48),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(140, 140, 140),
            },

            ThemeKind::Vesper => Theme {
                kind,
                bg: Color::Rgb(16, 16, 16),            // #101010
                fg: Color::Rgb(255, 255, 255),         // Pure White
                primary: Color::Rgb(255, 199, 153),    // Amber Glow
                secondary: Color::Rgb(205, 205, 210),  // Silver text
                accent: Color::Rgb(255, 165, 110),     // Sunset Orange
                success: Color::Rgb(155, 215, 160),
                warning: Color::Rgb(255, 210, 115),
                error: Color::Rgb(255, 130, 130),
                border: Color::Rgb(55, 55, 55),
                border_focused: Color::Rgb(255, 199, 153),
                user_msg_bg: Color::Rgb(30, 30, 30),
                assistant_msg_bg: Color::Rgb(22, 22, 22),
                tool_msg_bg: Color::Rgb(25, 25, 25),
                code_bg: Color::Rgb(10, 10, 10),
                status_bar_bg: Color::Rgb(10, 10, 10),
                status_bar_fg: Color::Rgb(220, 220, 225),
                selected_bg: Color::Rgb(48, 48, 48),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(130, 130, 135),
            },

            // ==========================================
            // 2. ICONIC DARK THEMES
            // ==========================================
            ThemeKind::CatppuccinMocha => Theme {
                kind,
                bg: Color::Rgb(30, 30, 46),            // Base #1e1e2e
                fg: Color::Rgb(205, 214, 244),         // Text #cdd6f4
                primary: Color::Rgb(137, 180, 250),    // Blue #89b4fa
                secondary: Color::Rgb(203, 166, 247),  // Mauve #cba6f7
                accent: Color::Rgb(245, 194, 231),     // Pink #f5c2e7
                success: Color::Rgb(166, 227, 161),    // Green #a6e3a1
                warning: Color::Rgb(249, 226, 175),    // Yellow #f9e2af
                error: Color::Rgb(243, 139, 168),      // Red #f38ba8
                border: Color::Rgb(88, 91, 112),       // Surface2 #585b70
                border_focused: Color::Rgb(137, 180, 250),
                user_msg_bg: Color::Rgb(49, 50, 68),
                assistant_msg_bg: Color::Rgb(36, 36, 54),
                tool_msg_bg: Color::Rgb(40, 44, 60),
                code_bg: Color::Rgb(24, 24, 37),
                status_bar_bg: Color::Rgb(17, 17, 27),
                status_bar_fg: Color::Rgb(186, 194, 222),
                selected_bg: Color::Rgb(69, 71, 90),
                selected_fg: Color::Rgb(245, 224, 220),
                muted: Color::Rgb(108, 112, 134),
            },

            ThemeKind::TokyoNight => Theme {
                kind,
                bg: Color::Rgb(26, 27, 38),            // #1a1b26
                fg: Color::Rgb(192, 202, 245),         // #c0caf5
                primary: Color::Rgb(122, 162, 247),    // #7aa2f7
                secondary: Color::Rgb(187, 154, 247),  // #bb9af7
                accent: Color::Rgb(125, 207, 255),     // #7dcfff
                success: Color::Rgb(158, 206, 106),    // #9ece6a
                warning: Color::Rgb(224, 175, 104),    // #e0af68
                error: Color::Rgb(247, 118, 142),      // #f7768e
                border: Color::Rgb(65, 72, 104),       // #414868
                border_focused: Color::Rgb(122, 162, 247),
                user_msg_bg: Color::Rgb(36, 40, 59),
                assistant_msg_bg: Color::Rgb(31, 35, 53),
                tool_msg_bg: Color::Rgb(34, 38, 56),
                code_bg: Color::Rgb(22, 22, 30),
                status_bar_bg: Color::Rgb(19, 20, 29),
                status_bar_fg: Color::Rgb(169, 177, 214),
                selected_bg: Color::Rgb(46, 52, 78),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(86, 95, 137),
            },

            ThemeKind::Dracula => Theme {
                kind,
                bg: Color::Rgb(40, 42, 54),            // #282a36
                fg: Color::Rgb(248, 248, 242),         // #f8f8f2
                primary: Color::Rgb(189, 147, 249),    // Purple #bd93f9
                secondary: Color::Rgb(255, 121, 198),  // Pink #ff79c6
                accent: Color::Rgb(139, 233, 253),     // Cyan #8be9fd
                success: Color::Rgb(80, 250, 123),     // Green #50fa7b
                warning: Color::Rgb(241, 250, 140),    // Yellow #f1fa8c
                error: Color::Rgb(255, 85, 85),        // Red #ff5555
                border: Color::Rgb(98, 114, 164),      // Comment #6272a4
                border_focused: Color::Rgb(189, 147, 249),
                user_msg_bg: Color::Rgb(68, 71, 90),
                assistant_msg_bg: Color::Rgb(48, 50, 64),
                tool_msg_bg: Color::Rgb(55, 57, 72),
                code_bg: Color::Rgb(30, 31, 41),
                status_bar_bg: Color::Rgb(25, 26, 34),
                status_bar_fg: Color::Rgb(248, 248, 242),
                selected_bg: Color::Rgb(68, 71, 90),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(98, 114, 164),
            },

            ThemeKind::RosePine => Theme {
                kind,
                bg: Color::Rgb(25, 23, 36),            // Base #191724
                fg: Color::Rgb(224, 222, 244),         // Text #e0def4
                primary: Color::Rgb(235, 188, 186),    // Rose #ebbcba
                secondary: Color::Rgb(196, 167, 231),  // Iris #c4a7e7
                accent: Color::Rgb(156, 207, 216),     // Foam #9ccfd8
                success: Color::Rgb(49, 116, 143),     // Pine #31748f
                warning: Color::Rgb(246, 193, 119),    // Gold #f6c177
                error: Color::Rgb(235, 111, 146),      // Love #eb6f92
                border: Color::Rgb(82, 79, 103),
                border_focused: Color::Rgb(235, 188, 186),
                user_msg_bg: Color::Rgb(38, 35, 58),
                assistant_msg_bg: Color::Rgb(31, 29, 46),
                tool_msg_bg: Color::Rgb(34, 32, 50),
                code_bg: Color::Rgb(20, 18, 30),
                status_bar_bg: Color::Rgb(15, 14, 23),
                status_bar_fg: Color::Rgb(224, 222, 244),
                selected_bg: Color::Rgb(64, 61, 82),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(110, 106, 134),
            },

            ThemeKind::Synthwave84 => Theme {
                kind,
                bg: Color::Rgb(38, 20, 56),            // #261438
                fg: Color::Rgb(249, 248, 245),         // #f9f8f5
                primary: Color::Rgb(255, 126, 219),    // Hot Neon Pink
                secondary: Color::Rgb(54, 243, 210),   // Electric Mint
                accent: Color::Rgb(254, 222, 93),      // Sunset Yellow
                success: Color::Rgb(114, 241, 184),    // Cyber Green
                warning: Color::Rgb(255, 142, 60),     // Neon Orange
                error: Color::Rgb(254, 68, 80),        // Laser Red
                border: Color::Rgb(97, 43, 115),
                border_focused: Color::Rgb(255, 126, 219),
                user_msg_bg: Color::Rgb(55, 30, 78),
                assistant_msg_bg: Color::Rgb(46, 24, 66),
                tool_msg_bg: Color::Rgb(50, 27, 72),
                code_bg: Color::Rgb(27, 13, 40),
                status_bar_bg: Color::Rgb(20, 9, 30),
                status_bar_fg: Color::Rgb(255, 126, 219),
                selected_bg: Color::Rgb(85, 42, 110),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(140, 90, 160),
            },

            ThemeKind::OneDark => Theme {
                kind,
                bg: Color::Rgb(40, 44, 52),            // #282c34
                fg: Color::Rgb(171, 178, 191),         // #abb2bf
                primary: Color::Rgb(97, 175, 239),     // Blue #61afef
                secondary: Color::Rgb(198, 120, 221),  // Purple #c678dd
                accent: Color::Rgb(86, 182, 194),      // Cyan #56b6c2
                success: Color::Rgb(152, 195, 121),    // Green #98c379
                warning: Color::Rgb(229, 192, 123),    // Yellow #e5c07b
                error: Color::Rgb(224, 108, 117),      // Red #e06c75
                border: Color::Rgb(75, 82, 99),
                border_focused: Color::Rgb(97, 175, 239),
                user_msg_bg: Color::Rgb(50, 56, 66),
                assistant_msg_bg: Color::Rgb(44, 49, 58),
                tool_msg_bg: Color::Rgb(47, 52, 62),
                code_bg: Color::Rgb(30, 33, 39),
                status_bar_bg: Color::Rgb(33, 37, 43),
                status_bar_fg: Color::Rgb(171, 178, 191),
                selected_bg: Color::Rgb(62, 68, 81),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(92, 99, 112),
            },

            ThemeKind::Nord => Theme {
                kind,
                bg: Color::Rgb(46, 52, 64),            // nord0 #2e3440
                fg: Color::Rgb(236, 239, 244),         // nord4 #eceff4
                primary: Color::Rgb(136, 192, 208),    // nord8 #88c0d0
                secondary: Color::Rgb(129, 161, 193),  // nord9 #81a1c1
                accent: Color::Rgb(143, 188, 187),     // nord7 #8fbcbb
                success: Color::Rgb(163, 190, 140),    // nord14 #a3be8c
                warning: Color::Rgb(235, 203, 139),    // nord13 #ebcb8b
                error: Color::Rgb(191, 97, 106),       // nord11 #bf616a
                border: Color::Rgb(76, 86, 106),
                border_focused: Color::Rgb(136, 192, 208),
                user_msg_bg: Color::Rgb(59, 66, 82),
                assistant_msg_bg: Color::Rgb(51, 58, 73),
                tool_msg_bg: Color::Rgb(54, 61, 76),
                code_bg: Color::Rgb(36, 41, 51),
                status_bar_bg: Color::Rgb(36, 41, 51),
                status_bar_fg: Color::Rgb(216, 222, 233),
                selected_bg: Color::Rgb(67, 76, 94),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(94, 106, 130),
            },

            ThemeKind::GruvboxDark => Theme {
                kind,
                bg: Color::Rgb(40, 40, 40),            // #282828
                fg: Color::Rgb(235, 219, 178),         // #ebdbb2
                primary: Color::Rgb(250, 189, 47),     // yellow #fabd2f
                secondary: Color::Rgb(254, 128, 25),   // orange #fe8019
                accent: Color::Rgb(131, 165, 152),     // blue #83a598
                success: Color::Rgb(184, 187, 38),     // green #b8bb26
                warning: Color::Rgb(250, 189, 47),
                error: Color::Rgb(251, 73, 52),        // red #fb4934
                border: Color::Rgb(102, 92, 84),
                border_focused: Color::Rgb(250, 189, 47),
                user_msg_bg: Color::Rgb(60, 56, 54),
                assistant_msg_bg: Color::Rgb(50, 48, 47),
                tool_msg_bg: Color::Rgb(55, 52, 50),
                code_bg: Color::Rgb(29, 32, 33),
                status_bar_bg: Color::Rgb(29, 32, 33),
                status_bar_fg: Color::Rgb(213, 196, 161),
                selected_bg: Color::Rgb(80, 73, 69),
                selected_fg: Color::Rgb(251, 241, 199),
                muted: Color::Rgb(146, 131, 116),
            },

            ThemeKind::Cyberpunk => Theme {
                kind,
                bg: Color::Rgb(13, 13, 26),            // #0d0d1a
                fg: Color::Rgb(230, 245, 255),         // #e6f5ff
                primary: Color::Rgb(0, 240, 255),      // Neon Cyan
                secondary: Color::Rgb(255, 0, 128),    // Hot Magenta
                accent: Color::Rgb(255, 230, 0),       // Electric Yellow
                success: Color::Rgb(0, 255, 159),      // Matrix Green
                warning: Color::Rgb(255, 170, 0),      // Neon Orange
                error: Color::Rgb(255, 42, 109),       // Crimson Pink
                border: Color::Rgb(45, 30, 75),
                border_focused: Color::Rgb(0, 240, 255),
                user_msg_bg: Color::Rgb(28, 20, 50),
                assistant_msg_bg: Color::Rgb(18, 16, 35),
                tool_msg_bg: Color::Rgb(25, 22, 45),
                code_bg: Color::Rgb(8, 8, 16),
                status_bar_bg: Color::Rgb(5, 5, 12),
                status_bar_fg: Color::Rgb(0, 240, 255),
                selected_bg: Color::Rgb(65, 35, 95),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(95, 80, 130),
            },

            ThemeKind::Monokai => Theme {
                kind,
                bg: Color::Rgb(39, 40, 34),            // #272822
                fg: Color::Rgb(248, 248, 242),         // #f8f8f2
                primary: Color::Rgb(102, 217, 239),    // Cyan #66d9ef
                secondary: Color::Rgb(249, 38, 114),   // Pink #f92672
                accent: Color::Rgb(166, 226, 46),      // Green #a6e22e
                success: Color::Rgb(166, 226, 46),
                warning: Color::Rgb(253, 151, 31),     // Orange #fd971f
                error: Color::Rgb(249, 38, 114),
                border: Color::Rgb(73, 72, 62),
                border_focused: Color::Rgb(102, 217, 239),
                user_msg_bg: Color::Rgb(62, 61, 50),
                assistant_msg_bg: Color::Rgb(48, 49, 42),
                tool_msg_bg: Color::Rgb(54, 55, 47),
                code_bg: Color::Rgb(28, 29, 24),
                status_bar_bg: Color::Rgb(28, 29, 24),
                status_bar_fg: Color::Rgb(248, 248, 242),
                selected_bg: Color::Rgb(85, 84, 72),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(117, 113, 94),
            },

            ThemeKind::Kanagawa => Theme {
                kind,
                bg: Color::Rgb(31, 31, 40),            // sumiInk3 #1f1f28
                fg: Color::Rgb(220, 215, 186),         // fujiWhite #dcd7ba
                primary: Color::Rgb(126, 156, 216),    // crystalBlue #7e9cd8
                secondary: Color::Rgb(149, 127, 184),  // oniViolet #957fb8
                accent: Color::Rgb(230, 195, 134),     // carpYellow #e6c386
                success: Color::Rgb(152, 187, 108),    // springGreen #98bb6c
                warning: Color::Rgb(255, 158, 100),    // surimiOrange #ff9e64
                error: Color::Rgb(232, 130, 137),      // autumnRed #e88289
                border: Color::Rgb(84, 84, 109),
                border_focused: Color::Rgb(126, 156, 216),
                user_msg_bg: Color::Rgb(42, 42, 55),
                assistant_msg_bg: Color::Rgb(34, 34, 45),
                tool_msg_bg: Color::Rgb(38, 38, 50),
                code_bg: Color::Rgb(22, 22, 29),
                status_bar_bg: Color::Rgb(22, 22, 29),
                status_bar_fg: Color::Rgb(200, 192, 160),
                selected_bg: Color::Rgb(54, 54, 70),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(114, 113, 133),
            },

            ThemeKind::AyuDark => Theme {
                kind,
                bg: Color::Rgb(15, 20, 25),            // #0f1419
                fg: Color::Rgb(230, 225, 207),         // #e6e1cf
                primary: Color::Rgb(57, 186, 230),     // #39bae6
                secondary: Color::Rgb(255, 180, 84),   // #ffb454
                accent: Color::Rgb(240, 113, 120),     // #f07178
                success: Color::Rgb(194, 217, 76),     // #c2d94c
                warning: Color::Rgb(255, 180, 84),
                error: Color::Rgb(255, 51, 51),
                border: Color::Rgb(45, 53, 64),
                border_focused: Color::Rgb(57, 186, 230),
                user_msg_bg: Color::Rgb(25, 33, 42),
                assistant_msg_bg: Color::Rgb(19, 24, 31),
                tool_msg_bg: Color::Rgb(22, 28, 36),
                code_bg: Color::Rgb(11, 14, 18),
                status_bar_bg: Color::Rgb(11, 14, 18),
                status_bar_fg: Color::Rgb(180, 185, 195),
                selected_bg: Color::Rgb(38, 48, 62),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(92, 103, 115),
            },

            ThemeKind::Palenight => Theme {
                kind,
                bg: Color::Rgb(41, 45, 62),            // #292d3e
                fg: Color::Rgb(191, 199, 213),         // #bfc7d5
                primary: Color::Rgb(130, 170, 255),    // #82aaff
                secondary: Color::Rgb(199, 146, 234),  // #c792ea
                accent: Color::Rgb(137, 221, 255),     // #89ddff
                success: Color::Rgb(195, 232, 141),    // #c3e88d
                warning: Color::Rgb(255, 203, 107),    // #ffcb6b
                error: Color::Rgb(255, 83, 112),       // #ff5370
                border: Color::Rgb(71, 75, 100),
                border_focused: Color::Rgb(130, 170, 255),
                user_msg_bg: Color::Rgb(51, 56, 78),
                assistant_msg_bg: Color::Rgb(44, 48, 67),
                tool_msg_bg: Color::Rgb(48, 52, 73),
                code_bg: Color::Rgb(30, 33, 46),
                status_bar_bg: Color::Rgb(30, 33, 46),
                status_bar_fg: Color::Rgb(191, 199, 213),
                selected_bg: Color::Rgb(65, 71, 98),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(103, 110, 149),
            },

            ThemeKind::SolarizedDark => Theme {
                kind,
                bg: Color::Rgb(0, 43, 54),              // base03 #002b36
                fg: Color::Rgb(131, 148, 150),          // base0 #839496
                primary: Color::Rgb(38, 139, 210),      // blue #268bd2
                secondary: Color::Rgb(108, 113, 196),   // violet #6c71c4
                accent: Color::Rgb(42, 161, 152),       // cyan #2aa198
                success: Color::Rgb(133, 153, 0),       // green #859900
                warning: Color::Rgb(181, 137, 0),       // yellow #b58900
                error: Color::Rgb(220, 50, 47),         // red #dc322f
                border: Color::Rgb(88, 110, 117),
                border_focused: Color::Rgb(38, 139, 210),
                user_msg_bg: Color::Rgb(7, 54, 66),
                assistant_msg_bg: Color::Rgb(3, 48, 60),
                tool_msg_bg: Color::Rgb(5, 50, 62),
                code_bg: Color::Rgb(0, 34, 43),
                status_bar_bg: Color::Rgb(0, 30, 38),
                status_bar_fg: Color::Rgb(147, 161, 161),
                selected_bg: Color::Rgb(7, 54, 66),
                selected_fg: Color::Rgb(253, 246, 227),
                muted: Color::Rgb(88, 110, 117),
            },

            ThemeKind::GitHubDarkDimmed => Theme {
                kind,
                bg: Color::Rgb(34, 39, 46),             // #22272e
                fg: Color::Rgb(173, 186, 199),          // #adbac7
                primary: Color::Rgb(83, 155, 245),      // #539bf5
                secondary: Color::Rgb(186, 142, 244),   // #ba8ef4
                accent: Color::Rgb(108, 182, 255),      // #6cb6ff
                success: Color::Rgb(87, 171, 90),       // #57ab5a
                warning: Color::Rgb(198, 153, 56),      // #c69938
                error: Color::Rgb(244, 112, 103),       // #f47067
                border: Color::Rgb(68, 76, 86),
                border_focused: Color::Rgb(83, 155, 245),
                user_msg_bg: Color::Rgb(45, 51, 59),
                assistant_msg_bg: Color::Rgb(38, 44, 52),
                tool_msg_bg: Color::Rgb(40, 46, 54),
                code_bg: Color::Rgb(28, 33, 40),
                status_bar_bg: Color::Rgb(28, 33, 40),
                status_bar_fg: Color::Rgb(173, 186, 199),
                selected_bg: Color::Rgb(55, 62, 71),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(118, 131, 144),
            },

            ThemeKind::EmeraldMatrix => Theme {
                kind,
                bg: Color::Rgb(8, 15, 12),              // Deep Obsidian
                fg: Color::Rgb(180, 240, 200),          // Matrix pale
                primary: Color::Rgb(0, 255, 136),       // Electric Emerald
                secondary: Color::Rgb(0, 200, 180),     // Cyber Teal
                accent: Color::Rgb(50, 255, 200),       // Bright Mint
                success: Color::Rgb(0, 255, 136),
                warning: Color::Rgb(230, 240, 50),      // Neon Lime
                error: Color::Rgb(255, 75, 90),
                border: Color::Rgb(20, 50, 35),
                border_focused: Color::Rgb(0, 255, 136),
                user_msg_bg: Color::Rgb(16, 32, 24),
                assistant_msg_bg: Color::Rgb(12, 22, 17),
                tool_msg_bg: Color::Rgb(14, 26, 20),
                code_bg: Color::Rgb(4, 9, 7),
                status_bar_bg: Color::Rgb(4, 8, 6),
                status_bar_fg: Color::Rgb(0, 255, 136),
                selected_bg: Color::Rgb(25, 60, 42),
                selected_fg: Color::Rgb(255, 255, 255),
                muted: Color::Rgb(60, 120, 85),
            },

            // ==========================================
            // 3. ELEGANT LIGHT THEMES
            // ==========================================
            ThemeKind::MinimalistLight => Theme {
                kind,
                bg: Color::Rgb(250, 250, 250),          // #fafafa
                fg: Color::Rgb(24, 24, 27),             // #18181b
                primary: Color::Rgb(37, 99, 235),       // Blue 600
                secondary: Color::Rgb(124, 58, 237),    // Violet 600
                accent: Color::Rgb(13, 148, 136),       // Teal 600
                success: Color::Rgb(22, 163, 74),       // Green 600
                warning: Color::Rgb(217, 119, 6),       // Amber 600
                error: Color::Rgb(220, 38, 38),         // Red 600
                border: Color::Rgb(228, 228, 231),
                border_focused: Color::Rgb(37, 99, 235),
                user_msg_bg: Color::Rgb(243, 244, 246),
                assistant_msg_bg: Color::Rgb(255, 255, 255),
                tool_msg_bg: Color::Rgb(241, 245, 249),
                code_bg: Color::Rgb(241, 245, 249),
                status_bar_bg: Color::Rgb(228, 228, 231),
                status_bar_fg: Color::Rgb(39, 39, 42),
                selected_bg: Color::Rgb(219, 234, 254),
                selected_fg: Color::Rgb(30, 58, 138),
                muted: Color::Rgb(113, 113, 122),
            },

            ThemeKind::CatppuccinLatte => Theme {
                kind,
                bg: Color::Rgb(239, 241, 245),          // Base #eff1f5
                fg: Color::Rgb(76, 79, 105),            // Text #4c4f69
                primary: Color::Rgb(30, 102, 245),      // Blue #1e66f5
                secondary: Color::Rgb(136, 57, 239),    // Mauve #8839ef
                accent: Color::Rgb(234, 118, 203),     // Pink #ea76cb
                success: Color::Rgb(64, 160, 43),       // Green #40a02b
                warning: Color::Rgb(223, 142, 29),      // Yellow #df8e1d
                error: Color::Rgb(210, 15, 57),         // Red #d20f39
                border: Color::Rgb(204, 208, 218),      // Surface2
                border_focused: Color::Rgb(30, 102, 245),
                user_msg_bg: Color::Rgb(230, 233, 239),
                assistant_msg_bg: Color::Rgb(245, 246, 250),
                tool_msg_bg: Color::Rgb(230, 233, 239),
                code_bg: Color::Rgb(220, 224, 232),
                status_bar_bg: Color::Rgb(220, 224, 232),
                status_bar_fg: Color::Rgb(76, 79, 105),
                selected_bg: Color::Rgb(204, 208, 218),
                selected_fg: Color::Rgb(30, 102, 245),
                muted: Color::Rgb(140, 143, 161),
            },

            ThemeKind::GitHubLight => Theme {
                kind,
                bg: Color::Rgb(255, 255, 255),          // Pure White
                fg: Color::Rgb(36, 41, 47),             // Slate Black #24292f
                primary: Color::Rgb(9, 105, 218),       // GitHub Blue #0969da
                secondary: Color::Rgb(130, 80, 223),    // Purple #8250df
                accent: Color::Rgb(5, 152, 108),        // Teal
                success: Color::Rgb(26, 127, 55),       // Green #1a7f37
                warning: Color::Rgb(154, 103, 0),       // Orange #9a6700
                error: Color::Rgb(207, 34, 46),         // Red #cf222e
                border: Color::Rgb(208, 215, 222),      // Gray border
                border_focused: Color::Rgb(9, 105, 218),
                user_msg_bg: Color::Rgb(246, 248, 250),
                assistant_msg_bg: Color::Rgb(255, 255, 255),
                tool_msg_bg: Color::Rgb(246, 248, 250),
                code_bg: Color::Rgb(246, 248, 250),
                status_bar_bg: Color::Rgb(246, 248, 250),
                status_bar_fg: Color::Rgb(36, 41, 47),
                selected_bg: Color::Rgb(221, 234, 254),
                selected_fg: Color::Rgb(9, 105, 218),
                muted: Color::Rgb(101, 109, 118),
            },

            ThemeKind::PaperwhiteMinimal => Theme {
                kind,
                bg: Color::Rgb(252, 252, 250),          // Cream Paper
                fg: Color::Rgb(33, 37, 41),             // Ink Charcoal
                primary: Color::Rgb(13, 110, 253),      // Cobalt
                secondary: Color::Rgb(111, 66, 193),    // Royal Purple
                accent: Color::Rgb(108, 117, 125),      // Slate
                success: Color::Rgb(25, 135, 84),
                warning: Color::Rgb(215, 140, 10),
                error: Color::Rgb(220, 53, 69),
                border: Color::Rgb(218, 222, 226),
                border_focused: Color::Rgb(33, 37, 41),
                user_msg_bg: Color::Rgb(245, 245, 242),
                assistant_msg_bg: Color::Rgb(255, 255, 255),
                tool_msg_bg: Color::Rgb(241, 243, 245),
                code_bg: Color::Rgb(241, 243, 245),
                status_bar_bg: Color::Rgb(238, 240, 242),
                status_bar_fg: Color::Rgb(33, 37, 41),
                selected_bg: Color::Rgb(225, 230, 238),
                selected_fg: Color::Rgb(13, 110, 253),
                muted: Color::Rgb(120, 128, 136),
            },

            ThemeKind::RosePineDawn => Theme {
                kind,
                bg: Color::Rgb(250, 244, 237),          // Base #faf4ed
                fg: Color::Rgb(87, 82, 121),            // Text #575279
                primary: Color::Rgb(215, 130, 126),     // Rose #d7827e
                secondary: Color::Rgb(144, 122, 169),   // Iris #907aa9
                accent: Color::Rgb(86, 148, 159),       // Foam #56949f
                success: Color::Rgb(40, 105, 131),      // Pine #286983
                warning: Color::Rgb(234, 157, 52),      // Gold #ea9d34
                error: Color::Rgb(180, 99, 122),        // Love #b4637a
                border: Color::Rgb(206, 202, 220),
                border_focused: Color::Rgb(215, 130, 126),
                user_msg_bg: Color::Rgb(242, 233, 222),
                assistant_msg_bg: Color::Rgb(255, 250, 245),
                tool_msg_bg: Color::Rgb(242, 233, 222),
                code_bg: Color::Rgb(242, 233, 222),
                status_bar_bg: Color::Rgb(235, 226, 215),
                status_bar_fg: Color::Rgb(87, 82, 121),
                selected_bg: Color::Rgb(223, 215, 205),
                selected_fg: Color::Rgb(87, 82, 121),
                muted: Color::Rgb(152, 147, 165),
            },

            ThemeKind::TokyoNightDay => Theme {
                kind,
                bg: Color::Rgb(225, 226, 231),          // #e1e2e7
                fg: Color::Rgb(52, 59, 88),             // #343b58
                primary: Color::Rgb(52, 84, 186),       // Blue #34548a
                secondary: Color::Rgb(152, 84, 220),    // Purple #9854dc
                accent: Color::Rgb(15, 125, 150),       // Teal #0f7d96
                success: Color::Rgb(99, 139, 43),       // Green #638b2b
                warning: Color::Rgb(143, 94, 21),       // Orange #8f5e15
                error: Color::Rgb(198, 67, 86),         // Red #c64356
                border: Color::Rgb(180, 182, 192),
                border_focused: Color::Rgb(52, 84, 186),
                user_msg_bg: Color::Rgb(210, 212, 220),
                assistant_msg_bg: Color::Rgb(235, 236, 240),
                tool_msg_bg: Color::Rgb(210, 212, 220),
                code_bg: Color::Rgb(210, 212, 220),
                status_bar_bg: Color::Rgb(205, 207, 216),
                status_bar_fg: Color::Rgb(52, 59, 88),
                selected_bg: Color::Rgb(190, 195, 210),
                selected_fg: Color::Rgb(52, 84, 186),
                muted: Color::Rgb(130, 135, 150),
            },

            ThemeKind::SolarizedLight => Theme {
                kind,
                bg: Color::Rgb(253, 246, 227),          // base3 #fdf6e3
                fg: Color::Rgb(101, 123, 131),          // base00 #657b83
                primary: Color::Rgb(38, 139, 210),      // blue #268bd2
                secondary: Color::Rgb(108, 113, 196),   // violet
                accent: Color::Rgb(42, 161, 152),       // cyan
                success: Color::Rgb(133, 153, 0),       // green
                warning: Color::Rgb(181, 137, 0),       // yellow
                error: Color::Rgb(220, 50, 47),         // red
                border: Color::Rgb(211, 203, 178),
                border_focused: Color::Rgb(38, 139, 210),
                user_msg_bg: Color::Rgb(238, 232, 213),
                assistant_msg_bg: Color::Rgb(245, 239, 220),
                tool_msg_bg: Color::Rgb(238, 232, 213),
                code_bg: Color::Rgb(238, 232, 213),
                status_bar_bg: Color::Rgb(238, 232, 213),
                status_bar_fg: Color::Rgb(101, 123, 131),
                selected_bg: Color::Rgb(211, 203, 178),
                selected_fg: Color::Rgb(7, 54, 66),
                muted: Color::Rgb(147, 161, 161),
            },
        }
    }

    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border_style(&self, focused: bool) -> Style {
        if focused {
            Style::default().fg(self.border_focused)
        } else {
            Style::default().fg(self.border)
        }
    }

    pub fn badge_style(&self, bg: Color, fg: Color) -> Style {
        Style::default().bg(bg).fg(fg).add_modifier(Modifier::BOLD)
    }
}
