pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import "../theme"


Item {
    id: root

    property var categories: []
    property int topCount: 10
    /// Selected category id. Empty string means "All".
    property string activeCategoryId: ""

    /// Emitted with the selected category id ("" for All) and its display name.
    signal selected(string id, string name)

    implicitWidth: 1024
    implicitHeight: 40

    ScrollView {
        anchors.fill: parent
        anchors.topMargin: 4
        anchors.bottomMargin: 4
        clip: true
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AlwaysOff

        Row {
            height: 32
            spacing: Theme.spacingSm

            Rectangle {
                id: allTab
                width: 100
                height: 32
                radius: Theme.radiusMd
                color: root.activeCategoryId === "" ? Theme.accent : Theme.surface
                border.width: 1
                border.color: root.activeCategoryId === "" ? Theme.accent : Theme.borderColor

                Text {
                    anchors.centerIn: parent
                    text: qsTr("All")
                    color: root.activeCategoryId === "" ? Theme.textPrimary : Theme.textSecondary
                    font.pixelSize: Theme.fontSizeMd
                    font.weight: root.activeCategoryId === "" ? Font.DemiBold : Font.Normal
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.selected("", "All")
                }
            }

            Repeater {
                model: root.categories

                delegate: Rectangle {
                    id: tab
                    required property int index
                    required property string id
                    required property string name

                    readonly property bool active: root.activeCategoryId === id

                    visible: index < root.topCount
                    width: 100
                    height: 32
                    radius: Theme.radiusMd
                    color: active ? Theme.accent : Theme.surface
                    border.width: 1
                    border.color: active ? Theme.accent : Theme.borderColor

                    Text {
                        anchors.centerIn: parent
                        text: tab.name
                        color: tab.active ? Theme.textPrimary : Theme.textSecondary
                        font.pixelSize: Theme.fontSizeMd
                        font.weight: tab.active ? Font.DemiBold : Font.Normal
                        elide: Text.ElideRight
                        width: parent.width - 8
                        horizontalAlignment: Text.AlignHCenter
                    }

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.selected(tab.id, tab.name)
                    }
                }
            }
        }
    }
}
