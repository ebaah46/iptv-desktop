pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"
import "../components"

Item {
    id: root

    // ── Data ─────────────────────────────────────────────────────────
    property var channels: []
    property var categories: []

    // ── Filter state ─────────────────────────────────────────────────
    property string activeCategoryId: ""
    property string activeCategoryName: "All"
    property string searchQuery: ""

    // ── Chrome state ─────────────────────────────────────────────────
    property string activeTab: "Home"
    property int activeSidebarIndex: 2
    property bool loading: false
    property int channelCount: channels && channels.channelCount !== undefined ? channels.channelCount : 0

    readonly property int visibleCount: channels && channels.loadedCount !== undefined ? channels.loadedCount : 0
    readonly property int totalCount: channels && channels.channelCount !== undefined ? channels.channelCount : 0
    readonly property bool hasMore: channels && channels.hasMore !== undefined ? channels.hasMore : false

    // ── Status bar ───────────────────────────────────────────────────
    property bool hasContent: false
    property string nowPlaying: "Nothing playing"
    property string appVersion: "v0.1.0"
    property string coreVersion: "v0.1.0"
    property string streamQuality: "HD"

    // ── Signals ──────────────────────────────────────────────────────
    signal channelClicked(string channelId, string channelName)
    signal tabSelected(string tab)
    signal searchChanged(string query)
    signal categorySelected(string id, string name)
    signal sidebarSelected(int index)

    readonly property bool searching: searchQuery.trim() !== ""
    readonly property bool showMore: visibleCount < totalCount
    readonly property string moreLabel: qsTr("Show More (%1 of %2)").arg(Math.min(visibleCount, totalCount)).arg(totalCount)

    function applyFilters() {
        if (channels && typeof channels.applyFilters === "function")
            channels.applyFilters(searchQuery, activeCategoryId, "", "");
    }

    function loadMore() {
        if (channels && typeof channels.loadMore === "function")
            channels.loadMore();
    }

    Component.onCompleted: applyFilters()

    // Debounce typing so we don't run a full catalog query per keystroke.
    Timer {
        id: searchDebounce
        interval: 200
        repeat: false
        onTriggered: root.applyFilters()
    }

    Rectangle {
        anchors.fill: parent
        color: Theme.background

        ColumnLayout {
            anchors.fill: parent
            spacing: 0

            TopBar {
                Layout.fillWidth: true
                activeTab: root.activeTab
                searchQuery: root.searchQuery
                onTabSelected: tab => root.tabSelected(tab)
                onSearchChanged: query => {
                    root.searchQuery = query;
                    root.searchChanged(query);
                    searchDebounce.restart();
                }
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 0

                Sidebar {
                    Layout.fillHeight: true
                    activeIndex: root.activeSidebarIndex
                    onSelected: index => {
                        // SidebarEntry::Countries (3) navigates away; the Iced
                        // app leaves the highlight on the previous entry.
                        if (index !== 3)
                            root.activeSidebarIndex = index;
                        root.sidebarSelected(index);
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: Theme.background

                    BusyIndicator {
                        anchors.centerIn: parent
                        running: root.loading
                        visible: root.loading
                    }

                    ScrollView {
                        id: contentScroll
                        anchors.fill: parent
                        clip: true
                        visible: !root.loading
                        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

                        Column {
                            width: contentScroll.availableWidth
                            leftPadding: Theme.spacingXl
                            rightPadding: Theme.spacingXl
                            topPadding: Theme.spacingXl
                            bottomPadding: Theme.spacingXl
                            spacing: root.searching ? Theme.spacingMd : Theme.spacingXl

                            // ── Category tabs ───────────────────────────
                            CategoryTabs {
                                width: parent.width - parent.leftPadding - parent.rightPadding
                                visible: !root.searching
                                categories: root.categories
                                activeCategoryId: root.activeCategoryId
                                onSelected: (id, name) => {
                                    root.activeCategoryId = id;
                                    root.activeCategoryName = name;
                                    root.categorySelected(id, name);
                                    root.applyFilters();
                                }
                            }

                            // ── Category title ──────────────────────────
                            Text {
                                width: parent.width - parent.leftPadding - parent.rightPadding
                                visible: !root.searching
                                text: root.activeCategoryName
                                color: Theme.textPrimary
                                font.pixelSize: Theme.fontSizeXl
                                font.weight: Font.Bold
                                elide: Text.ElideRight
                            }

                            // ── Search heading ──────────────────────────
                            Text {
                                width: parent.width - parent.leftPadding - parent.rightPadding
                                visible: root.searching
                                text: qsTr("Search results for \"%1\"").arg(root.searchQuery)
                                color: Theme.textPrimary
                                font.pixelSize: Theme.fontSizeXl
                                font.weight: Font.Bold
                                elide: Text.ElideRight
                            }

                            Text {
                                width: parent.width - parent.leftPadding - parent.rightPadding
                                visible: root.searching
                                text: qsTr("%1 channels found").arg(root.totalCount)
                                color: Theme.textSecondary
                                font.pixelSize: Theme.fontSizeMd
                            }

                            // ── Channel grid ────────────────────────────
                            Item {
                                id: gridWrap
                                width: parent.width - parent.leftPadding - parent.rightPadding
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

                            // ── Show more ───────────────────────────────
                            Item {
                                width: parent.width - parent.leftPadding - parent.rightPadding
                                height: root.showMore ? moreButton.height : 0
                                visible: root.showMore

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
                                        text: root.moreLabel
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

            StatusBar {
                Layout.fillWidth: true
                hasContent: root.hasContent
                nowPlaying: root.nowPlaying
                appVersion: root.appVersion
                coreVersion: root.coreVersion
                channelCount: root.channelCount
                streamQuality: root.streamQuality
            }
        }
    }
}
