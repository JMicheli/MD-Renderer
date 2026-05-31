#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;
layout(location = 3) in vec3 a_tangent;

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec2 v_uv;
layout(location = 2) out mat3 v_TBN;

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

// Push constant data containing an object's world transforms
layout(push_constant) uniform MdrPushConstants
{
  // The translation/rotation/scale of the current object
	mat4 transformation_matrix;
} object;

// Shader Entry Point
// //////////////////
void main() {
  // Calculate world position of input vertex
  vec4 world_position =  object.transformation_matrix * vec4(a_position, 1.0);

  // Calculate surface normal at input vertex
  // TODO fix to use transpose inverse (or probably just remove nonuniform scaling)
  vec3 normal = normalize(mat3(object.transformation_matrix) * a_normal);
  // Write tangent and bitangent vectors for normal mapping
  vec3 tangent = normalize(mat3(object.transformation_matrix) * a_tangent);
  // Gram-Schmidt re-orthogonalization of the tangent with respect to the normal
  tangent = normalize(tangent - dot(tangent, normal) * normal);
  vec3 bitangent = normalize(cross(tangent, normal));
  v_TBN = mat3(tangent, bitangent, normal);
  
  // Write output UVs
  v_uv = a_uv;
  
  // Write output of vertex position
  v_position = world_position.xyz;
  gl_Position = scene_data.camera.proj * scene_data.camera.view * world_position;
}
