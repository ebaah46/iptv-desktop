pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import iptv_desktop
import "screens"

ApplicationWindow {
    id: root
    width: 1024
    height: 720
    visible: true
    title: "IPTV"

    required property var appState
    required property var player

    // 1 = Home, 2 = Countries, 3 = Country channels, 4 = Player.
    // Index 0 is loading.
    property int page: 1
    readonly property int currentPage: root.appState.ready ? root.page : 0

    // Selection carried from the Countries page into the country channels page.
    property string selectedCountryCode: ""
    property string selectedCountryName: ""
    property string selectedCountryFlag: ""

    // Selection carried into the player page, plus where Back should return to.
    property string selectedChannelId: ""
    property string selectedChannelName: ""
    property int playerReturnPage: 1

    ChannelList {
        id: channelList
        onFavoritesChanged: countryChannelList.refreshFavorites()
    }

    // Separate model so the country page does not clobber the home filters.
    ChannelList {
        id: countryChannelList
        onFavoritesChanged: channelList.refreshFavorites()
    }

    StackLayout {
        anchors.fill: parent
        currentIndex: root.currentPage

        LoadingScreen {}

        // Defer creation until the catalog is ready so the CategoryList
        // singleton is populated when the screen is instantiated.
        Loader {
            active: root.appState.ready
            sourceComponent: HomeScreen {
                channels: channelList
                categories: CategoryList
                onSidebarSelected: index => {
                    // SidebarEntry::Countries is index 3.
                    if (index === 3)
                        root.page = 2;
                }
                onChannelClicked: (channelId, channelName) => {
                    root.player.clearSource();
                    root.selectedChannelId = channelId;
                    root.selectedChannelName = channelName;
                    root.playerReturnPage = 1;
                    root.page = 4;
                }
            }
        }

        // Created on first visit; CountryList is a singleton backed by the
        // catalog, so it is safe to reference once the app is ready.
        Loader {
            active: root.page === 2
            sourceComponent: CountriesScreen {
                countries: CountryList
                selectedCountry: root.selectedCountryCode
                onBackRequested: root.page = 1
                onCountrySelected: (code, name, flag) => {
                    root.selectedCountryCode = code;
                    root.selectedCountryName = name;
                    root.selectedCountryFlag = flag;
                    root.page = 3;
                }
            }
        }

        Loader {
            active: root.page === 3
            sourceComponent: CountryChannelsScreen {
                channels: countryChannelList
                countryCode: root.selectedCountryCode
                countryName: root.selectedCountryName
                flagGlyph: root.selectedCountryFlag
                onBackRequested: root.page = 2
                onChannelClicked: (channelId, channelName) => {
                    root.player.clearSource();
                    root.selectedChannelId = channelId;
                    root.selectedChannelName = channelName;
                    root.playerReturnPage = 3;
                    root.page = 4;
                }
            }
        }

        Loader {
            active: root.page === 4
            sourceComponent: PlayerScreen {
                player: root.player
                channelId: root.selectedChannelId
                channelName: root.selectedChannelName
                onBackRequested: root.page = root.playerReturnPage
            }
        }
    }

    Component.onCompleted: root.appState.load()
}
