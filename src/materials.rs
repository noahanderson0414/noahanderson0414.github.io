pub const BLUR_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying vec4 color;
varying float radius;
varying vec2 texel_size;

uniform sampler2D _ScreenTexture;

void main() {
    vec4 total = vec4(0.0);

    for (int y = int(-radius); y <= int(radius); y++) {
        for (int x = int(-radius); x <= int(radius); x++) {
            total += texture2D(_ScreenTexture, uv_screen + vec2(x, y) * texel_size);
        }
    }

    total /= pow(1.0 + radius * 2.0, 2.0);
    gl_FragColor = vec4(mix(total.xyz, color.xyz, color.a), 1.0);
}
"#;

pub const BLUR_VERTEX_SHADER: &'static str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;
varying lowp vec2 uv_screen;
varying lowp vec4 color;
varying lowp float radius;
varying lowp vec2 texel_size;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec4 Color;
uniform float Radius;
uniform vec2 TexelSize;

void main() {
    vec4 result = Projection * Model * vec4(position, 1.0);

    uv_screen = result.xy / 2.0 + vec2(0.5);
    uv = texcoord;
    color = Color;
    radius = Radius;
    texel_size = TexelSize;

    gl_Position = result;
}
"#;

pub const GRADIENT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec2 uv;
varying vec4 start_color;
varying vec4 end_color;

void main() {
    gl_FragColor = mix(mix(start_color, end_color, uv.x), mix(start_color, end_color, uv.y), 0.5);
}
"#;

pub const GRADIENT_VERTEX_SHADER: &'static str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;
varying lowp vec4 start_color;
varying lowp vec4 end_color;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec4 StartColor;
uniform vec4 EndColor;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);

    uv = texcoord;
    start_color = StartColor;
    end_color = EndColor;
}
"#;
