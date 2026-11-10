pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"
import "../components"

// Standalone full-screen page (no sidebar/top bar), matching the Iced app
// where `Page::Country` renders only `countries::view`.
Item {
    id: root

    property var countries: []
    property string selectedCountry: ""

    signal backRequested
    signal countrySelected(string code, string name, string flagGlyph)

    Rectangle {
        anchors.fill: parent
        color: Theme.background

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Theme.spacingXl
            spacing: Theme.spacingMd

            // ── Back button ──────────────────────────────────────────
            Rectangle {
                id: backButton
                Layout.preferredWidth: backLabel.implicitWidth + 2 * Theme.spacingLg
                Layout.preferredHeight: backLabel.implicitHeight + 2 * Theme.spacingSm
                radius: Theme.radiusSm
                color: backMouse.containsMouse ? Theme.surfaceHover : Theme.surface
                border.width: 1
                border.color: Theme.borderColor

                Text {
                    id: backLabel
                    anchors.centerIn: parent
                    text: qsTr("← Back")
                    color: Theme.textPrimary
                    font.pixelSize: Theme.fontSizeMd
                }

                MouseArea {
                    id: backMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.backRequested()
                }
            }

            // ── Title ────────────────────────────────────────────────
            Text {
                Layout.fillWidth: true
                text: qsTr("Available Countries")
                color: Theme.textPrimary
                font.pixelSize: Theme.fontSizeXl
            }

            // ── Country grid ─────────────────────────────────────────
            ScrollView {
                id: scroll
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

                CountryGrid {
                    id: grid
                    width: scroll.availableWidth
                    spacing: Theme.spacingMd
                    columns: Math.max(1, Math.floor((grid.width + grid.spacing) / (Theme.countryCardWidth + grid.spacing)))
                    countries: root.countries
                    selectedCode: root.selectedCountry
                    onCountryClicked: (code, name, flag) => {
                        root.selectedCountry = code;
                        root.countrySelected(code, name, flag);
                    }
                }
            }
        }
    }
}
