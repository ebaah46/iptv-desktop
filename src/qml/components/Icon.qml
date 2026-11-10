import QtQuick
import QtQuick.Shapes

/// Renders a single-colour icon from SVG path data (24x24 viewBox).
/// Vector-based, so it stays crisp at any size and needs no image assets.
Item {
    id: root

    property string path: ""
    property color color: "#ffffff"

    implicitWidth: 24
    implicitHeight: 24

    Shape {
        width: 24
        height: 24
        scale: root.width / 24
        transformOrigin: Item.TopLeft
        preferredRendererType: Shape.CurveRenderer

        ShapePath {
            fillColor: root.color
            strokeColor: "transparent"
            PathSvg {
                path: root.path
            }
        }
    }
}
