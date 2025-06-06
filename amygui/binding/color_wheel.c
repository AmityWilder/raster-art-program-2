#include "amygui.h"
#include "raylib.h"
#include "raymath.h"
#include "rlgl.h"

// (copied from raygui)
// Convert color data from HSV to RGB
// NOTE: Color data should be passed normalized
static Vector3 ConvertHSVtoRGB(Vector3 hsv)
{
    Vector3 rgb = { 0 };
    float hh = 0.0f, p = 0.0f, q = 0.0f, t = 0.0f, ff = 0.0f;
    long i = 0;

    // NOTE: Comparing float values could not work properly
    if (hsv.y <= 0.0f)
    {
        rgb.x = hsv.z;
        rgb.y = hsv.z;
        rgb.z = hsv.z;
        return rgb;
    }

    hh = hsv.x;
    if (hh >= 360.0f) hh = 0.0f;
    hh /= 60.0f;

    i = (long)hh;
    ff = hh - i;
    p = hsv.z*(1.0f - hsv.y);
    q = hsv.z*(1.0f - (hsv.y*ff));
    t = hsv.z*(1.0f - (hsv.y*(1.0f - ff)));

    switch (i)
    {
        case 0:
        {
            rgb.x = hsv.z;
            rgb.y = t;
            rgb.z = p;
        } break;
        case 1:
        {
            rgb.x = q;
            rgb.y = hsv.z;
            rgb.z = p;
        } break;
        case 2:
        {
            rgb.x = p;
            rgb.y = hsv.z;
            rgb.z = t;
        } break;
        case 3:
        {
            rgb.x = p;
            rgb.y = q;
            rgb.z = hsv.z;
        } break;
        case 4:
        {
            rgb.x = t;
            rgb.y = p;
            rgb.z = hsv.z;
        } break;
        case 5:
        default:
        {
            rgb.x = hsv.z;
            rgb.y = p;
            rgb.z = q;
        } break;
    }

    return rgb;
}

// Draw HSV color picker wheel, returns updated color in HSV
Vector3 GuiColorPickerHSVWheel(Rectangle bounds, float triangleInnerSep, float previewRadius, float wheelThick, int wheelSegments, Vector3 hsv)
{
    const Color selectingColor = BLUE;
    const Color idleColor = GRAY;

    Vector2 mousePos = GetMousePosition();
    bool isMouseDown = IsMouseButtonDown(MOUSE_BUTTON_LEFT);

    float halfWidth = bounds.width/2;
    float halfHeight = bounds.height/2;
    Vector2 center = { bounds.x + halfWidth, bounds.y + halfHeight };
    float outerRadius = (halfWidth < halfHeight)? halfWidth : halfHeight;
    float innerRadius = outerRadius - wheelThick;
    float triangleRadius = innerRadius - triangleInnerSep;

    bool isInPicker = CheckCollisionPointRec(mousePos, bounds);
    bool isInHueWheel = isInPicker && (Vector2DistanceSqr(mousePos, center) >= innerRadius*innerRadius);
    bool isInTriangle = isInPicker && !isInHueWheel;

    float hue = hsv.x;
    float sat = hsv.y;
    float val = hsv.z;

    if (isMouseDown && isInHueWheel)
    {
        hue = RAD2DEG*atan2f((mousePos.y - center.y), (mousePos.x - center.x));
        if (hue < 0) hue += 360.0f;
    }

    Vector2 colorPos = { center.x + cosf(DEG2RAD* hue          )*triangleRadius, center.y + sinf(DEG2RAD* hue          )*triangleRadius };
    Vector2 whitePos = { center.x + cosf(DEG2RAD*(hue + 120.0f))*triangleRadius, center.y + sinf(DEG2RAD*(hue + 120.0f))*triangleRadius };
    Vector2 blackPos = { center.x + cosf(DEG2RAD*(hue + 240.0f))*triangleRadius, center.y + sinf(DEG2RAD*(hue + 240.0f))*triangleRadius };


    if (isMouseDown && isInTriangle)
    {
        Vector2 mouseRel = Vector2Subtract(mousePos, blackPos);

        Vector2 colorVec = Vector2Normalize(Vector2Subtract(colorPos, blackPos));
        Vector2 whiteVec = Vector2Normalize(Vector2Subtract(whitePos, blackPos));
        float tColor = Vector2DotProduct(colorVec, mouseRel);
        float tWhite = Vector2DotProduct(whiteVec, mouseRel);

        if ((tColor < 0) && (tWhite < 0)) val = sat = 0;
        else
        {
            // clamped mouse position
            Vector2 cmousePos = mousePos;

            if (tColor >= 0)
            {
                Vector2 onColorVec = Vector2Add(blackPos, Vector2Scale(colorVec, tColor));
                if (Vector2DistanceSqr(whitePos, onColorVec) < Vector2DistanceSqr(whitePos, mousePos)) cmousePos = onColorVec;
            }

            if (tWhite >= 0)
            {
                Vector2 onWhiteVec = Vector2Add(blackPos, Vector2Scale(whiteVec, tWhite));
                if (Vector2DistanceSqr(colorPos, onWhiteVec) < Vector2DistanceSqr(colorPos, mousePos)) cmousePos = onWhiteVec;
            }

            // linear intersection
            float u = Clamp(
                -((blackPos.x - cmousePos.x)*(blackPos.y - whitePos.y) - (blackPos.y - cmousePos.y)*(blackPos.x - whitePos.x))/
                 ((blackPos.x - cmousePos.x)*(whitePos.y - colorPos.y) - (blackPos.y - cmousePos.y)*(whitePos.x - colorPos.x)),
                0.0f, 1.0f);

            Vector2 intersection = Vector2Lerp(whitePos, colorPos, u);

            // quotient property of square roots: sqrt(x/y) = sqrt(x)/sqrt(y)
            val = Clamp(sqrtf(Vector2DistanceSqr(   cmousePos, blackPos)/Vector2DistanceSqr(intersection, blackPos)), 0.0f, 1.0f);
            sat = Clamp(sqrtf(Vector2DistanceSqr(intersection, whitePos)/Vector2DistanceSqr(    colorPos, whitePos)), 0.0f, 1.0f);
        }
    }

    Vector2 samplePos = Vector2Lerp(blackPos, Vector2Lerp(whitePos, colorPos, sat), val);
    hsv.x = hue;
    hsv.y = sat;
    hsv.z = val;

    // hue line

    float hueLineThick = 4.0f;
    Vector2 start = {
        center.x + cosf(DEG2RAD*hue)*(innerRadius - hueLineThick),
        center.y + sinf(DEG2RAD*hue)*(innerRadius - hueLineThick),
    };
    Vector2 end = {
        center.x + cosf(DEG2RAD*hue)*(outerRadius + hueLineThick),
        center.y + sinf(DEG2RAD*hue)*(outerRadius + hueLineThick),
    };
    DrawLineEx(start, end, 4.0f, (isMouseDown && isInHueWheel)? selectingColor : idleColor);

    Texture2D texShapes = GetShapesTexture();
    Rectangle shapeRect = GetShapesTextureRectangle();

    // circle

    rlSetTexture(texShapes.id);
    rlBegin(RL_QUADS);
        Vector3 colorA = { 1.0f, 0.0f, 0.0f };
        Vector2 outerA = { outerRadius + center.x, center.y };
        Vector2 innerA = { innerRadius + center.x, center.y };
        float stepSize = 360.0f/wheelSegments;

        for (int i = 1; i <= wheelSegments; i++)
        {
            float wheelHue = i*stepSize;
            Vector3 colorB = ConvertHSVtoRGB(CLITERAL(Vector3) { wheelHue, 1.0f, 1.0f });
            Vector2 outerB = { cosf(DEG2RAD*wheelHue)*outerRadius + center.x, sinf(DEG2RAD*wheelHue)*outerRadius + center.y };
            Vector2 innerB = { cosf(DEG2RAD*wheelHue)*innerRadius + center.x, sinf(DEG2RAD*wheelHue)*innerRadius + center.y };

            rlColor3f(colorA.x, colorA.y, colorA.z);
            rlTexCoord2f(shapeRect.x/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
            rlVertex2f(outerA.x, outerA.y);

            rlTexCoord2f(shapeRect.x/texShapes.width, shapeRect.y/texShapes.height);
            rlVertex2f(innerA.x, innerA.y);

            rlColor3f(colorB.x, colorB.y, colorB.z);
            rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, shapeRect.y/texShapes.height);
            rlVertex2f(innerB.x, innerB.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
            rlVertex2f(outerB.x, outerB.y);

            colorA = colorB;
            outerA = outerB;
            innerA = innerB;
        }

    rlEnd();

    // triangle

    Vector3 fullSaturation = ConvertHSVtoRGB(CLITERAL(Vector3) { hue, 1.0f, 1.0f });

    rlSetTexture(texShapes.id);
    rlBegin(RL_TRIANGLES);
        rlColor3f(fullSaturation.x, fullSaturation.y, fullSaturation.z);
        rlTexCoord2f(shapeRect.x/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(colorPos.x, colorPos.y);

        rlColor3f(0.0f, 0.0f, 0.0f);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(blackPos.x, blackPos.y);

        rlColor3f(1.0f, 1.0f, 1.0f);
        rlTexCoord2f(shapeRect.x/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(whitePos.x, whitePos.y);
    rlEnd();
    rlSetTexture(0);

    // preview

    Vector3 rgb = ConvertHSVtoRGB(hsv);
    Color outlineColor = (isMouseDown && isInTriangle)? selectingColor : idleColor;
    float left   = samplePos.x - previewRadius;
    float right  = samplePos.x + previewRadius;
    float top    = samplePos.y - previewRadius;
    float bottom = samplePos.y + previewRadius;

    rlSetTexture(texShapes.id);
    rlBegin(RL_QUADS);
        // outline
        rlNormal3f(0.0f, 0.0f, 1.0f);
        rlColor4ub(outlineColor.r, outlineColor.g, outlineColor.b, 255);

        rlTexCoord2f(shapeRect.x/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(left - 1, top - 1);

        rlTexCoord2f(shapeRect.x/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(left - 1, bottom + 1);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(right + 1, bottom + 1);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(right + 1, top - 1);

        // preview
        rlColor3f(rgb.x, rgb.y, rgb.z);

        rlTexCoord2f(shapeRect.x/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(left, top);

        rlTexCoord2f(shapeRect.x/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(left, bottom);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(right, bottom);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(right, top);
    rlEnd();
    rlSetTexture(0);

    return hsv;
}
