import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: root

    property string backendName: "Backend"
    property string description: ""
    property string statusText: "Runtime check pending"
    property bool selected: false
    property bool detected: false

    signal selectedBackend()

    implicitWidth: 320
    implicitHeight: 190
    radius: 16
    color: selected ? "#202b45" : "#171d2e"
    border.width: selected ? 2 : 1
    border.color: selected ? "#55a7ff" : "#303a52"

    Behavior on border.color {
        ColorAnimation { duration: 140 }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 20
        spacing: 12

        RowLayout {
            Layout.fillWidth: true

            Label {
                text: root.backendName
                color: "#f4f7ff"
                font.pixelSize: 19
                font.bold: true
                Layout.fillWidth: true
            }

            Rectangle {
                radius: 8
                color: root.detected ? "#183d35" : "#343044"
                implicitWidth: statusLabel.implicitWidth + 16
                implicitHeight: 26

                Label {
                    id: statusLabel
                    anchors.centerIn: parent
                    text: root.detected ? "Runtime found" : "Not detected"
                    color: root.detected ? "#8de0c2" : "#c7bddf"
                    font.pixelSize: 11
                }
            }
        }

        Label {
            Layout.fillWidth: true
            Layout.fillHeight: true
            text: root.description
            color: "#aab5cc"
            wrapMode: Text.WordWrap
            font.pixelSize: 13
        }

        Label {
            Layout.fillWidth: true
            text: root.statusText
            color: "#8f9bb4"
            font.pixelSize: 11
            wrapMode: Text.WordWrap
        }

        Button {
            text: root.selected ? "Selected" : "Select backend"
            Layout.fillWidth: true
            onClicked: root.selectedBackend()
        }
    }
}
