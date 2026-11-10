pragma Singleton

import QtQuick

// Single source of truth for colors, spacing, and sizing.
QtObject {
    // ── Backgrounds ──────────────────────────────────────────────────
    readonly property color background: "#13131a"
    readonly property color surface: "#1e1e2e"
    readonly property color surfaceHover: "#2a2a3e"
    readonly property color cardBg: "#1a1a2e"
    readonly property color sidebarBg: "#181825"
    readonly property color topbarBg: "#14141f"
    readonly property color statusbarBg: "#0e0e18"

    // ── Accents / interactive ────────────────────────────────────────
    readonly property color primary: "#1a73e8"
    readonly property color primaryHover: "#1557b0"
    readonly property color accent: "#ff6b35"
    readonly property color accentHover: "#e05520"
    readonly property color success: "#4caf50"

    // ── Text ─────────────────────────────────────────────────────────
    readonly property color textPrimary: "#ffffff"
    readonly property color textSecondary: "#a0a0b0"

    // ── Border ───────────────────────────────────────────────────────
    readonly property color borderColor: "#2a2a3e"

    // ── Progress ─────────────────────────────────────────────────────
    readonly property color progressBg: "#2a2a3e"
    readonly property color progressFill: "#1a73e8"

    // ── Player controls ──────────────────────────────────────────────
    // Opaque surface of the control bar.
    readonly property color controlsBg: "#121217"
    // Radius of the play/pause and transport buttons.
    readonly property real controlsRadius: 20
    // Icon size for player controls.
    readonly property real controlIconBox: 36
    // Rendered size of the icon glyph inside its box. Keep it a few px smaller
    // than controlIconBox so the glyph never touches the button edges.
    readonly property real controlIconGlyph: 20

    // ── Spacing ──────────────────────────────────────────────────────
    readonly property real spacingXs: 4
    readonly property real spacingSm: 8
    readonly property real spacingMd: 12
    readonly property real spacingLg: 16
    readonly property real spacingXl: 24

    // ── Border-radius ────────────────────────────────────────────────
    readonly property real radiusSm: 6
    readonly property real radiusMd: 10
    readonly property real radiusLg: 16

    // ── Font sizes ───────────────────────────────────────────────────
    readonly property real fontSizeSm: 11
    readonly property real fontSizeMd: 13
    readonly property real fontSizeLg: 15
    readonly property real fontSizeXl: 20

    // ── Component sizing ─────────────────────────────────────────────
    readonly property real sidebarWidth: 180
    readonly property real topbarHeight: 52
    readonly property real statusbarHeight: 48
    readonly property real channelCardWidth: 180
    readonly property real channelCardHeight: 210

    // Country card geometry: wider than tall, since a country row is text-only.
    readonly property real countryCardWidth: 180
    readonly property real countryCardHeight: 100

    // Compact chip used by the horizontal country strip on the Countries screen.
    // Deliberately short: the strip is a fixed-height band above the channel grid,
    // so every pixel here is a pixel the grid does not get.
    readonly property real countryChipWidth: 168
    readonly property real countryChipHeight: 44
    readonly property real countryFlagGlyph: 50
}
