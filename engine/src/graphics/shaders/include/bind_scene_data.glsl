// Data representing the scene
layout(set = 0, binding = 0) buffer MdrSceneData {
  // The camera being used to render the scene
  CameraData camera;
  // Up to MAX_POINT_LIGHTS point light values
  PointLightData point_lights[MAX_POINT_LIGHTS];
  // Maximum point_light index with a valid value
  uint point_light_count;
} scene_data;
