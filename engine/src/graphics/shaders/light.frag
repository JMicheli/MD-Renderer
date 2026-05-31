#version 450

// Configuration
// /////////////
#define GAMMA_FACTOR 2.2

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 i_light_color;
layout(location = 1) in float i_light_brightness;

layout(location = 0) out vec4 f_color;

// Shader Entry Point
// //////////////////
void main() {
  
  // Perform gamma correction
  vec3 color = i_light_color;
  vec3 gamma_corrected_result = pow(color, vec3(1.0/GAMMA_FACTOR));

  f_color = vec4(gamma_corrected_result, 1.0);
}
