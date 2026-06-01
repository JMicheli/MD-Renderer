#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10
#define GAMMA_FACTOR 2.2
#define AMBIENT_FACTOR 0.05

#define PI 3.14159265359

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

// Shader Entry Point
// //////////////////
void main() {
  vec3 diffuse_color = texture(diffuse_map, v_uv).xyz;
  float roughness = texture(roughness_map, v_uv).x;
  float specular_strength = max(material.shininess * (1.0 - roughness), 0.0001);

  // Surface normal from normal map and TBN
  vec3 N = texture(normal_map, v_uv).xyz;
  N = N * 2.0 - 1.0;
  N = normalize(v_TBN * N);
  // View to fragment location
  vec3 V = normalize(scene_data.camera.position - v_position);

  // Ambient lighting
  vec3 ambient = diffuse_color * AMBIENT_FACTOR; 

  // Loop over all the scene lights, accumulating the result
  vec3 result = ambient;
  for (uint i = 0; i < scene_data.point_light_count; i++) {
    // Calculate per-light variables
    vec3 light_position = scene_data.point_lights[i].position;
    // Use distance from light to calculate light_color with fall-off
    float light_dist = length(light_position - v_position);
    float attenuation = 1.0 / max(light_dist * light_dist, 0.001);
    vec3 light_color = scene_data.point_lights[i].color * scene_data.point_lights[i].brightness * attenuation;
    // Light to fragment location
    vec3 L = normalize(light_position - v_position);

    // Diffuse
    float diffuse_intensity = max(dot(N, L), 0.0);
    vec3 diffuse = light_color * diffuse_intensity * diffuse_color;
    
    // Specular
    vec3 H = normalize(L + V); 
    float spec = pow(max(dot(N, H), 0.0), specular_strength);
    float energy_conservation = (specular_strength + 8.0) / (8.0 * PI);
    vec3 specular = light_color * spec * material.specular_color * energy_conservation;
    
    result += diffuse + specular;
  } 
  
  result = pow(result, vec3(1.0 / GAMMA_FACTOR));
  f_color = vec4(result, 1.0);
}
