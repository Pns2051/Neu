import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ItemDelegate {
    id: control
    property string text: ""
    property bool active: false
    
    width: parent.width
    height: 48
    
    contentItem: Label {
        text: control.text
        color: control.active ? "white" : "#B3B3B3"
        font.bold: control.active
        verticalAlignment: Text.AlignVCenter
        leftPadding: 16
    }
    
    background: Rectangle {
        color: control.active ? "#33FFFFFF" : (control.hovered ? "#11FFFFFF" : "transparent")
        radius: 8
        anchors.margins: 4
    }
}
