pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Layouts
import "../theme"


Rectangle {
    id: root

    property string countryCode: ""
    property string countryName: ""
    /// Flag emoji glyph (e.g. "🇫🇷"), not a URL.
    property string flagGlyph: ""
    property bool selected: false

    signal clicked(string code, string name, string flagGlyph)

    implicitWidth: Theme.countryChipWidth
    implicitHeight: Theme.countryChipHeight
    radius: Theme.radiusMd
    color: mouse.pressed ? Theme.accent : mouse.containsMouse ? Theme.surfaceHover : selected ? Theme.surface : Theme.cardBg
    border.width: selected ? 2 : 1
    border.color: selected ? Theme.accent : Theme.borderColor
    clip: true

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Theme.spacingSm
        anchors.rightMargin: Theme.spacingSm
        anchors.topMargin: Theme.spacingXs
        anchors.bottomMargin: Theme.spacingXs
        spacing: Theme.spacingSm

        Text {
            Layout.alignment: Qt.AlignVCenter
            text: root.flagGlyph
            color: Theme.textPrimary
            font.pixelSize: Theme.fontSizeLg
            visible: root.flagGlyph !== ""
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: 0

            Text {
                Layout.fillWidth: true
                text: root.countryName
                color: Theme.textPrimary
                font.pixelSize: Theme.fontSizeMd
                elide: Text.ElideRight
            }

            Text {
                Layout.fillWidth: true
                text: root.countryCode
                color: Theme.textSecondary
                font.pixelSize: Theme.fontSizeSm
                elide: Text.ElideRight
            }
        }
    }

    MouseArea {
        id: mouse
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked(root.countryCode, root.countryName, root.flagGlyph)
    }
}
