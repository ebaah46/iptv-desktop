import QtQuick
import "../theme"

/// Square, padding-free icon button matching the Iced `icon_button` helper:
/// a fixed 36x36 box with a centred 20px glyph.
Rectangle {
    id: root

    property string iconPath: ""
    property color iconColor: Theme.textPrimary
    property color baseColor: Theme.surface
    property color hoverColor: Theme.surfaceHover
    property color pressColor: Theme.primaryHover
    property real cornerRadius: Theme.radiusSm

    signal clicked

    implicitWidth: Theme.controlIconBox
    implicitHeight: Theme.controlIconBox
    radius: cornerRadius
    color: mouse.pressed ? pressColor : mouse.containsMouse ? hoverColor : baseColor

    Icon {
        anchors.centerIn: parent
        width: Theme.controlIconGlyph
        height: Theme.controlIconGlyph
        path: root.iconPath
        color: root.iconColor
    }

    MouseArea {
        id: mouse
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked()
    }
}
