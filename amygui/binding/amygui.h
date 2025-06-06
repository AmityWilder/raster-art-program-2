#include "raylib.h"

// Draw HSV color picker wheel, returns updated color in HSV
Vector3 GuiColorPickerHSVWheel(
    Vector2 center,
    float previewRadius,
    float triangleRadius,
    float wheelInnerRadius,
    float wheelOuterRadius,
    int wheelSegments,
    Vector3 hsv
);
