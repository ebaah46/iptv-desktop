pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Layouts
import "../theme"

// Migrated from src/ui/components/status_bar.rs.
// The volume icon at the bottom right is intentionally omitted.
Rectangle {
    id: root

    property string nowPlaying: "Nothing playing"
    property bool hasContent: false
    property string appVersion: "v0.1.0"
    property string coreVersion: "v0.1.0"
    property int channelCount: 0
    property string streamQuality: "HD"

    implicitWidth: 1024
    implicitHeight: Theme.statusbarHeight
    color: Theme.statusbarBg

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Theme.spacingLg
        anchors.rightMargin: Theme.spacingLg
        spacing: Theme.spacingMd

        // ── Left section ────────────────────────────────────────────
        RowLayout {
            Layout.alignment: Qt.AlignVCenter
            spacing: Theme.spacingSm

            Rectangle {
                Layout.alignment: Qt.AlignVCenter
                Layout.preferredWidth: 8
                Layout.preferredHeight: 8
                color: root.hasContent ? Theme.success : Theme.textSecondary
            }

            Text {
                Layout.alignment: Qt.AlignVCenter
                text: root.hasContent ? qsTr("Now Playing:") : qsTr("Standby")
                color: Theme.textSecondary
                font.pixelSize: Theme.fontSizeSm
            }

            Text {
                Layout.alignment: Qt.AlignVCenter
                text: root.nowPlaying
                color: Theme.textPrimary
                font.pixelSize: Theme.fontSizeMd
            }
        }

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        // ── Right section (badges only) ─────────────────────────────
        RowLayout {
            Layout.alignment: Qt.AlignVCenter
            spacing: Theme.spacingSm

            Repeater {
                model: [root.streamQuality, root.channelCount + " Ch", "App " + root.appVersion, "Core " + root.coreVersion]

                delegate: Rectangle {
                    id: badge
                    required property string modelData

                    Layout.preferredWidth: badgeText.implicitWidth + 16
                    Layout.preferredHeight: 22
                    color: Theme.surface
                    radius: Theme.radiusSm
                    border.width: 1
                    border.color: Theme.borderColor

                    Text {
                        id: badgeText
                        anchors.centerIn: parent
                        text: badge.modelData
                        color: Theme.textSecondary
                        font.pixelSize: Theme.fontSizeSm
                    }
                }
            }
        }
    }
}
