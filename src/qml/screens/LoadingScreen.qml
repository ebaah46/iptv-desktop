import QtQuick
import QtQuick.Controls
import "../theme"

// Shown while the catalog is loading. Uses the same background as the ready
// state so the hand-off into HomeScreen is seamless.
Rectangle {
    color: Theme.background

    BusyIndicator {
        anchors.centerIn: parent
        running: true
    }
}
