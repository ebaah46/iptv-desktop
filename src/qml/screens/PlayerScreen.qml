pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import QtMultimedia
import "../theme"
import "../components"

/// Here QtMultimedia's `MediaPlayer` owns playback.
/// The Rust `QmlPlayer` bridges commands and state
/// to the core: `playChannel()` resolves a stream, `source` carries the URL back
/// to QML, and the `*Requested` signals drive the MediaPlayer. UI-only state
/// (position, volume, controls visibility) lives here.
Item {
    id: root

    property var player: null
    property string channelId: ""
    property string channelName: ""

    signal backRequested

    // ── Playback state ───────────────────────────────────────────────
    property bool isPlaying: false
    property bool isLoading: false
    property string errorMessage: ""
    property real positionSecs: 0
    property real durationSecs: 0
    property real volume: 1.0
    property bool isMuted: false
    property bool isFullscreen: false
    property bool controlsVisible: true

    readonly property bool hasDuration: durationSecs > 0
    // Keep the controls reachable even while an error is shown: the core may
    // still be retrying candidates, and the user needs play/back regardless.
    readonly property bool showControls: controlsVisible

    // True once playback has actually started for the current channel. Lets the
    // start-up timeout distinguish "never started" from "user paused".
    property bool hasStarted: false

    // Mirrors AUTO_HIDE_TICKS (6) 
    readonly property int autoHideMs: 3000
    readonly property int skipSecs: 10
    // How long to wait for playback to start before surfacing a silent failure.
    readonly property int playbackTimeoutMs: 15000

    // ── Auto-hide ────────────────────────────────────────────────────
    Timer {
        id: autoHideTimer
        interval: root.autoHideMs
        repeat: false
        onTriggered: {
            if (root.isPlaying)
                root.controlsVisible = false;
        }
    }

    // Some failures are silent: the core can find no candidate stream, or the
    // media never loads, without MediaPlayer emitting errorOccurred. Surface
    // those instead of leaving the spinner up forever.
    Timer {
        id: playbackTimeout
        interval: root.playbackTimeoutMs
        repeat: false
        onTriggered: {
            // Only a channel that never started is a failure. If the user
            // paused after playback began, `hasStarted` is already true.
            if (!root.hasStarted && !root.isPlaying && root.errorMessage === "") {
                root.isLoading = false;
                root.errorMessage = qsTr("Playback did not start. Press play to retry.");
                root.controlsVisible = true;
            }
        }
    }

    function revealControls() {
        root.controlsVisible = true;
        if (root.isPlaying)
            autoHideTimer.restart();
    }

    function formatTime(secs) {
        const total = Math.max(0, Math.floor(secs));
        const minutes = Math.floor(total / 60);
        const seconds = total % 60;
        return minutes + ":" + (seconds < 10 ? "0" + seconds : seconds);
    }

    function startPlayback() {
        if (!root.player || root.channelId === "")
            return;
        root.isLoading = true;
        root.errorMessage = "";
        root.hasStarted = false;
        playbackTimeout.restart();
        root.player.playChannel(root.channelId);
    }

    function togglePlayPause() {
        if (root.isPlaying) {
            media.pause();
        } else if (root.errorMessage !== "" || media.source.toString() === "") {
            // A failed source cannot be resumed; re-resolve the channel instead.
            root.startPlayback();
        } else {
            media.play();
        }
        root.revealControls();
    }

    function seekBy(deltaSecs) {
        if (!root.hasDuration)
            return;
        const target = Math.max(0, Math.min(root.durationSecs, root.positionSecs + deltaSecs));
        media.position = target * 1000;
        root.revealControls();
    }

    function toggleFullscreen() {
        const win = root.Window.window;
        if (!win)
            return;
        root.isFullscreen = !root.isFullscreen;
        win.visibility = root.isFullscreen ? Window.FullScreen : Window.Windowed;
        root.revealControls();
    }

    function goBack() {
        playbackTimeout.stop();
        media.stop();
        root.backRequested();
    }

    Component.onCompleted: {
        // Drop the previous channel's URL so the new MediaPlayer does not start
        // loading a stale source before the core resolves this channel.
        if (root.player)
            root.player.clearSource();
        root.startPlayback();
    }

    Component.onDestruction: {
        playbackTimeout.stop();
        media.stop();
        media.source = "";
    }

    // ── Media ────────────────────────────────────────────────────────
    MediaPlayer {
        id: media
        source: root.player && root.player.source ? root.player.source : ""
        videoOutput: videoOut

        audioOutput: AudioOutput {
            volume: root.isMuted ? 0 : root.volume
        }

        onPlaybackStateChanged: {
            if (root.player)
                root.player.onPlaybackStateChanged(playbackState);

            root.isPlaying = playbackState === MediaPlayer.PlayingState;
            if (root.isPlaying) {
                root.isLoading = false;
                root.errorMessage = "";
                root.hasStarted = true;
                playbackTimeout.stop();
                autoHideTimer.restart();
            } else {
                autoHideTimer.stop();
                root.controlsVisible = true;
                // A deliberate pause after playback began means the media is
                // ready; don't let the start-up timeout fire afterwards. A
                // paused state before playback ever started is still a failure.
                if (playbackState === MediaPlayer.PausedState && root.hasStarted)
                    playbackTimeout.stop();
            }
        }

        onMediaStatusChanged: {
            if (mediaStatus === MediaPlayer.LoadingMedia || mediaStatus === MediaPlayer.BufferingMedia || mediaStatus === MediaPlayer.StalledMedia) {
                root.isLoading = true;
            } else if (mediaStatus === MediaPlayer.LoadedMedia || mediaStatus === MediaPlayer.BufferedMedia) {
                // A candidate actually loaded, so any earlier candidate's error
                // is stale. Keep it until this point so silent retries stay
                // visible instead of blanking the UI. The start-up timeout keeps
                // running: loading is not the same as playing.
                root.isLoading = false;
                root.errorMessage = "";
            } else if (mediaStatus === MediaPlayer.InvalidMedia) {
                root.isLoading = false;
                if (root.errorMessage === "")
                    root.errorMessage = qsTr("Unsupported or invalid stream");
                root.controlsVisible = true;
                playbackTimeout.stop();
            }
        }

        onPositionChanged: root.positionSecs = position / 1000
        onDurationChanged: root.durationSecs = duration / 1000

        onErrorOccurred: (error, errorString) => {
            root.isLoading = false;
            root.errorMessage = errorString !== "" ? errorString : qsTr("Playback failed");
            // An error can arrive while the controls are auto-hidden mid-stream.
            root.controlsVisible = true;
            playbackTimeout.stop();
            if (root.player)
                root.player.onError(root.errorMessage);
        }
    }

    // Commands from the core (via PlayerAdapter) arrive as QmlPlayer signals.
    Connections {
        target: root.player
        function onPlayRequested() {
            media.play();
        }
        function onPauseRequested() {
            media.pause();
        }
        function onStopRequested() {
            media.stop();
        }
        function onSeekRequested(position) {
            media.position = position * 1000;
        }
    }

    // Keep the volume slider in sync with external volume changes without
    // fighting the user's drag.
    Binding {
        target: volumeSlider
        property: "value"
        value: root.volume * 100
        when: !volumeSlider.pressed
    }

    // ── Layout ───────────────────────────────────────────────────────
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
                    onClicked: root.goBack()
                }
            }

            // ── Channel name ─────────────────────────────────────────
            Text {
                Layout.fillWidth: true
                text: qsTr("Now Playing: %1").arg(root.channelName)
                color: Theme.textPrimary
                font.pixelSize: Theme.fontSizeXl
                font.weight: Font.Bold
                elide: Text.ElideRight
            }

            // ── Video area ───────────────────────────────────────────
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                color: Theme.surface
                radius: Theme.radiusMd
                clip: true

                VideoOutput {
                    id: videoOut
                    anchors.fill: parent
                    // VideoOutput defaults to PreserveAspectFit, which letterboxes
                    // the stream and exposes the container background along the
                    // edges. Cover the container instead so the viewport and the
                    // container always match.
                    fillMode: VideoOutput.PreserveAspectCrop
                }

                // Moving the cursor over the video re-shows the control bar
                // after it auto-hides.
                MouseArea {
                    anchors.fill: parent
                    hoverEnabled: true
                    onPositionChanged: root.revealControls()
                    onEntered: root.revealControls()
                    onClicked: root.revealControls()
                }

                // ── Control bar ──────────────────────────────────────
                Rectangle {
                    id: controlBar
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    height: 90
                    color: Theme.controlsBg
                    opacity: 0.92
                    visible: root.showControls

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Theme.spacingSm
                        spacing: Theme.spacingSm

                        // Timeline (seek bar)
                        Item {
                            id: timeline
                            Layout.fillWidth: true
                            Layout.preferredHeight: 4

                            Rectangle {
                                anchors.fill: parent
                                radius: 2
                                color: Theme.progressBg
                            }

                            Rectangle {
                                width: root.hasDuration ? parent.width * Math.min(1, root.positionSecs / root.durationSecs) : 0
                                height: parent.height
                                radius: 2
                                color: Theme.progressFill
                            }

                            MouseArea {
                                anchors.fill: parent
                                enabled: root.hasDuration
                                cursorShape: Qt.PointingHandCursor

                                function seekTo(x) {
                                    const ratio = Math.max(0, Math.min(1, x / width));
                                    media.position = ratio * root.durationSecs * 1000;
                                    root.revealControls();
                                }

                                onPressed: mouse => seekTo(mouse.x)
                                onPositionChanged: mouse => {
                                    if (pressed)
                                        seekTo(mouse.x);
                                }
                            }
                        }

                        // Time labels (right-aligned)
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: Theme.spacingXs

                            Item {
                                Layout.fillWidth: true
                            }

                            Text {
                                text: root.formatTime(root.positionSecs)
                                color: Theme.textPrimary
                                font.pixelSize: Theme.fontSizeSm
                            }

                            Text {
                                text: "/ " + root.formatTime(root.durationSecs)
                                color: Theme.textSecondary
                                font.pixelSize: Theme.fontSizeSm
                            }
                        }

                        // Transport + volume row
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: Theme.spacingMd

                            IconButton {
                                iconPath: Icons.skipBack
                                onClicked: root.seekBy(-root.skipSecs)
                            }

                            IconButton {
                                iconPath: root.isPlaying ? Icons.pause : Icons.play
                                baseColor: root.isPlaying ? Theme.primary : Theme.accent
                                hoverColor: root.isPlaying ? Theme.primaryHover : Theme.accentHover
                                pressColor: root.isPlaying ? Theme.primaryHover : Theme.accentHover
                                cornerRadius: Theme.controlsRadius
                                onClicked: root.togglePlayPause()
                            }

                            IconButton {
                                iconPath: Icons.skipForward
                                onClicked: root.seekBy(root.skipSecs)
                            }

                            Item {
                                Layout.fillWidth: true
                            }

                            Slider {
                                id: volumeSlider
                                Layout.preferredWidth: 120
                                Layout.preferredHeight: Theme.controlIconBox
                                from: 0
                                to: 100
                                onMoved: root.volume = value / 100

                                background: Rectangle {
                                    x: volumeSlider.leftPadding
                                    y: volumeSlider.topPadding + volumeSlider.availableHeight / 2 - height / 2
                                    width: volumeSlider.availableWidth
                                    height: 4
                                    radius: 2
                                    color: Theme.progressBg

                                    Rectangle {
                                        width: volumeSlider.visualPosition * parent.width
                                        height: parent.height
                                        radius: 2
                                        color: Theme.primary
                                    }
                                }

                                handle: Rectangle {
                                    x: volumeSlider.leftPadding + volumeSlider.visualPosition * (volumeSlider.availableWidth - width)
                                    y: volumeSlider.topPadding + volumeSlider.availableHeight / 2 - height / 2
                                    width: 12
                                    height: 12
                                    radius: 6
                                    color: Theme.textPrimary
                                }
                            }

                            IconButton {
                                iconPath: root.isMuted ? Icons.volumeOff : Icons.volumeOn
                                onClicked: {
                                    root.isMuted = !root.isMuted;
                                    root.revealControls();
                                }
                            }

                            IconButton {
                                iconPath: root.isFullscreen ? Icons.fullscreenExit : Icons.fullscreenEnter
                                onClicked: root.toggleFullscreen()
                            }
                        }
                    }
                }

                // ── Loading overlay ──────────────────────────────────
                BusyIndicator {
                    anchors.centerIn: parent
                    running: root.isLoading
                    visible: root.isLoading
                }

                // ── Error overlay ────────────────────────────────────
                Text {
                    anchors.centerIn: parent
                    width: parent.width - 2 * Theme.spacingXl
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.WordWrap
                    text: root.errorMessage
                    color: Theme.accent
                    font.pixelSize: Theme.fontSizeLg
                    visible: root.errorMessage !== ""
                }
            }
        }
    }
}
