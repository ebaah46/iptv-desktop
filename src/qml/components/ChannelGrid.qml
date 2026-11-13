pragma ComponentBehavior: Bound

import QtQuick
import "../theme"


Grid {
    id: root

    property var channels: []

    signal channelClicked(string channelId, string channelName)
    signal favoriteToggled(string channelId)

    spacing: Theme.spacingMd

    Repeater {
        model: root.channels

        delegate: ChannelCard {
            required property string name
            required property string id
            // `var`, not `string`: channels without a logo expose an invalid
            // variant for this role, which QML surfaces as `undefined`.
            required property var logoUrl
            required property bool isFavorite

            channelId: id
            channelName: name
            logoSource: logoUrl ?? ""
            favorite: isFavorite
            onClicked: (channelId, channelName) => root.channelClicked(channelId, channelName)
            onFavoriteToggled: channelId => root.favoriteToggled(channelId)
        }
    }
}
