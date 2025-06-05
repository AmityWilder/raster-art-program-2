#include "amygui.h"
#include "raylib.h"
#include "raymath.h"
#include "rlgl.h"

Vector3 GuiColorPickerHSVWheel(Rectangle bounds, Vector3 colorHSV)
{
    Vector3 result = { 0 };

    const Vector2 mousePos = GetMousePosition();
    const bool isMouseDown = IsMouseButtonDown(MOUSE_BUTTON_LEFT);

    const float thick = 20;
    const float innerSpace = 7;
    const float halfWidth = bounds.width/2;
    const float halfHeight = bounds.height/2;
    const Vector2 center = { bounds.x + halfWidth, bounds.y + halfHeight };
    const float outerRadius = (halfWidth < halfHeight) ? halfWidth : halfHeight;
    const float innerRadius = outerRadius - thick;
    const float triangleRadius = innerRadius - innerSpace;

    const bool isInHueWheel = Vector2DistanceSqr(mousePos, center) >= innerRadius*innerRadius;

    float hue;
    if (isMouseDown && isInHueWheel)
    {
        hue = atan2f((mousePos.y - center.y), (mousePos.x - center.x));
        if (hue < 0) hue += 2*PI;
    }
    else
    {
        hue = DEG2RAD*colorHSV.x;
    }

    const Vector2 colorPos = { center.x + cosf( hue          )*triangleRadius, center.y + sinf( hue          )*triangleRadius };
    const Vector2 whitePos = { center.x + cosf((hue + 2*PI/3))*triangleRadius, center.y + sinf((hue + 2*PI/3))*triangleRadius };
    const Vector2 blackPos = { center.x + cosf((hue + 4*PI/3))*triangleRadius, center.y + sinf((hue + 4*PI/3))*triangleRadius };

    float val = colorHSV.z;
    float sat = colorHSV.y;
    const Vector2 s = mousePos;
    const Vector2 p0 = blackPos;
    const Vector2 p1 = whitePos;
    const Vector2 p2 = colorPos;

    if (isMouseDown && !isInHueWheel)
    {
        // linear intersection
        float u =
            ((p0.x - s.x)*(p0.y - p1.y) - (p0.y - s.y)*(p0.x - p1.x))/
            ((p0.x - s.x)*(p1.y - p2.y) - (p0.y - s.y)*(p1.x - p2.x));

        u = Clamp(-u, 0.0f, 1.0f);
        Vector2 p = Vector2Lerp(p1, p2, u);

        // quotient property of square roots: sqrt(x/y) = sqrt(x)/sqrt(y)
        float a = Clamp(sqrtf(Vector2DistanceSqr(s, p0)/Vector2DistanceSqr(p,  p0)), 0.0f, 1.0f);
        float b = Clamp(sqrtf(Vector2DistanceSqr(p, p1)/Vector2DistanceSqr(p2, p1)), 0.0f, 1.0f);
        val = a;
        sat = b;
    }
    result.x = hue*RAD2DEG;
    result.y = sat;
    result.z = val;

    const Color colorMax = ColorFromHSV(RAD2DEG*hue, 1.0f, 1.0f);

    // hue line

    const float hueLineThick = 4.0;
    const Vector2 start = {
        center.x + cosf(hue)*(innerRadius - hueLineThick),
        center.y + sinf(hue)*(innerRadius - hueLineThick),
    };
    const Vector2 end = {
        center.x + cosf(hue)*(outerRadius + hueLineThick),
        center.y + sinf(hue)*(outerRadius + hueLineThick),
    };
    DrawLineEx(start, end, 4.0, BLUE);

    const Texture2D texShapes = GetShapesTexture();
    const Rectangle shapeRect = GetShapesTextureRectangle();

    // circle

    rlSetTexture(texShapes.id);
    rlBegin(RL_QUADS);
    Color colorA = { 255, 0, 0, 255 };
    Vector2 outerA = { outerRadius + center.x, center.y };
    Vector2 innerA = { innerRadius + center.x, center.y };
    for (float t = 1; t <= 360; ++t)
    {
        const Color colorB = ColorFromHSV(t, 1.0f, 1.0f);
        const Vector2 outerB = { cosf(DEG2RAD*t)*outerRadius + center.x, sinf(DEG2RAD*t)*outerRadius + center.y };
        const Vector2 innerB = { cosf(DEG2RAD*t)*innerRadius + center.x, sinf(DEG2RAD*t)*innerRadius + center.y };

        rlColor4ub(colorA.r, colorA.g, colorA.b, 255);
        rlTexCoord2f(shapeRect.x/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(outerA.x, outerA.y);

        rlTexCoord2f(shapeRect.x/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(innerA.x, innerA.y);

        rlColor4ub(colorB.r, colorB.g, colorB.b, 255);
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

    rlSetTexture(texShapes.id);
    rlBegin(RL_TRIANGLES);
        rlColor4ub(colorMax.r, colorMax.g, colorMax.b, 255);
        rlTexCoord2f(shapeRect.x/texShapes.width, (shapeRect.y + shapeRect.height)/texShapes.height);
        rlVertex2f(colorPos.x, colorPos.y);

        rlColor4ub(0, 0, 0, 255);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(blackPos.x, blackPos.y);

        rlColor4ub(255, 255, 255, 255);
        rlTexCoord2f(shapeRect.x/texShapes.width, shapeRect.y/texShapes.height);
        rlVertex2f(whitePos.x, whitePos.y);
    rlEnd();
    rlSetTexture(0);

    // sample
    const float a = val;
    const float b = sat;
    {
        const float sampleRadius = 3;
        const Vector2 samplePos = Vector2Lerp(p0, Vector2Lerp(p1, p2, b), a);
        const Rectangle sampleRect = { samplePos.x - sampleRadius, samplePos.y - sampleRadius, sampleRadius*2, sampleRadius*2 };
        const Rectangle outlineRect = {
            sampleRect.x - 1,
            sampleRect.y - 1,
            sampleRect.width  + 2,
            sampleRect.height + 2,
        };
        DrawRectangleRec(outlineRect, BLUE);
        DrawRectangleRec(sampleRect, ColorFromHSV(RAD2DEG*hue, sat, val));
    }

    return result;
}
