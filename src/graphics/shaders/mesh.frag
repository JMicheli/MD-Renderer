#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10
#define GAMMA_FACTOR 2.2

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in mat3 v_TBN;


layout(location = 0) out vec4 f_color;

// Input buffer objects
// ////////////////////

// Data representing a camera in the scene
struct CameraData {
  // Camera's position in world space
  vec3 position;
  // View transformation matrix
  mat4 view;
  // Perspective projection matrix
  mat4 proj;
};

// Data representing a point light
struct PointLightData {
  // The RGB color of the light
  vec3 color;
  // The position of the light in world space
  vec3 position;
  // The brightness factor of the light
  float brightness;
};

// Data representing the scene
layout(set = 0, binding = 0) buffer MdrSceneData {
  // The camera being used to render the scene
  CameraData camera;
  // Up to MAX_POINT_LIGHTS point light values
  PointLightData point_lights[MAX_POINT_LIGHTS];
  // Maximum point_light index with a valid value
  uint point_light_count;
} scene_data;

// Data representing a material
layout(set = 1, binding = 0) uniform MdrMaterialUniformData {
  // The color of an object's specular highlight
  vec3 specular_color;
  // The exponential specular factor for Blinn-Phong 
  float shininess;
} material;

// Material texture maps
// Base color of material
layout(set = 1, binding = 1) uniform sampler2D diffuse_map;
// Roughness map for material
layout(set = 1, binding = 2) uniform sampler2D roughness_map;
// Normal map for material
layout(set = 1, binding = 3) uniform sampler2D normal_map;

///////////////////////
//TODO Remove test code
///////////////////////
const float ambient_strength = 0.1;

// Lighting functions
// //////////////////
vec3 calculate_point_light_contribution(PointLightData light, vec3 specular_strength, vec3 N, vec3 V);

// Shader Entry Point
// //////////////////
void main() {
  // Calculate normalized directional vectors for lighting
  // Surface normal from normal map and TBN
  vec3 N = texture(normal_map, v_uv).xyz;
  N = N * 2.0 - 1.0;
  N = normalize(v_TBN * N);
  // Direction to viewer
  vec3 V = normalize(scene_data.camera.position - v_position);

  // Sample diffuse map to get color
  vec4 diffuse_color = texture(diffuse_map, v_uv);
  // Sample specular map to get specular strength
  vec3 specular_strength = vec3(1.0) - texture(roughness_map, v_uv).xyx;

  vec3 result = vec3(0.0);
  for (int i = 0; i < scene_data.point_light_count; i++) {
    result += calculate_point_light_contribution(scene_data.point_lights[i], specular_strength, N, V) * diffuse_color.xyz;
  }

  // Perform gamma correction
  vec3 gamma_corrected_result = pow(result, vec3(1.0/GAMMA_FACTOR));

  f_color = vec4(gamma_corrected_result, diffuse_color.w);
}

// Impl lighting functions
// ///////////////////////

vec3 calculate_point_light_contribution(PointLightData light, vec3 specular_strength, vec3 N, vec3 V) {
  // Light-specific direction vectors
  // Direction to light
  vec3 L = normalize(light.position - v_position);
  // Blinn-Phong halfway vector
  vec3 H = normalize(L + V);

  // Light color adjusted by brightness
  vec3 light_color = light.color * light.brightness;

  // Blinn-Phong BRDF
  // Ambient contribution
  vec3 ambient = ambient_strength * light_color;
  
  // Diffuse contribution
  float diffusion_coefficient = max(dot(N, L), 0.0);
  vec3 diffuse = diffusion_coefficient * light_color;

  // Specular contribution
  float specular_coefficient = pow(max(dot(N, H), 0.0), material.shininess);
  vec3 specular = specular_strength * light_color * specular_coefficient ;

  return (ambient + diffuse + specular);
}