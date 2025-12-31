#version 300 es
precision highp float;

// Vibrance Shader for Hyprland - Enhances color vibrance while preserving skin tones
// Adjust these values to customize:
const float VIBRANCE = 0.75;              // -1.0 to 1.0 (positive = more vibrant)
const float SKIN_TONE_PROTECTION = 0.75; // 0.0 to 1.0 (higher = preserve skin tones)

in vec2 v_texcoord;
out vec4 fragColor;
uniform sampler2D tex;

float getLuminance(vec3 color) {
    return dot(color, vec3(0.2126, 0.7152, 0.0722));
}

float skinToneLikelihood(vec3 color) {
    float r = color.r;
    float g = color.g;
    float b = color.b;

    bool skinCondition1 = r > g && g > b;
    float skinCondition2 = 0.0;
    if (r >= 0.4 && r <= 0.85 && g >= 0.2 && g <= 0.7 && b >= 0.1 && b <= 0.5) {
        skinCondition2 = 1.0;
    }

    return float(skinCondition1) * skinCondition2;
}

float getSaturation(vec3 color) {
    float minVal = min(min(color.r, color.g), color.b);
    float maxVal = max(max(color.r, color.g), color.b);

    return (maxVal == 0.0) ? 0.0 : (maxVal - minVal) / maxVal;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = pixColor.rgb;

    if (VIBRANCE == 0.0) {
        fragColor = pixColor;
        return;
    }

    float saturation = getSaturation(color);
    float luma = getLuminance(color);
    float vibranceAmount = (1.0 - saturation) * abs(VIBRANCE);
    float skinProtection = skinToneLikelihood(color) * SKIN_TONE_PROTECTION;

    vec3 result;

    if (VIBRANCE > 0.0) {
        float adjustedVibrance = vibranceAmount * (1.0 - skinProtection);
        vec3 grayColor = vec3(luma);
        result = mix(grayColor, color, 1.0 + adjustedVibrance);
    } else {
        vec3 grayColor = vec3(luma);
        result = mix(color, grayColor, vibranceAmount);
    }

    fragColor = vec4(result, pixColor.a);
}
