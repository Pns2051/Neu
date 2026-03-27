import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Effects

Window {
    id: root
    width: 1000
    height: 700
    visible: true
    title: "Neu"
    color: "#121212"

    property string currentView: "search"
    property string nowPlayingTitle: "Neu Music Platform"
    property string nowPlayingArtist: "Select a track to start"
    property bool isPlaying: false
    property real playbackProgress: 0.0
    property real volume: 0.8

    RowLayout {
        anchors.fill: parent
        anchors.bottomMargin: 96 // Space for player bar
        spacing: 0

        // SIDEBAR
        Rectangle {
            Layout.fillHeight: true
            Layout.preferredWidth: 240
            color: "#000000"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 16
                spacing: 8

                RowLayout {
                    spacing: 12
                    Layout.bottomMargin: 24
                    Image {
                        source: "qrc:/assets/logo.png"
                        sourceSize: Qt.size(32, 32)
                    }
                    Label {
                        text: "Neu"
                        color: "white"
                        font.pixelSize: 20
                        font.bold: true
                    }
                }

                SidebarItem {
                    text: "Home"
                    active: root.currentView === "recommended"
                    onClicked: root.currentView = "recommended"
                }
                SidebarItem {
                    text: "Search"
                    active: root.currentView === "search"
                    onClicked: root.currentView = "search"
                }
                SidebarItem {
                    text: "Playlists"
                    active: root.currentView === "playlists"
                    onClicked: root.currentView = "playlists"
                }

                Label { text: "Login"; color: "#666666"; font.pixelSize: 12; Layout.topMargin: 16 }
                RowLayout {
                    spacing: 8
                    Button { text: "YT"; onClicked: auth.loginYouTube() }
                    Button { text: "SP"; onClicked: auth.loginSpotify() }
                }

                Item { Layout.fillHeight: true }
                
                Button { text: "Lyrics"; onClicked: lyricsPopup.open() }
                Button { text: "Settings"; onClicked: settingsPopup.open() }
            }
        }

        // MAIN CONTENT
        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: root.currentView === "search" ? 0 : (root.currentView === "recommended" ? 1 : 2)

            // Search View
            ColumnLayout {
                spacing: 16
                Layout.margins: 24

    property var searchResults: []

                RowLayout {
                    spacing: 12
                    TextField {
                        id: searchInput
                        Layout.fillWidth: true
                        placeholderText: "Search YouTube or Local..."
                        onAccepted: root.searchResults = plugins.search(text)
                    }
                    Button {
                        text: "Search"
                        onClicked: root.searchResults = plugins.search(searchInput.text)
                    }
                }

                Label {
                    text: searchResults.length > 0 ? "Search Results (" + searchResults.length + ")" : "Search Results"
                    color: "white"
                    font.pixelSize: 18
                    font.bold: true
                }

                ListView {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    model: root.searchResults
                    delegate: ItemDelegate {
                        width: parent.width
                        height: 56
                        onClicked: {
                            root.nowPlayingTitle = modelData.title
                            root.nowPlayingArtist = modelData.artist
                            root.isPlaying = true
                            playback.play(plugins.getPlugin(modelData.source)->stream(modelData.id))
                            discord.updatePresence(modelData.title, modelData.artist, modelData.duration)
                        }

                        RowLayout {
                            anchors.fill: parent
                            anchors.leftMargin: 16
                            anchors.rightMargin: 16
                            spacing: 12
                            
                            Column {
                                Layout.fillWidth: true
                                Label { text: modelData.title; color: "white" }
                                Label { text: modelData.artist + " • " + modelData.source; color: "#B3B3B3"; font.pixelSize: 12 }
                            }

                            Button {
                                text: "+"
                                flat: true
                                onClicked: {
                                    playlists.addTrackToPlaylist(1, modelData)
                                    console.log("Added to playlist: " + modelData.title)
                                }
                            }
                        }
                    }
                }
                }
            }

            // Recommended View
            Item {
                Label {
                    text: "Recommendations Coming Soon"
                    color: "white"
                    anchors.centerIn: parent
                }
            }

            // Playlists View
            Item {
                Label {
                    text: "Playlists View Coming Soon"
                    color: "white"
                    anchors.centerIn: parent
                }
            }
        }
    }

    // PLAYER BAR (Glassmorphism)
    Rectangle {
        anchors.bottom: parent.bottom
        width: parent.width
        height: 96
        color: "#AA181818" // Semi-transparent
        border.color: "#33282828"
        border.width: 1

        RowLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 24

            Column {
                Layout.preferredWidth: parent.width * 0.3
                Label { text: root.nowPlayingTitle; color: "white"; font.bold: true }
                Label { text: root.nowPlayingArtist; color: "#B3B3B3"; font.pixelSize: 12 }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignCenter
                
                RowLayout {
                    Layout.alignment: Qt.AlignCenter
                    spacing: 24
                    Button { text: "||"; onClicked: root.isPlaying = !root.isPlaying }
                }

                Slider {
                    Layout.fillWidth: true
                    value: root.playbackProgress
                }
            }

            RowLayout {
                Layout.preferredWidth: parent.width * 0.2
                Layout.alignment: Qt.AlignRight
                Label { text: "Vol"; color: "#B3B3B3" }
                Slider { 
                    value: root.volume 
                    onValueChanged: playback.setVolume(value)
                }
            }
        }
    }

    Popup {
        id: lyricsPopup
        anchors.centerIn: parent
        width: 600; height: 500
        background: Rectangle { color: "#CC121212"; radius: 12 }
        ScrollView {
            anchors.fill: parent
            Label {
                id: lyricsText
                padding: 32
                width: parent.width
                wrapMode: Text.WordWrap
                color: "white"
                font.pixelSize: 20
                text: "Lyrics will appear here..."
            }
        }
    }

    Popup {
        id: settingsPopup
        anchors.centerIn: parent
        width: 300
        height: 200
        modal: true
        focus: true
        ColumnLayout {
            anchors.fill: parent
            padding: 24
            CheckBox {
                text: "Automatic Update Checker"
                checked: true
                onCheckedChanged: updater.setUpdatesEnabled(checked)
            }
            Button { text: "Close"; onClicked: settingsPopup.close() }
        }
    }

    Connections {
        target: lyricsProvider
        function onLyricsReady(l) { lyricsText.text = l }
    }
    
    Connections {
        target: updater
        function onUpdateAvailable(version) { console.log("New version available: " + version) }
    }

    Connections {
        target: playback
        function onPositionChanged(pos) { root.playbackProgress = pos }
        function onDurationChanged(dur) { playbackSlider.to = dur }
        function onPlayingChanged(p) { root.isPlaying = p }
    }
}
