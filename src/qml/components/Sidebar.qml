pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"

Rectangle {
    id: root

    /// All category items.
    property var items: ["Favorites", "Recent", "All", "Countries", "Kids", "Settings"]
    /// Index of the currently selected item.
    property int activeIndex: 2
    /// Number of favorite channels, shown as a badge on the Favorites row.
    property int favoriteCount: 0

    signal selected(int index)

    implicitWidth: Theme.sidebarWidth
    implicitHeight: 640
    color: Theme.sidebarBg

    ColumnLayout {
        anchors.fill: parent
        spacing: Theme.spacingMd

        Text {
            Layout.fillWidth: true
            Layout.topMargin: Theme.spacingLg
            text: qsTr("Categories")
            color: Theme.textPrimary
            font.pixelSize: 21
            horizontalAlignment: Text.AlignHCenter
        }

        ScrollView {
            id: scroll
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.bottomMargin: Theme.spacingLg
            clip: true
            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

            Column {
                id: list
                width: scroll.availableWidth
                spacing: 4
                leftPadding: Theme.spacingSm
                rightPadding: Theme.spacingSm

                Repeater {
                    model: root.items

                    delegate: Rectangle {
                        id: row
                        required property int index
                        required property string modelData

                        readonly property bool active: index === root.activeIndex

                        width: list.width - list.leftPadding - list.rightPadding
                        height: 36
                        color: mouse.pressed ? Theme.surface : mouse.containsMouse ? Theme.surfaceHover : (active ? Theme.sidebarBg : "transparent")

                        Text {
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.left: parent.left
                            anchors.leftMargin: 10
                            text: row.modelData
                            color: row.active ? Theme.textPrimary : Theme.textSecondary
                            font.pixelSize: Theme.fontSizeMd
                            font.weight: row.active ? Font.DemiBold : Font.Normal
                        }

                        Rectangle {
                            visible: row.index === 0 && root.favoriteCount > 0
                            anchors.right: parent.right
                            anchors.rightMargin: 8
                            anchors.verticalCenter: parent.verticalCenter
                            width: favoriteCountLabel.implicitWidth + 8
                            height: 18
                            radius: 9
                            color: Theme.accent

                            Text {
                                id: favoriteCountLabel
                                anchors.centerIn: parent
                                text: root.favoriteCount
                                color: Theme.textPrimary
                                font.pixelSize: Theme.fontSizeSm
                            }
                        }

                        MouseArea {
                            id: mouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: root.selected(row.index)
                        }
                    }
                }
            }
        }
    }
}
