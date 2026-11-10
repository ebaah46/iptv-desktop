pragma ComponentBehavior: Bound

import QtQuick
import "../theme"


Grid {
    id: root

    property var countries: []
    /// Code of the currently selected country, drawn with an accent border.
    property string selectedCode: ""

    signal countryClicked(string code, string name, string flagGlyph)

    spacing: Theme.spacingMd

    Repeater {
        model: root.countries

        delegate: CountryCard {
            // QModelItem roles use the raw Rust field names (see CountryItem).
            required property string code
            required property string name
            required property var languages
            required property string flag_url

            countryCode: code
            countryName: name
            countryLanguages: languages
            // `flag_url` is a flag emoji glyph (not a URL).
            flagGlyph: flag_url
            selected: root.selectedCode === code
            onClicked: (code, name, flag) => root.countryClicked(code, name, flag)
        }
    }
}
