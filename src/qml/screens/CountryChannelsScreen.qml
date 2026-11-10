pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"
import "../components"

/// Channels for a single country.
///
/// Reuses the paginated ChannelList with a country filter.
/// The owning screen passes in its own ChannelList instance so
/// the home filters are left untouched.
Item {
    id: root

    property var channels: []
    property string countryCode: ""
    property string countryName: ""
    property string flagGlyph: ""

    signal backRequested
    signal channelClicked(string channelId, string channelName)

    readonly property int visibleCount: channels && channels.loadedCount !== undefined ? channels.loadedCount : 0
    readonly property int totalCount: channels && channels.channelCount !== undefined ? channels.channelCount : 0
    readonly property bool hasMore: channels && channels.hasMore !== undefined ? channels.hasMore : false

    function applyFilters() {
        if (channels && typeof channels.applyFilters === "function")
            channels.applyFilters("", "", countryCode, "");
    }

    function loadMore() {
        if (channels && typeof channels.loadMore === "function")
            channels.loadMore();
    }

    Component.onCompleted: applyFilters()

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
            RowLayout {
                Layout.fillWidth: true
                spacing: Theme.spacingSm

                Text {
                    text: root.flagGlyph
                    color: Theme.textPrimary
                    font.pixelSize: Theme.fontSizeXl
                    visible: root.flagGlyph !== ""
                }

                Text {
                    Layout.fillWidth: true
                    text: root.countryName
                    color: Theme.textPrimary
                    font.pixelSize: Theme.fontSizeXl
                    elide: Text.ElideRight
                }
            }

            Text {
                Layout.fillWidth: true
                text: qsTr("%1 channels").arg(root.totalCount)
                color: Theme.textSecondary
                font.pixelSize: Theme.fontSizeMd
            }

            // ── Channel grid ─────────────────────────────────────────
            ScrollView {
                id: scroll
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

                Column {
                    width: scroll.availableWidth
                    spacing: Theme.spacingMd

                    Item {
                        id: gridWrap
                        width: parent.width
                        height: grid.height

                        ChannelGrid {
                            id: grid
                            anchors.horizontalCenter: parent.horizontalCenter
                            spacing: Theme.spacingMd
                            columns: Math.max(1, Math.floor((gridWrap.width + spacing) / (Theme.channelCardWidth + spacing)))
                            channels: root.channels
                            onChannelClicked: (channelId, channelName) => root.channelClicked(channelId, channelName)
                        }
                    }

                    // ── Show more ───────────────────────────────────────
                    Item {
                        width: parent.width
                        height: root.hasMore ? moreButton.height : 0
                        visible: root.hasMore

                        Rectangle {
                            id: moreButton
                            anchors.horizontalCenter: parent.horizontalCenter
                            width: moreText.implicitWidth + 2 * Theme.spacingLg
                            height: 32
                            radius: Theme.radiusSm
                            color: moreMouse.containsMouse ? Theme.surfaceHover : Theme.surface
                            border.width: 1
                            border.color: Theme.borderColor

                            Text {
                                id: moreText
                                anchors.centerIn: parent
                                text: qsTr("Show More (%1 of %2)").arg(Math.min(root.visibleCount, root.totalCount)).arg(root.totalCount)
                                color: Theme.textPrimary
                                font.pixelSize: Theme.fontSizeMd
                            }

                            MouseArea {
                                id: moreMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: root.loadMore()
                            }
                        }
                    }
                }
            }
        }
    }
}
