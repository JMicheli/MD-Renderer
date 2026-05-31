#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10

// Inputs/Ouputs
// /////////////

// Drawn vertices
layout(location = 0) in vec3 a_position;

// Per-instance data outputs (can I do this?)
layout(location = 0) out vec3 i_light_color;
layout(location = 1) out float i_light_brightness;

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

// Data representing the scene - only camera data is used here
layout(set = 0, binding = 0) buffer MdrSceneData {
  // The camera being used to render the scene
  CameraData camera;
  // Up to MAX_POINT_LIGHTS point light values
  PointLightData point_lights[MAX_POINT_LIGHTS];
  // Maximum point_light index with a valid value
  uint point_light_count;
} scene_data;

// Shader Entry Point
// //////////////////
void main() {
  // We know which light we are from the gl_instanceIndex (Vulkan's gl_InstanceId)
  PointLightData light = scene_data.point_lights[gl_InstanceIndex];
  // Calculate world position of input vertex (no transformations right now)
  vec4 world_position =  vec4(a_position + light.position, 1.0);

  gl_Position = scene_data.camera.proj * scene_data.camera.view * world_position;
  i_light_color = light.color;
  i_light_brightness = light.brightness;
}
