pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"

Rectangle {
    id: root

    property string activeTab: "Home"
    property string searchQuery: ""
    /// Default navigation tabs shown in the top bar.
    property var tabs: ["Home", "Guide", "Movies", "Series", "Recordings"]

    signal tabSelected(string tab)
    signal searchChanged(string query)

    // Keep the field in sync with programmatic changes from the owner.
    // User edits arrive via onTextEdited below.
    onSearchQueryChanged: {
        if (searchField.text !== searchQuery)
            searchField.text = searchQuery;
    }

    implicitWidth: 1024
    implicitHeight: Theme.topbarHeight
    color: Theme.topbarBg

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // ── Logo area ────────────────────────────────────────────────
        RowLayout {
            Layout.alignment: Qt.AlignVCenter
            Layout.leftMargin: Theme.spacingLg
            Layout.rightMargin: Theme.spacingLg
            spacing: Theme.spacingSm

            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                color: Theme.accent
            }

            Text {
                Layout.alignment: Qt.AlignVCenter
                text: "IPTV"
                color: Theme.textPrimary
                font.pixelSize: Theme.fontSizeLg
                font.weight: Font.Bold
            }
        }

        // ── Navigation tabs ──────────────────────────────────────────
        RowLayout {
            Layout.alignment: Qt.AlignVCenter
            spacing: Theme.spacingXs

            Repeater {
                model: root.tabs

                delegate: Rectangle {
                    id: tab
                    required property int index
                    required property string modelData

                    readonly property bool active: modelData === root.activeTab

                    Layout.preferredHeight: 36
                    implicitWidth: tabLabel.implicitWidth + 20 // Iced button default horizontal padding
                    radius: Theme.radiusSm
                    color: active ? Theme.primary : "transparent"

                    Text {
                        id: tabLabel
                        anchors.centerIn: parent
                        text: tab.modelData
                        color: tab.active ? Theme.textPrimary : Theme.textSecondary
                        font.pixelSize: Theme.fontSizeMd
                        font.weight: tab.active ? Font.DemiBold : Font.Normal
                    }

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.tabSelected(tab.modelData)
                    }
                }
            }
        }

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        // ── Search + settings ────────────────────────────────────────
        RowLayout {
            Layout.alignment: Qt.AlignVCenter
            Layout.rightMargin: Theme.spacingLg
            spacing: Theme.spacingSm

            Rectangle {
                Layout.preferredWidth: 250
                Layout.preferredHeight: 32
                color: Theme.surface
                radius: Theme.radiusSm

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: Theme.spacingSm
                    anchors.rightMargin: Theme.spacingSm
                    spacing: Theme.spacingSm

                    Text {
                        Layout.alignment: Qt.AlignVCenter
                        text: "🔍"
                        color: Theme.textPrimary
                        font.pixelSize: Theme.fontSizeMd
                    }

                    TextField {
                        id: searchField
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        text: root.searchQuery
                        placeholderText: qsTr("Enter channel search here...")
                        placeholderTextColor: Theme.textSecondary
                        color: Theme.textPrimary
                        font.pixelSize: Theme.fontSizeMd
                        verticalAlignment: TextInput.AlignVCenter
                        leftPadding: 0
                        rightPadding: 0
                        topPadding: 0
                        bottomPadding: 0
                        selectByMouse: true
                        background: Item {}
                        onTextEdited: root.searchChanged(text)
                    }
                }
            }

            Item {
                Layout.preferredWidth: 32
                Layout.preferredHeight: 32

                Text {
                    anchors.centerIn: parent
                    text: "⚙️"
                    color: Theme.textPrimary
                    font.pixelSize: Theme.fontSizeMd
                }
            }
        }
    }
}
