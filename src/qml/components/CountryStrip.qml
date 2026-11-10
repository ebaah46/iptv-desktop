pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import "../theme"


ListView {
    id: root

    property var countries: []
    property string selectedCode: ""

    signal countryClicked(string code, string name, string flagGlyph)

    orientation: ListView.Horizontal
    spacing: Theme.spacingSm
    clip: true
    boundsBehavior: Flickable.StopAtBounds
    implicitHeight: Theme.countryChipHeight

    model: root.countries

    ScrollBar.horizontal: ScrollBar {
        policy: ScrollBar.AsNeeded
    }

    delegate: CountryChip {
        required property string code
        required property string name
        required property string flag_url

        countryCode: code
        countryName: name
        // The model's flag_url is a flag emoji glyph, not a URL.
        flagGlyph: flag_url
        selected: root.selectedCode === code
        onClicked: (code, name, flag) => root.countryClicked(code, name, flag)
    }
}
