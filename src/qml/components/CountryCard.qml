pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Layouts
import "../theme"


Rectangle {
    id: root

    property string countryCode: ""
    property string countryName: ""
    property var countryLanguages: []
    /// Flag emoji glyph (e.g. "🇫🇷"), not a URL.
    property string flagGlyph: ""
    property bool selected: false

    signal clicked(string code, string name, string flagGlyph)

    // Two languages max keeps every card the same height; the rest is a
    // tooltip concern, not a layout concern.
    function metaText() {
        var langs = root.countryLanguages;
        if (!langs || langs.length === 0)
            return root.countryCode;
        var parts = [];
        for (var i = 0; i < langs.length && i < 2; i++)
            parts.push(langs[i]);
        return parts.join(" · ");
    }

    implicitWidth: Theme.countryCardWidth
    implicitHeight: Theme.countryCardHeight
    radius: Theme.radiusMd
    color: mouse.pressed ? Theme.accent : mouse.containsMouse ? Theme.surfaceHover : Theme.cardBg
    border.width: selected ? 2 : 1.5
    border.color: selected ? Theme.accent : Theme.borderColor
    clip: true

    RowLayout {
        anchors.fill: parent
        anchors.margins: Theme.spacingMd
        spacing: Theme.spacingSm

        // The catalog's `flag_url` field is a flag emoji glyph (e.g. "🇫🇷"),
        // not a URL. Render it as text; an empty glyph degrades to the code so
        // the card never collapses.
        Item {
            Layout.preferredWidth: Theme.countryFlagGlyph + Theme.spacingSm
            Layout.preferredHeight: Theme.countryFlagGlyph
            Layout.alignment: Qt.AlignVCenter

            Text {
                anchors.centerIn: parent
                width: parent.width
                text: root.flagGlyph !== "" ? root.flagGlyph : root.countryCode
                color: Theme.textPrimary
                font.pixelSize: root.flagGlyph !== "" ? Theme.countryFlagGlyph : Theme.fontSizeLg
                font.weight: Font.DemiBold
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: Theme.spacingXs

            Text {
                Layout.fillWidth: true
                text: root.countryName
                color: Theme.textPrimary
                font.pixelSize: Theme.fontSizeLg
                font.weight: Font.DemiBold
                wrapMode: Text.WordWrap
                elide: Text.ElideRight
                maximumLineCount: 2
            }

            Text {
                Layout.fillWidth: true
                text: root.metaText()
                color: Theme.textSecondary
                font.pixelSize: Theme.fontSizeMd
                wrapMode: Text.WordWrap
                elide: Text.ElideRight
                maximumLineCount: 2
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
