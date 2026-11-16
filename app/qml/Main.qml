import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window

ApplicationWindow {
    id: window

    width: 1120
    height: 760
    minimumWidth: 860
    minimumHeight: 620
    visible: true
    title: "TomOS Bedrock Launcher"
    color: "#0e1320"

    property string selectedBackend: "windows"
    property string currentPage: "home"

    palette.window: "#0e1320"
    palette.windowText: "#f4f7ff"
    palette.button: "#26334d"
    palette.buttonText: "#f4f7ff"
    palette.highlight: "#3989e8"
    palette.highlightedText: "#ffffff"

    RowLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.preferredWidth: 224
            Layout.fillHeight: true
            color: "#111827"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 20
                spacing: 18

                RowLayout {
                    spacing: 10

                    Rectangle {
                        width: 38
                        height: 38
                        radius: 11
                        color: "#2477d4"

                        Label {
                            anchors.centerIn: parent
                            text: "T"
                            color: "white"
                            font.pixelSize: 22
                            font.bold: true
                        }
                    }

                    ColumnLayout {
                        spacing: 1
                        Label {
                            text: "TomOS"
                            color: "#f4f7ff"
                            font.pixelSize: 17
                            font.bold: true
                        }
                        Label {
                            text: "BEDROCK LAUNCHER"
                            color: "#8290aa"
                            font.pixelSize: 9
                            font.letterSpacing: 1.1
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    height: 1
                    color: "#273249"
                }

                Repeater {
                    model: [
                        { label: "Home", page: "home" },
                        { label: "Game versions", page: "versions" },
                        { label: "Add-ons", page: "addons" },
                        { label: "Worlds", page: "worlds" },
                        { label: "Controllers", page: "controllers" },
                        { label: "Multiplayer", page: "multiplayer" }
                    ]

                    delegate: Button {
                        required property var modelData
                        Layout.fillWidth: true
                        text: modelData.label
                        highlighted: window.currentPage === modelData.page
                        onClicked: window.currentPage = modelData.page
                    }
                }

                Item { Layout.fillHeight: true }

                Label {
                    text: "EARLY DEVELOPMENT BUILD"
                    color: "#72809a"
                    font.pixelSize: 9
                    font.letterSpacing: 1
                }
                Label {
                    text: "Game files are never bundled."
                    color: "#8794aa"
                    font.pixelSize: 11
                    wrapMode: Text.WordWrap
                    Layout.fillWidth: true
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: "#0e1320"

            ScrollView {
                anchors.fill: parent
                clip: true

                ColumnLayout {
                    width: parent.width
                    spacing: 22

                    RowLayout {
                        Layout.fillWidth: true
                        Layout.leftMargin: 34
                        Layout.rightMargin: 34
                        Layout.topMargin: 30

                        ColumnLayout {
                            spacing: 4
                            Label {
                                text: window.currentPage === "home" ? "Your game library" : "Coming soon"
                                color: "#f4f7ff"
                                font.pixelSize: 28
                                font.bold: true
                            }
                            Label {
                                text: window.currentPage === "home"
                                    ? "Choose how you want to run Minecraft Bedrock on Linux."
                                    : "This section will be enabled as its backend features are implemented."
                                color: "#9aa8c0"
                                font.pixelSize: 13
                            }
                        }

                        Item { Layout.fillWidth: true }

                        Rectangle {
                            radius: 10
                            color: "#182338"
                            border.color: "#2b3b58"
                            implicitWidth: hostLabel.implicitWidth + 24
                            implicitHeight: 36

                            Label {
                                id: hostLabel
                                anchors.centerIn: parent
                                text: hostDistribution + " · " + hostArchitecture + " · " + hostSession
                                color: "#b8c7df"
                                font.pixelSize: 11
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 34
                        Layout.rightMargin: 34
                        implicitHeight: 120
                        radius: 18
                        color: "#17243a"
                        border.color: "#2b456b"

                        RowLayout {
                            anchors.fill: parent
                            anchors.margins: 22
                            spacing: 18

                            Rectangle {
                                width: 54
                                height: 54
                                radius: 15
                                color: "#214f87"
                                Label {
                                    anchors.centerIn: parent
                                    text: "◆"
                                    color: "#9bcaff"
                                    font.pixelSize: 25
                                }
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 5
                                Label {
                                    text: "One launcher. Two Bedrock routes."
                                    color: "#f4f7ff"
                                    font.pixelSize: 18
                                    font.bold: true
                                }
                                Label {
                                    text: "Live host details and runtime executable checks are supplied by the Rust service."
                                    color: "#a9b8d0"
                                    wrapMode: Text.WordWrap
                                    Layout.fillWidth: true
                                    font.pixelSize: 12
                                }
                            }
                        }
                    }

                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.leftMargin: 34
                        Layout.rightMargin: 34
                        spacing: 12

                        Label {
                            text: "Select a backend"
                            color: "#f4f7ff"
                            font.pixelSize: 19
                            font.bold: true
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 16

                            BackendCard {
                                Layout.fillWidth: true
                                backendName: "Windows Bedrock"
                                description: "Run the Windows edition through a compatible Linux runtime. Requires a separately installed, user-authorised setup."
                                statusText: windowsStatus
                                selected: window.selectedBackend === "windows"
                                detected: windowsDetected
                                onSelectedBackend: window.selectedBackend = "windows"
                            }

                            BackendCard {
                                Layout.fillWidth: true
                                backendName: "Android Bedrock"
                                description: "Run the Android edition through the community MCPE Launcher runtime. Compatibility varies by game version and graphics stack."
                                statusText: androidStatus
                                selected: window.selectedBackend === "android"
                                detected: androidDetected
                                onSelectedBackend: window.selectedBackend = "android"
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 34
                        Layout.rightMargin: 34
                        implicitHeight: 132
                        radius: 16
                        color: "#151d2d"
                        border.color: "#29354b"

                        RowLayout {
                            anchors.fill: parent
                            anchors.margins: 20
                            spacing: 18

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 5
                                Label {
                                    text: "Ready to play?"
                                    color: "#f4f7ff"
                                    font.pixelSize: 18
                                    font.bold: true
                                }
                                Label {
                                    text: "Runtime presence is not proof of game compatibility. Profile checks and a safe launch workflow are still required."
                                    color: "#9aa8c0"
                                    wrapMode: Text.WordWrap
                                    Layout.fillWidth: true
                                    font.pixelSize: 12
                                }
                            }

                            Button {
                                text: "Play"
                                enabled: false
                                Layout.preferredWidth: 140
                                Layout.preferredHeight: 46
                            }
                        }
                    }

                    Label {
                        Layout.leftMargin: 34
                        Layout.rightMargin: 34
                        Layout.bottomMargin: 26
                        Layout.fillWidth: true
                        text: "Read-only host detection · No game files, accounts or runtime changes are made by this screen."
                        color: "#697790"
                        font.pixelSize: 11
                        horizontalAlignment: Text.AlignHCenter
                    }
                }
            }
        }
    }
}
