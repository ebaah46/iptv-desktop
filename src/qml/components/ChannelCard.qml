pragma ComponentBehavior: Bound

import QtQuick
import "../theme"


Rectangle {
    id: root

    property string channelId: ""
    property string channelName: ""
    property string logoSource: ""

    signal clicked(string channelId, string channelName)

    readonly property real logoSize: Theme.channelCardHeight * 0.6
    readonly property real progress: 0.8

    // A channel without a logo exposes `undefined` for its logo role. Assigning
    // that to `Image.source` makes QML resolve a bogus "<dir>/undefined" URL,
    // so normalise anything that is not a real string to an empty source.
    readonly property string resolvedLogoSource: (typeof logoSource === "string" && logoSource !== "undefined") ? logoSource : ""

    implicitWidth: Theme.channelCardWidth
    implicitHeight: Theme.channelCardHeight
    radius: Theme.radiusLg
    color: mouse.pressed ? Theme.accent : mouse.containsMouse ? Theme.surfaceHover : Theme.cardBg
    border.width: 1.5
    border.color: Theme.borderColor
    clip: true

    Column {
        anchors.fill: parent
        anchors.margins: Theme.spacingMd
        spacing: Theme.spacingSm

        Item {
            width: parent.width
            height: root.logoSize

            Image {
                id: logoImage
                anchors.centerIn: parent
                width: root.logoSize
                height: root.logoSize
                source: root.resolvedLogoSource
                asynchronous: true
                sourceSize.width: root.logoSize
                sourceSize.height: root.logoSize
                fillMode: Image.PreserveAspectFit
                visible: status === Image.Ready
            }

            // Fallback for a missing, still-loading, or broken logo: a neutral
            // tile showing the channel's initial (or a TV glyph when unnamed).
            Rectangle {
                anchors.centerIn: parent
                width: root.logoSize
                height: root.logoSize
                radius: Theme.radiusMd
                color: Theme.surfaceHover
                border.width: 1
                border.color: Theme.borderColor
                visible: logoImage.status !== Image.Ready

                Text {
                    anchors.centerIn: parent
                    text: root.channelName.length > 0 ? root.channelName.charAt(0).toUpperCase() : "📺"
                    color: Theme.accent
                    font.pixelSize: Math.round(root.logoSize * 0.4)
                    font.weight: Font.Bold
                }
            }
        }

        Text {
            width: parent.width
            text: root.channelName
            color: Theme.textPrimary
            font.pixelSize: Theme.fontSizeLg
            font.weight: Font.DemiBold
            elide: Text.ElideRight
        }

        Text {
            width: parent.width
            text: qsTr("Now Playing…")
            color: Theme.textSecondary
            font.pixelSize: Theme.fontSizeMd
            elide: Text.ElideRight
        }

        Rectangle {
            width: parent.width
            height: 4
            radius: 2
            color: Theme.progressBg

            Rectangle {
                width: parent.width * root.progress
                height: parent.height
                radius: 2
                color: Theme.progressFill
            }
        }
    }

    MouseArea {
        id: mouse
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked(root.channelId, root.channelName)
    }
}
